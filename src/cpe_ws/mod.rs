//! # Módulo de Comunicación Web Service - `cpe_ws`
//!
//! Conexión segura con los servicios SOAP / REST de la SUNAT para el envío
//! de comprobantes y consulta de constancias de recepción (CDR).
//!
//! Dispone de soporte dual mediante las funciones con sufijo `_async` y `_sync`.

use crate::cpe_cdr::CpeCdr;
use crate::cpe_empaquetado::{cpe_codificar_base64, cpe_decodificar_base64};
use crate::cpe_error::{CpeError, CpeResult};

/// Entorno de conexión a los servicios de la SUNAT.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpeAmbiente {
    /// Servidores de prueba y homologación Beta de SUNAT.
    Beta,
    /// Servidores oficiales de producción de SUNAT.
    Produccion,
    /// Servidor de Operador de Servicios Electrónicos (OSE) o URL personalizada.
    Personalizado(String),
}

impl CpeAmbiente {
    /// Retorna la URL del servicio SOAP `billService` de SUNAT.
    #[must_use]
    pub fn url_bill_service(&self) -> &str {
        match self {
            Self::Beta => "https://e-beta.sunat.gob.pe/ol-ti-itcpfegem-beta/billService",
            Self::Produccion => "https://e-factura.sunat.gob.pe/ol-ti-itcpfegem/billService",
            Self::Personalizado(url) => url.as_str(),
        }
    }

    /// Retorna la URL del servicio SOAP de consultas `billConsultService`.
    #[must_use]
    pub fn url_consult_service(&self) -> &str {
        match self {
            Self::Beta => "https://e-beta.sunat.gob.pe/ol-ti-itwsconsvalidcpe-beta/billConsultService",
            Self::Produccion => "https://e-factura.sunat.gob.pe/ol-it-wsconsvalidcpe/billConsultService",
            Self::Personalizado(url) => url.as_str(),
        }
    }
}

/// Credenciales de acceso SOL (SUNAT Operaciones en Línea).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpeCredencialesSol {
    /// RUC del emisor (11 dígitos).
    pub ruc: String,
    /// Usuario secundario SOL (generalmente con permisos para emisión de comprobantes).
    pub usuario_sol: String,
    /// Contraseña de la Clave SOL.
    pub clave_sol: String,
}

impl CpeCredencialesSol {
    /// Crea un nuevo conjunto de credenciales SOL.
    #[must_use]
    pub fn nuevo(ruc: &str, usuario_sol: &str, clave_sol: &str) -> Self {
        Self {
            ruc: ruc.trim().to_string(),
            usuario_sol: usuario_sol.trim().to_string(),
            clave_sol: clave_sol.trim().to_string(),
        }
    }

    /// Retorna el Username compuesto para la cabecera WS-Security (`{RUC}{USUARIO}`).
    #[must_use]
    pub fn username_token(&self) -> String {
        format!("{}{}", self.ruc, self.usuario_sol)
    }
}

/// Construye la cabecera WS-Security SOAP para autenticación con Clave SOL.
fn cpe_construir_soap_header_security(credenciales: &CpeCredencialesSol) -> String {
    format!(
        r#"    <soapenv:Header>
        <wsse:Security xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd">
            <wsse:UsernameToken>
                <wsse:Username>{}</wsse:Username>
                <wsse:Password>{}</wsse:Password>
            </wsse:UsernameToken>
        </wsse:Security>
    </soapenv:Header>"#,
        credenciales.username_token(),
        credenciales.clave_sol
    )
}

/// Construye el Envelope SOAP para el método `sendBill`.
#[must_use]
pub fn cpe_construir_envelope_send_bill(
    credenciales: &CpeCredencialesSol,
    nombre_archivo_zip: &str,
    zip_bytes: &[u8],
) -> String {
    let base64_zip = cpe_codificar_base64(zip_bytes);
    let header = cpe_construir_soap_header_security(credenciales);

    format!(
        r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ser="http://service.sunat.gob.pe" xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd">
{}
    <soapenv:Body>
        <ser:sendBill>
            <fileName>{}</fileName>
            <contentFile>{}</contentFile>
        </ser:sendBill>
    </soapenv:Body>
</soapenv:Envelope>"#,
        header, nombre_archivo_zip, base64_zip
    )
}

/// Construye el Envelope SOAP para el método `sendSummary`.
#[must_use]
pub fn cpe_construir_envelope_send_summary(
    credenciales: &CpeCredencialesSol,
    nombre_archivo_zip: &str,
    zip_bytes: &[u8],
) -> String {
    let base64_zip = cpe_codificar_base64(zip_bytes);
    let header = cpe_construir_soap_header_security(credenciales);

    format!(
        r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ser="http://service.sunat.gob.pe" xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd">
{}
    <soapenv:Body>
        <ser:sendSummary>
            <fileName>{}</fileName>
            <contentFile>{}</contentFile>
        </ser:sendSummary>
    </soapenv:Body>
</soapenv:Envelope>"#,
        header, nombre_archivo_zip, base64_zip
    )
}

/// Construye el Envelope SOAP para el método `getStatus` (consulta de ticket).
#[must_use]
pub fn cpe_construir_envelope_get_status(
    credenciales: &CpeCredencialesSol,
    ticket: &str,
) -> String {
    let header = cpe_construir_soap_header_security(credenciales);

    format!(
        r#"<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:ser="http://service.sunat.gob.pe" xmlns:wsse="http://docs.oasis-open.org/wss/2004/01/oasis-200401-wss-wssecurity-secext-1.0.xsd">
{}
    <soapenv:Body>
        <ser:getStatus>
            <ticket>{}</ticket>
        </ser:getStatus>
    </soapenv:Body>
</soapenv:Envelope>"#,
        header, ticket
    )
}

/// Extrae el CDR en Base64 de la respuesta SOAP de `sendBill`.
pub fn cpe_extraer_cdr_de_respuesta_soap(soap_response: &str) -> CpeResult<CpeCdr> {
    if let Some(pos) = soap_response.find("<applicationResponse>") {
        let inicio = pos + "<applicationResponse>".len();
        let fin = soap_response[inicio..]
            .find("</applicationResponse>")
            .ok_or_else(|| {
                CpeError::ErrorXml("Etiqueta </applicationResponse> no encontrada".to_string())
            })?
            + inicio;

        let base64_cdr = &soap_response[inicio..fin];
        let zip_bytes = cpe_decodificar_base64(base64_cdr)?;
        CpeCdr::desde_zip(&zip_bytes)
    } else if let Some(fault_pos) = soap_response.find("<faultcode>") {
        let inicio = fault_pos + "<faultcode>".len();
        let fin = soap_response[inicio..].find("</faultcode>").unwrap_or(0) + inicio;
        let codigo = &soap_response[inicio..fin];

        let msg_inicio = soap_response
            .find("<faultstring>")
            .map(|p| p + "<faultstring>".len())
            .unwrap_or(0);
        let msg_fin = soap_response[msg_inicio..]
            .find("</faultstring>")
            .unwrap_or(0)
            + msg_inicio;
        let mensaje = &soap_response[msg_inicio..msg_fin];

        Err(CpeError::ErrorSunatWebService {
            codigo: codigo.trim().to_string(),
            mensaje: mensaje.trim().to_string(),
        })
    } else {
        Err(CpeError::ErrorXml(
            "Respuesta SOAP inesperada: no contiene applicationResponse ni SoapFault".to_string(),
        ))
    }
}

// ----------------------------------------------------------------------------
// Implementación Asíncrona (con feature "async")
// ----------------------------------------------------------------------------
#[cfg(feature = "async")]
/// Envía un comprobante de pago de forma asíncrona a la SUNAT (`sendBill`).
///
/// # Errors
/// Retorna `CpeError::ErrorRed` o `CpeError::ErrorSunatWebService` si la llamada falla.
pub async fn cpe_enviar_documento_async(
    ambiente: &CpeAmbiente,
    credenciales: &CpeCredencialesSol,
    nombre_archivo_zip: &str,
    zip_bytes: &[u8],
) -> CpeResult<CpeCdr> {
    let soap_envelope =
        cpe_construir_envelope_send_bill(credenciales, nombre_archivo_zip, zip_bytes);
    let client = reqwest::Client::new();

    let res = client
        .post(ambiente.url_bill_service())
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "urn:sendBill")
        .body(soap_envelope)
        .send()
        .await
        .map_err(|e| CpeError::ErrorRed(format!("Error HTTP al enviar comprobante a SUNAT: {e}")))?;

    let cuerpo_respuesta = res
        .text()
        .await
        .map_err(|e| CpeError::ErrorRed(format!("Error al leer respuesta SOAP: {e}")))?;

    cpe_extraer_cdr_de_respuesta_soap(&cuerpo_respuesta)
}

#[cfg(feature = "async")]
/// Envía un resumen diario o comunicación de baja de forma asíncrona a la SUNAT (`sendSummary`).
///
/// Retorna el número de ticket asignado por SUNAT para posterior consulta.
pub async fn cpe_enviar_resumen_async(
    ambiente: &CpeAmbiente,
    credenciales: &CpeCredencialesSol,
    nombre_archivo_zip: &str,
    zip_bytes: &[u8],
) -> CpeResult<String> {
    let soap_envelope =
        cpe_construir_envelope_send_summary(credenciales, nombre_archivo_zip, zip_bytes);
    let client = reqwest::Client::new();

    let res = client
        .post(ambiente.url_bill_service())
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("SOAPAction", "urn:sendSummary")
        .body(soap_envelope)
        .send()
        .await
        .map_err(|e| CpeError::ErrorRed(format!("Error HTTP al enviar resumen a SUNAT: {e}")))?;

    let cuerpo_respuesta = res
        .text()
        .await
        .map_err(|e| CpeError::ErrorRed(format!("Error al leer respuesta SOAP: {e}")))?;

    if let Some(pos) = cuerpo_respuesta.find("<ticket>") {
        let inicio = pos + "<ticket>".len();
        let fin = cuerpo_respuesta[inicio..]
            .find("</ticket>")
            .ok_or_else(|| CpeError::ErrorXml("Etiqueta </ticket> no encontrada".to_string()))?
            + inicio;
        Ok(cuerpo_respuesta[inicio..fin].trim().to_string())
    } else {
        cpe_extraer_cdr_de_respuesta_soap(&cuerpo_respuesta)?;
        Err(CpeError::ErrorXml(
            "Respuesta no contiene número de ticket".to_string(),
        ))
    }
}

// ----------------------------------------------------------------------------
// Implementación Síncrona (con feature "blocking")
// ----------------------------------------------------------------------------
#[cfg(feature = "blocking")]
/// Envía un comprobante de pago de forma síncrona a la SUNAT (`sendBill`).
///
/// # Errors
/// Retorna `CpeError::ErrorRed` o `CpeError::ErrorSunatWebService` si la llamada falla.
pub fn cpe_enviar_documento_sync(
    ambiente: &CpeAmbiente,
    credenciales: &CpeCredencialesSol,
    nombre_archivo_zip: &str,
    zip_bytes: &[u8],
) -> CpeResult<CpeCdr> {
    let soap_envelope =
        cpe_construir_envelope_send_bill(credenciales, nombre_archivo_zip, zip_bytes);

    let res = ureq::post(ambiente.url_bill_service())
        .set("Content-Type", "text/xml; charset=utf-8")
        .set("SOAPAction", "urn:sendBill")
        .send_string(&soap_envelope)
        .map_err(|e| CpeError::ErrorRed(format!("Error HTTP síncrono al contactar SUNAT: {e}")))?;

    let cuerpo_respuesta = res
        .into_string()
        .map_err(|e| CpeError::ErrorRed(format!("Error al leer respuesta SOAP: {e}")))?;

    cpe_extraer_cdr_de_respuesta_soap(&cuerpo_respuesta)
}

#[cfg(feature = "blocking")]
/// Envía un resumen diario o baja de forma síncrona a la SUNAT (`sendSummary`).
pub fn cpe_enviar_resumen_sync(
    ambiente: &CpeAmbiente,
    credenciales: &CpeCredencialesSol,
    nombre_archivo_zip: &str,
    zip_bytes: &[u8],
) -> CpeResult<String> {
    let soap_envelope =
        cpe_construir_envelope_send_summary(credenciales, nombre_archivo_zip, zip_bytes);

    let res = ureq::post(ambiente.url_bill_service())
        .set("Content-Type", "text/xml; charset=utf-8")
        .set("SOAPAction", "urn:sendSummary")
        .send_string(&soap_envelope)
        .map_err(|e| CpeError::ErrorRed(format!("Error HTTP síncrono al enviar resumen: {e}")))?;

    let cuerpo_respuesta = res
        .into_string()
        .map_err(|e| CpeError::ErrorRed(format!("Error al leer respuesta SOAP: {e}")))?;

    if let Some(pos) = cuerpo_respuesta.find("<ticket>") {
        let inicio = pos + "<ticket>".len();
        let fin = cuerpo_respuesta[inicio..]
            .find("</ticket>")
            .ok_or_else(|| CpeError::ErrorXml("Etiqueta </ticket> no encontrada".to_string()))?
            + inicio;
        Ok(cuerpo_respuesta[inicio..fin].trim().to_string())
    } else {
        cpe_extraer_cdr_de_respuesta_soap(&cuerpo_respuesta)?;
        Err(CpeError::ErrorXml(
            "Respuesta no contiene número de ticket".to_string(),
        ))
    }
}
