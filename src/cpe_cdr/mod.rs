//! # Módulo de Constancia de Recepción (CDR) - `cpe_cdr`
//!
//! Procesamiento y validación del comprobante de recepción emitido por la SUNAT u OSE.

use crate::cpe_empaquetado::cpe_descomprimir_primer_archivo;
use crate::cpe_error::{CpeError, CpeResult};
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};

/// Estado del comprobante según el código de respuesta del CDR de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CpeEstadoCdr {
    /// Código "0": Comprobante válido y plenamente aceptado por la SUNAT.
    Aceptado,
    /// Códigos "0100" a "1999" o aceptación con notas de advertencia: Aceptado tributariamente, pero contiene observaciones.
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
    /// Parsea un XML de CDR y construye la estructura `CpeCdr` utilizando un parser
    /// de eventos XML robusto e insensible a variaciones de namespaces.
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorXml` si la sintaxis del documento XML no es válida.
    pub fn desde_xml(xml: &str) -> CpeResult<Self> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut id = String::new();
        let mut fecha_respuesta = String::new();
        let mut hora_respuesta = None;
        let mut ruc_emisor = String::new();
        let mut documento_referenciado = String::new();
        let mut codigo_respuesta = String::new();
        let mut descripcion_respuesta = String::new();
        let mut hash_comprobante = None;
        let mut observaciones = Vec::new();

        let mut current_tag = String::new();
        let mut inside_response = false;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let local = match name.as_ref().split(|&b| b == b':').next_back() {
                        Some(l) => String::from_utf8_lossy(l).to_string(),
                        None => String::new(),
                    };
                    if local == "Response" {
                        inside_response = true;
                    }
                    current_tag = local;
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let local = match name.as_ref().split(|&b| b == b':').next_back() {
                        Some(l) => String::from_utf8_lossy(l).to_string(),
                        None => String::new(),
                    };
                    if local == "Response" {
                        inside_response = false;
                    }
                    current_tag.clear();
                }
                Ok(Event::Text(ref e)) => {
                    let text = e.unescape().unwrap_or_default().trim().to_string();
                    if !text.is_empty() {
                        match current_tag.as_str() {
                            "ID" if id.is_empty() => id = text,
                            "ResponseDate" => fecha_respuesta = text,
                            "ResponseTime" => hora_respuesta = Some(text),
                            "RecipientPartyID" => ruc_emisor = text,
                            "ReferenceID" => documento_referenciado = text,
                            "ResponseCode" if inside_response || codigo_respuesta.is_empty() => {
                                codigo_respuesta = text;
                            }
                            "Description"
                                if (inside_response || descripcion_respuesta.is_empty()) =>
                            {
                                descripcion_respuesta = text;
                            }
                            "DigestValue" => hash_comprobante = Some(text),
                            "Note" => observaciones.push(text),
                            _ => {}
                        }
                    }
                }
                Ok(Event::CData(ref e)) => {
                    let text = String::from_utf8_lossy(e.as_ref()).trim().to_string();
                    if !text.is_empty() {
                        match current_tag.as_str() {
                            "Description"
                                if (inside_response || descripcion_respuesta.is_empty()) =>
                            {
                                descripcion_respuesta = text;
                            }
                            "Note" => observaciones.push(text),
                            _ => {}
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(CpeError::ErrorXml(format!(
                        "Error al analizar sintaxis XML del CDR: {e}"
                    )));
                }
                _ => {}
            }
        }

        if codigo_respuesta.is_empty() {
            codigo_respuesta = "DESCONOCIDO".to_string();
        }
        if descripcion_respuesta.is_empty() {
            descripcion_respuesta = "Sin descripción".to_string();
        }

        let estado = if codigo_respuesta == "0" {
            if observaciones.is_empty() {
                CpeEstadoCdr::Aceptado
            } else {
                CpeEstadoCdr::AceptadoConObservaciones(observaciones.clone())
            }
        } else if let Ok(cod_num) = codigo_respuesta.parse::<u32>() {
            if (100..2000).contains(&cod_num) {
                let mut obs = observaciones.clone();
                if obs.is_empty() {
                    obs.push(descripcion_respuesta.clone());
                }
                CpeEstadoCdr::AceptadoConObservaciones(obs)
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
            observaciones,
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
