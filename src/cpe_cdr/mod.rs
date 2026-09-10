//! # Módulo de Constancia de Recepción (CDR) - `cpe_cdr`
//!
//! Procesamiento y validación del comprobante de recepción emitido por la SUNAT u OSE.

use crate::cpe_empaquetado::cpe_descomprimir_primer_archivo;
use crate::cpe_error::CpeResult;
use serde::{Deserialize, Serialize};

/// Estado del comprobante según el código de respuesta del CDR de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CpeEstadoCdr {
    /// Código "0": Comprobante válido y plenamente aceptado por la SUNAT.
    Aceptado,
    /// Códigos "0100" a "1999": Aceptado tributariamente, pero contiene observaciones.
    AceptadoConObservaciones(Vec<String>),
    /// Códigos "2000" o superiores: Comprobante rechazado por SUNAT (sin valor tributario).
    Rechazado(String),
}

/// Representa los datos extraídos de la Constancia de Recepción (CDR) de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeCdr {
    /// Identificador del CDR (ej: "R-20123456789-01-F001-00000001").
    pub id: String,
    /// Fecha de respuesta de SUNAT (YYYY-MM-DD).
    pub fecha_respuesta: String,
    /// Hora de respuesta de SUNAT (HH:MM:SS).
    pub hora_respuesta: Option<String>,
    /// RUC del emisor recepcionado.
    pub ruc_emisor: String,
    /// Documento referenciado en el CDR.
    pub documento_referenciado: String,
    /// Código numérico de respuesta de SUNAT (ej: "0", "0127", "2015").
    pub codigo_respuesta: String,
    /// Descripción textual provista por el validador de SUNAT.
    pub descripcion_respuesta: String,
    /// Estado clasificado del comprobante.
    pub estado: CpeEstadoCdr,
    /// Observaciones reportadas por SUNAT.
    pub observaciones: Vec<String>,
    /// Digest o hash del comprobante recepcionado.
    pub hash_comprobante: Option<String>,
}

impl CpeCdr {
    /// Parsea un XML de CDR y construye la estructura `CpeCdr`.
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorCdr` si no se encuentran los nodos obligatorios de respuesta.
    pub fn desde_xml(xml: &str) -> CpeResult<Self> {
        let codigo_respuesta = extraer_etiqueta(xml, "cbc:ResponseCode")
            .unwrap_or_else(|| "DESCONOCIDO".to_string());
        let descripcion_respuesta = extraer_etiqueta(xml, "cbc:Description")
            .unwrap_or_else(|| "Sin descripción".to_string());
        let id = extraer_etiqueta(xml, "cbc:ID").unwrap_or_default();
        let fecha_respuesta = extraer_etiqueta(xml, "cbc:ResponseDate").unwrap_or_default();
        let hora_respuesta = extraer_etiqueta(xml, "cbc:ResponseTime");
        let ruc_emisor = extraer_etiqueta(xml, "cbc:RecipientPartyID").unwrap_or_default();
        let documento_referenciado = extraer_etiqueta(xml, "cbc:ReferenceID").unwrap_or_default();
        let hash_comprobante = extraer_etiqueta(xml, "ds:DigestValue");

        // Clasificar el estado según el estándar de códigos de SUNAT
        let estado = if codigo_respuesta == "0" {
            CpeEstadoCdr::Aceptado
        } else if let Ok(cod_num) = codigo_respuesta.parse::<u32>() {
            if (100..2000).contains(&cod_num) {
                CpeEstadoCdr::AceptadoConObservaciones(vec![descripcion_respuesta.clone()])
            } else {
                CpeEstadoCdr::Rechazado(format!("[{codigo_respuesta}] {descripcion_respuesta}"))
            }
        } else {
            CpeEstadoCdr::Rechazado(format!("[{codigo_respuesta}] {descripcion_respuesta}"))
        };

        Ok(Self {
            id,
            fecha_respuesta,
            hora_respuesta,
            ruc_emisor,
            documento_referenciado,
            codigo_respuesta,
            descripcion_respuesta,
            estado,
            observaciones: Vec::new(),
            hash_comprobante,
        })
    }

    /// Descomprime un archivo ZIP que contiene la constancia de recepción (`R-*.zip`) y lo parsea.
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorEmpaquetado` o `CpeError::ErrorCdr` si la extracción o análisis fallan.
    pub fn desde_zip(zip_bytes: &[u8]) -> CpeResult<Self> {
        let (_nombre, contenido_xml) = cpe_descomprimir_primer_archivo(zip_bytes)?;
        let xml_str = String::from_utf8_lossy(&contenido_xml);
        Self::desde_xml(&xml_str)
    }

    /// Retorna `true` si el comprobante fue aceptado (con o sin observaciones).
    #[must_use]
    pub fn es_aceptado(&self) -> bool {
        matches!(
            self.estado,
            CpeEstadoCdr::Aceptado | CpeEstadoCdr::AceptadoConObservaciones(_)
        )
    }
}

/// Función auxiliar para extraer el contenido textual de una etiqueta XML simple.
fn extraer_etiqueta(xml: &str, tag: &str) -> Option<String> {
    let tag_apertura = format!("<{tag}>");
    let tag_cierre = format!("</{tag}>");

    let inicio = xml.find(&tag_apertura)? + tag_apertura.len();
    let fin = xml[inicio..].find(&tag_cierre)? + inicio;

    let contenido = xml[inicio..fin].trim();
    if contenido.starts_with("<![CDATA[") && contenido.ends_with("]]>") {
        Some(contenido[9..contenido.len() - 3].trim().to_string())
    } else {
        Some(contenido.to_string())
    }
}
