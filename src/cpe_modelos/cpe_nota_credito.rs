//! # Modelo de Nota de Crédito Electrónica - `cpe_nota_credito`
//!
//! Representa un comprobante de pago electrónico tipo 07 (Nota de Crédito Electrónica).

use super::cpe_comun::{
    CpeDocumentoReferencia, CpeEmisor, CpeItem, CpeLeyenda, CpeReceptor, CpeTotales,
};
use crate::cpe_catalogos::{CpeTipoDocumento, CpeTipoMoneda, CpeTipoNotaCredito};
use serde::{Deserialize, Serialize};

/// Estructura de Nota de Crédito Electrónica (CPE Tipo 07).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeNotaCredito {
    /// Serie del comprobante (4 caracteres, ej. "FC01" o "BC01").
    pub serie: String,
    /// Número correlativo (1 a 8 dígitos numéricos).
    pub correlativo: u32,
    /// Fecha de emisión (YYYY-MM-DD).
    pub fecha_emision: String,
    /// Hora de emisión (HH:MM:SS).
    pub hora_emision: Option<String>,
    /// Moneda del documento.
    pub moneda: CpeTipoMoneda,
    /// Motivo / Tipo de Nota de Crédito (Catálogo 09).
    pub tipo_nota_credito: CpeTipoNotaCredito,
    /// Descripción detallada del motivo de emisión.
    pub sustento_motivo: String,
    /// Documento que se modifica (Factura o Boleta afectada).
    pub documento_modificado: CpeDocumentoReferencia,
    /// Datos del emisor.
    pub emisor: CpeEmisor,
    /// Datos del adquirente / cliente.
    pub receptor: CpeReceptor,
    /// Ítems afectados.
    pub items: Vec<CpeItem>,
    /// Totales liquidados.
    pub totales: CpeTotales,
    /// Leyendas tributarias.
    pub leyendas: Vec<CpeLeyenda>,
}

impl CpeNotaCredito {
    /// Retorna el tipo de documento oficial (07).
    #[must_use]
    pub const fn tipo_documento(&self) -> CpeTipoDocumento {
        CpeTipoDocumento::NotaCredito
    }

    /// Genera el nombre base oficial: `{RUC}-07-{SERIE}-{CORRELATIVO}`.
    #[must_use]
    pub fn nombre_base(&self) -> String {
        format!(
            "{}-07-{}-{:08}",
            self.emisor.ruc.trim(),
            self.serie.trim(),
            self.correlativo
        )
    }

    /// Retorna el nombre de archivo `.xml`.
    #[must_use]
    pub fn nombre_archivo_xml(&self) -> String {
        format!("{}.xml", self.nombre_base())
    }

    /// Retorna el nombre de archivo `.zip`.
    #[must_use]
    pub fn nombre_archivo_zip(&self) -> String {
        format!("{}.zip", self.nombre_base())
    }
}
