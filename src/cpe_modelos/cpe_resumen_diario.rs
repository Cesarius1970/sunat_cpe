//! # Modelos de Resumen Diario y Bajas - `cpe_resumen_diario`
//!
//! Soporte para resúmenes diarios de boletas (RC) y comunicaciones de baja (RA).

use super::cpe_comun::CpeEmisor;
use crate::cpe_catalogos::{CpeTipoDocumento, CpeTipoMoneda};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Detalle individual de un ítem en el Resumen Diario de Boletas.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeItemResumenDiario {
    /// Número de orden en el resumen (inicia en 1).
    pub numero_orden: u32,
    /// Tipo de documento (ej. 03 Boleta, 07 NC, 08 ND).
    pub tipo_documento: CpeTipoDocumento,
    /// Serie y número del comprobante resumido (ej. "B001-1234").
    pub serie_numero: String,
    /// Tipo de documento del cliente (Catálogo 06).
    pub tipo_documento_cliente: String,
    /// Número de documento del cliente.
    pub numero_documento_cliente: String,
    /// Estado de la boleta (1: Agregar, 2: Modificar, 3: Anular).
    pub codigo_estado: u8,
    /// Total de la venta.
    pub total_venta: Decimal,
    /// Total de operaciones gravadas.
    pub total_gravado: Decimal,
    /// Total de operaciones exoneradas.
    pub total_exonerado: Decimal,
    /// Total de operaciones inafectas.
    pub total_inafecto: Decimal,
    /// Total de IGV.
    pub total_igv: Decimal,
}

/// Resumen Diario de Boletas de Venta y Notas Electrónicas (Tipo RC).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeResumenDiario {
    /// Identificador correlativo del resumen en el día (1 a 5 dígitos).
    pub correlativo: u32,
    /// Fecha de emisión de los comprobantes resumidos (YYYY-MM-DD).
    pub fecha_referencia: String,
    /// Fecha de generación y envío del resumen (YYYY-MM-DD).
    pub fecha_generacion: String,
    /// Moneda de las operaciones.
    pub moneda: CpeTipoMoneda,
    /// Datos del emisor.
    pub emisor: CpeEmisor,
    /// Listado de boletas o notas incluidas en el resumen.
    pub items: Vec<CpeItemResumenDiario>,
}

impl CpeResumenDiario {
    /// Genera el identificador oficial: `RC-{YYYYMMDD}-{CORRELATIVO}`.
    #[must_use]
    pub fn identificador(&self) -> String {
        let fecha_formateada = self.fecha_generacion.replace('-', "");
        format!("RC-{}-{:05}", fecha_formateada, self.correlativo)
    }

    /// Nombre base exigido por SUNAT: `{RUC}-RC-{YYYYMMDD}-{CORRELATIVO}`.
    #[must_use]
    pub fn nombre_base(&self) -> String {
        format!("{}-{}", self.emisor.ruc.trim(), self.identificador())
    }

    /// Nombre del archivo `.xml`.
    #[must_use]
    pub fn nombre_archivo_xml(&self) -> String {
        format!("{}.xml", self.nombre_base())
    }

    /// Nombre del archivo `.zip`.
    #[must_use]
    pub fn nombre_archivo_zip(&self) -> String {
        format!("{}.zip", self.nombre_base())
    }
}

/// Detalle individual de un comprobante dado de baja.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeItemComunicacionBaja {
    /// Número de orden en la comunicación de baja (inicia en 1).
    pub numero_orden: u32,
    /// Tipo de documento anulado (01 Factura, 07 NC, 08 ND).
    pub tipo_documento: CpeTipoDocumento,
    /// Serie del comprobante (ej: "F001").
    pub serie: String,
    /// Número del comprobante anulado.
    pub correlativo: u32,
    /// Motivo detallado de la anulación o baja.
    pub motivo_baja: String,
}

/// Comunicación de Baja de Facturas y Notas vinculadas (Tipo RA).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeComunicacionBaja {
    /// Identificador correlativo de la baja en el día (1 a 5 dígitos).
    pub correlativo: u32,
    /// Fecha de emisión de los comprobantes que se dan de baja (YYYY-MM-DD).
    pub fecha_referencia: String,
    /// Fecha de generación de la comunicación de baja (YYYY-MM-DD).
    pub fecha_generacion: String,
    /// Datos del emisor.
    pub emisor: CpeEmisor,
    /// Listado de comprobantes a anular.
    pub items: Vec<CpeItemComunicacionBaja>,
}

impl CpeComunicacionBaja {
    /// Genera el identificador oficial: `RA-{YYYYMMDD}-{CORRELATIVO}`.
    #[must_use]
    pub fn identificador(&self) -> String {
        let fecha_formateada = self.fecha_generacion.replace('-', "");
        format!("RA-{}-{:05}", fecha_formateada, self.correlativo)
    }

    /// Nombre base exigido por SUNAT: `{RUC}-RA-{YYYYMMDD}-{CORRELATIVO}`.
    #[must_use]
    pub fn nombre_base(&self) -> String {
        format!("{}-{}", self.emisor.ruc.trim(), self.identificador())
    }

    /// Nombre del archivo `.xml`.
    #[must_use]
    pub fn nombre_archivo_xml(&self) -> String {
        format!("{}.xml", self.nombre_base())
    }

    /// Nombre del archivo `.zip`.
    #[must_use]
    pub fn nombre_archivo_zip(&self) -> String {
        format!("{}.zip", self.nombre_base())
    }
}
