//! # Modelo de Boleta de Venta Electrónica - `cpe_boleta`
//!
//! Representa un comprobante de pago electrónico tipo 03 (Boleta de Venta Electrónica).

use super::cpe_comun::{
    CpeDocumentoReferencia, CpeEmisor, CpeFormaPago, CpeInformacionPago, CpeItem, CpeLeyenda,
    CpeReceptor, CpeTotales,
};
use crate::cpe_catalogos::{CpeTipoDocumento, CpeTipoMoneda, CpeTipoOperacion};
use serde::{Deserialize, Serialize};

/// Estructura de Boleta de Venta Electrónica (CPE Tipo 03).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeBoleta {
    /// Serie del comprobante (4 caracteres, ej. "B001").
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
    /// Datos del cliente / receptor.
    pub receptor: CpeReceptor,
    /// Modalidad de pago (Contado o Crédito).
    pub forma_pago: CpeFormaPago,
    /// Información bancaria opcional.
    pub informacion_pago: Option<CpeInformacionPago>,
    /// Ítems o detalles vendidos.
    pub items: Vec<CpeItem>,
    /// Totales liquidados.
    pub totales: CpeTotales,
    /// Leyendas tributarias.
    pub leyendas: Vec<CpeLeyenda>,
    /// Comprobantes relacionados o guías.
    pub documentos_referencia: Vec<CpeDocumentoReferencia>,
}

impl CpeBoleta {
    /// Retorna el tipo de documento oficial SUNAT (03).
    #[must_use]
    pub const fn tipo_documento(&self) -> CpeTipoDocumento {
        CpeTipoDocumento::BoletaVenta
    }

    /// Genera el nombre base oficial: `{RUC}-03-{SERIE}-{CORRELATIVO}`.
    #[must_use]
    pub fn nombre_base(&self) -> String {
        format!(
            "{}-03-{}-{:08}",
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
