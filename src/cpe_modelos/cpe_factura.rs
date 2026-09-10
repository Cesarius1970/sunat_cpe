//! # Modelo de Factura Electrónica - `cpe_factura`
//!
//! Representa un comprobante de pago electrónico tipo 01 (Factura Electrónica).

use super::cpe_comun::{
    CpeDocumentoReferencia, CpeEmisor, CpeFormaPago, CpeInformacionPago, CpeItem, CpeLeyenda,
    CpeReceptor, CpeTotales,
};
use crate::cpe_catalogos::{CpeTipoDocumento, CpeTipoMoneda, CpeTipoOperacion};
use serde::{Deserialize, Serialize};

/// Estructura de Factura Electrónica (CPE Tipo 01).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeFactura {
    /// Serie del comprobante (4 caracteres, ej. "F001").
    pub serie: String,
    /// Número correlativo del comprobante (1 a 8 dígitos numéricos).
    pub correlativo: u32,
    /// Fecha de emisión en formato ISO (YYYY-MM-DD).
    pub fecha_emision: String,
    /// Hora de emisión en formato ISO (HH:MM:SS).
    pub hora_emision: Option<String>,
    /// Fecha de vencimiento del pago (YYYY-MM-DD).
    pub fecha_vencimiento: Option<String>,
    /// Tipo de operación comercial (Catálogo 51).
    pub tipo_operacion: CpeTipoOperacion,
    /// Moneda de la transacción (Catálogo 02).
    pub moneda: CpeTipoMoneda,
    /// Datos del emisor electrónico.
    pub emisor: CpeEmisor,
    /// Datos del adquirente / receptor.
    pub receptor: CpeReceptor,
    /// Modalidad de pago (Contado o Crédito con cuotas).
    pub forma_pago: CpeFormaPago,
    /// Información bancaria o medio de pago (opcional).
    pub informacion_pago: Option<CpeInformacionPago>,
    /// Líneas o ítems de la factura.
    pub items: Vec<CpeItem>,
    /// Resumen de totales e impuestos calculados.
    pub totales: CpeTotales,
    /// Leyendas tributarias requeridas (Catálogo 52).
    pub leyendas: Vec<CpeLeyenda>,
    /// Guías de remisión u otros comprobantes vinculados.
    pub documentos_referencia: Vec<CpeDocumentoReferencia>,
}

impl CpeFactura {
    /// Retorna el tipo de documento oficial SUNAT (01).
    #[must_use]
    pub const fn tipo_documento(&self) -> CpeTipoDocumento {
        CpeTipoDocumento::Factura
    }

    /// Genera el nombre base oficial del archivo exigido por SUNAT: `{RUC}-01-{SERIE}-{CORRELATIVO}`.
    #[must_use]
    pub fn nombre_base(&self) -> String {
        format!(
            "{}-01-{}-{:08}",
            self.emisor.ruc.trim(),
            self.serie.trim(),
            self.correlativo
        )
    }

    /// Retorna el nombre con extensión `.xml`.
    #[must_use]
    pub fn nombre_archivo_xml(&self) -> String {
        format!("{}.xml", self.nombre_base())
    }

    /// Retorna el nombre con extensión `.zip`.
    #[must_use]
    pub fn nombre_archivo_zip(&self) -> String {
        format!("{}.zip", self.nombre_base())
    }
}
