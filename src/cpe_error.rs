//! # Módulo de Manejo de Errores - `cpe_error`
//!
//! Define los errores fuertemente tipados utilizados en todas las capas de `sunat_cpe`.

use thiserror::Error;

/// Representa todos los errores posibles que pueden ocurrir durante el ciclo de vida de un CPE.
#[derive(Debug, Error)]
pub enum CpeError {
    /// Error de validación de reglas de negocio de SUNAT o campos obligatorios faltantes.
    #[error("Error de validación CPE: {0}")]
    ErrorValidacion(String),

    /// Error al serializar o deserializar documentos XML conforme a los esquemas UBL.
    #[error("Error en procesamiento XML UBL: {0}")]
    ErrorXml(String),

    /// Error durante la canonicalización C14N o cálculo de firma digital XMLDSig.
    #[error("Error en firma digital XMLDSig: {0}")]
    ErrorFirmaDigital(String),

    /// Error al procesar certificados digitales X.509 o claves privadas PKCS#12 (.pfx/.p12).
    #[error("Error en certificado digital: {0}")]
    ErrorCertificado(String),

    /// Error durante la compresión o descompresión ZIP de comprobantes o CDR.
    #[error("Error en empaquetado ZIP: {0}")]
    ErrorEmpaquetado(String),

    /// Error de comunicación de red o protocolo HTTP/SOAP/REST con los servidores de SUNAT.
    #[error("Error de red/transporte con SUNAT: {0}")]
    ErrorRed(String),

    /// Error retornado en una respuesta SOAP Fault o fallo del Web Service de SUNAT.
    #[error("Fallo retornado por Web Service SUNAT [{codigo}]: {mensaje}")]
    ErrorSunatWebService {
        /// Código de error retornado por SUNAT (ejemplo: "0100", "2014").
        codigo: String,
        /// Mensaje descriptivo de SUNAT.
        mensaje: String,
    },

    /// Error al interpretar o verificar la Constancia de Recepción (CDR) de SUNAT.
    #[error("Error al procesar CDR de SUNAT: {0}")]
    ErrorCdr(String),

    /// Error de E/S local (lectura/escritura de archivos).
    #[error("Error de E/S de archivo: {0}")]
    ErrorIo(#[from] std::io::Error),
}

/// Tipo `Result` especializado para las operaciones de `sunat_cpe`.
pub type CpeResult<T> = Result<T, CpeError>;
