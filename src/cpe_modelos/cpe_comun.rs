//! # Entidades Comunes de Dominio CPE - `cpe_comun`
//!
//! Modelos compartidos entre Facturas, Boletas, Notas y Guías de Remisión.
//! Todos los campos monetarios utilizan estrictamente `rust_decimal::Decimal`.

use crate::cpe_catalogos::{
    CpeCodigoLeyenda, CpeMedioPago, CpeTipoAfectacionIgv, CpeTipoDocumento,
    CpeTipoDocumentoIdentidad,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Dirección fiscal o de entrega para el emisor o receptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeDireccion {
    /// Código de ubigeo (6 dígitos, ej: "150101").
    pub ubigeo: Option<String>,
    /// Departamento (ej: "LIMA").
    pub departamento: Option<String>,
    /// Provincia (ej: "LIMA").
    pub provincia: Option<String>,
    /// Distrito (ej: "LIMA").
    pub distrito: Option<String>,
    /// Dirección detallada (calle, avenida, número, mz, lote).
    pub direccion_detallada: String,
    /// Código de país ISO 3166-1 alfa-2 (por defecto "PE").
    pub codigo_pais: String,
}

impl Default for CpeDireccion {
    fn default() -> Self {
        Self {
            ubigeo: None,
            departamento: None,
            provincia: None,
            distrito: None,
            direccion_detallada: String::new(),
            codigo_pais: "PE".to_string(),
        }
    }
}

/// Datos del emisor electrónico del comprobante.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeEmisor {
    /// Número de RUC del emisor (11 dígitos).
    pub ruc: String,
    /// Razón social registrada en SUNAT.
    pub razon_social: String,
    /// Nombre comercial del establecimiento.
    pub nombre_comercial: Option<String>,
    /// Dirección fiscal principal o del punto de emisión.
    pub direccion: Option<CpeDireccion>,
}

/// Datos del cliente / receptor del comprobante.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeReceptor {
    /// Tipo de documento de identidad según Catálogo 06 (DNI, RUC, etc.).
    pub tipo_documento: CpeTipoDocumentoIdentidad,
    /// Número de documento de identidad.
    pub numero_documento: String,
    /// Razón social o nombres y apellidos.
    pub razon_social: String,
    /// Dirección opcional del receptor.
    pub direccion: Option<CpeDireccion>,
    /// Correo electrónico opcional para envío comercial.
    pub correo_electronico: Option<String>,
}

/// Detalle de un ítem o línea del comprobante de pago.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeItem {
    /// Número correlativo de la línea dentro del comprobante (iniciando en 1).
    pub numero_orden: u32,
    /// Código interno o SKU del producto/servicio.
    pub codigo_producto: Option<String>,
    /// Código de producto de SUNAT (Catálogo 25 - UNSPSC de 8 dígitos).
    pub codigo_producto_sunat: Option<String>,
    /// Unidad de medida según Catálogo 03 (ej: "NIU" para unidad, "KGM" para kilogramos).
    pub unidad_medida: String,
    /// Cantidad del bien o servicio vendido (hasta 10 decimales exactos).
    pub cantidad: Decimal,
    /// Descripción clara del bien o servicio.
    pub descripcion: String,
    /// Valor unitario sin impuestos (hasta 10 decimales).
    pub valor_unitario: Decimal,
    /// Precio de venta unitario con impuestos incluidos (hasta 10 decimales).
    pub precio_unitario: Decimal,
    /// Tipo de afectación al IGV (Catálogo 07).
    pub tipo_afectacion_igv: CpeTipoAfectacionIgv,
    /// Porcentaje del IGV aplicable (ej: 18.00 o 10.00 para regímenes especiales).
    pub porcentaje_igv: Decimal,
    /// Monto liquidado de IGV para esta línea (exactamente 2 decimales).
    pub monto_igv: Decimal,
    /// Base imponible de la línea = cantidad * valor_unitario (exactamente 2 decimales).
    pub base_imponible_igv: Decimal,
    /// Valor total de la venta de la línea (exactamente 2 decimales).
    pub total_valor_venta: Decimal,
    /// Monto de descuento por línea (si aplica).
    pub monto_descuento: Option<Decimal>,
}

/// Cuota individual en modalidad de pago al crédito.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeCuotaPago {
    /// Identificador o número de cuota (ej: "Cuota001").
    pub numero_cuota: String,
    /// Monto de la cuota a pagar (2 decimales).
    pub monto: Decimal,
    /// Fecha de vencimiento (formato YYYY-MM-DD).
    pub fecha_vencimiento: String,
}

/// Forma de pago del comprobante (Contado o Crédito con cuotas).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CpeFormaPago {
    /// Pago al Contado
    Contado,
    /// Pago al Crédito
    Credito {
        /// Monto neto pendiente de pago (Monto total - Retenciones o anticipos).
        monto_neto_pendiente: Decimal,
        /// Listado de cuotas programadas.
        cuotas: Vec<CpeCuotaPago>,
    },
}

/// Información de pago y bancarización opcional.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeInformacionPago {
    /// Medio de pago según Catálogo 59.
    pub medio_pago: CpeMedioPago,
    /// Número de cuenta bancaria o referencia de depósito.
    pub cuenta_bancaria: Option<String>,
}

/// Leyenda tributaria a incluir en el comprobante (Catálogo 52).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeLeyenda {
    /// Código oficial de la leyenda (ej: 1000, 2000, 2001).
    pub codigo: CpeCodigoLeyenda,
    /// Texto explicativo de la leyenda.
    pub texto: String,
}

/// Documento tributario de referencia (Guía de Remisión, Factura previa, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpeDocumentoReferencia {
    /// Tipo de documento según Catálogo 01 (ej: "01", "09").
    pub tipo_documento: CpeTipoDocumento,
    /// Serie y número del documento referenciado (ej: "F001-00001234").
    pub serie_numero: String,
    /// Fecha de emisión del comprobante previo (formato YYYY-MM-DD).
    pub fecha_emision: Option<String>,
}

/// Resumen de importes totales e impuestos liquidados en el CPE.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct CpeTotales {
    /// Total de operaciones gravadas (Base imponible IGV).
    pub total_operaciones_gravadas: Decimal,
    /// Total de operaciones inafectas.
    pub total_operaciones_inafectas: Decimal,
    /// Total de operaciones exoneradas.
    pub total_operaciones_exoneradas: Decimal,
    /// Total de operaciones gratuitas.
    pub total_operaciones_gratuitas: Decimal,
    /// Total de operaciones de exportación.
    pub total_operaciones_exportacion: Decimal,
    /// Total general de IGV liquidado (18% / 10%).
    pub total_igv: Decimal,
    /// Total de Impuesto Selectivo al Consumo (ISC).
    pub total_isc: Decimal,
    /// Total de otros tributos y cargos.
    pub total_otros_tributos: Decimal,
    /// Total de Impuesto a las Bolsas Plásticas (ICBPER).
    pub total_icbper: Decimal,
    /// Total de descuentos globales concedidos.
    pub total_descuentos_globales: Decimal,
    /// Importe total a pagar (PayableAmount).
    pub importe_total_pagar: Decimal,
}
