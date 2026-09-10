//! # Catálogo No. 09: Códigos de Tipo de Nota de Crédito Electrónica
//!
//! Motivos autorizados por SUNAT para la emisión de Notas de Crédito vinculadas a Facturas o Boletas.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Tipo de Nota de Crédito Electrónica según Catálogo 09 de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpeTipoNotaCredito {
    /// 01: Anulación de la operación
    AnulacionOperacion,
    /// 02: Anulación por error en el RUC
    AnulacionPorErrorEnRuc,
    /// 03: Corrección por error en la descripción
    CorreccionPorErrorEnDescripcion,
    /// 04: Descuento global
    DescuentoGlobal,
    /// 05: Descuento por ítem
    DescuentoPorItem,
    /// 06: Devolución total
    DevolucionTotal,
    /// 07: Devolución por ítem
    DevolucionPorItem,
    /// 08: Bonificación
    Bonificacion,
    /// 09: Disminución en el valor
    DisminucionEnValor,
    /// 10: Otros Conceptos
    OtrosConceptos,
    /// 11: Ajustes de operaciones de exportación
    AjustesExportacion,
    /// 12: Ajustes afectos al IVAP
    AjustesIvap,
    /// 13: Corrección del monto neto pendiente de pago y/o la(s) fechas de vencimiento del pago único o de las cuotas
    CorreccionMontoNetoOFechasPago,
    /// Otro código para compatibilidad hacia adelante
    Otro(String),
}

impl CpeTipoNotaCredito {
    /// Retorna el código de 2 caracteres.
    #[must_use]
    pub fn codigo(&self) -> &str {
        match self {
            Self::AnulacionOperacion => "01",
            Self::AnulacionPorErrorEnRuc => "02",
            Self::CorreccionPorErrorEnDescripcion => "03",
            Self::DescuentoGlobal => "04",
            Self::DescuentoPorItem => "05",
            Self::DevolucionTotal => "06",
            Self::DevolucionPorItem => "07",
            Self::Bonificacion => "08",
            Self::DisminucionEnValor => "09",
            Self::OtrosConceptos => "10",
            Self::AjustesExportacion => "11",
            Self::AjustesIvap => "12",
            Self::CorreccionMontoNetoOFechasPago => "13",
            Self::Otro(c) => c.as_str(),
        }
    }

    /// Retorna la descripción del motivo de la Nota de Crédito.
    #[must_use]
    pub fn descripcion(&self) -> &str {
        match self {
            Self::AnulacionOperacion => "Anulación de la operación",
            Self::AnulacionPorErrorEnRuc => "Anulación por error en el RUC",
            Self::CorreccionPorErrorEnDescripcion => "Corrección por error en la descripción",
            Self::DescuentoGlobal => "Descuento global",
            Self::DescuentoPorItem => "Descuento por ítem",
            Self::DevolucionTotal => "Devolución total",
            Self::DevolucionPorItem => "Devolución por ítem",
            Self::Bonificacion => "Bonificación",
            Self::DisminucionEnValor => "Disminución en el valor",
            Self::OtrosConceptos => "Otros Conceptos",
            Self::AjustesExportacion => "Ajustes de operaciones de exportación",
            Self::AjustesIvap => "Ajustes afectos al IVAP",
            Self::CorreccionMontoNetoOFechasPago => {
                "Corrección del monto neto pendiente de pago y/o fechas de cuotas"
            }
            Self::Otro(_) => "Otro motivo de Nota de Crédito",
        }
    }

    /// Parsea el código numérico.
    #[must_use]
    pub fn desde_codigo(codigo: &str) -> Self {
        match codigo.trim() {
            "01" => Self::AnulacionOperacion,
            "02" => Self::AnulacionPorErrorEnRuc,
            "03" => Self::CorreccionPorErrorEnDescripcion,
            "04" => Self::DescuentoGlobal,
            "05" => Self::DescuentoPorItem,
            "06" => Self::DevolucionTotal,
            "07" => Self::DevolucionPorItem,
            "08" => Self::Bonificacion,
            "09" => Self::DisminucionEnValor,
            "10" => Self::OtrosConceptos,
            "11" => Self::AjustesExportacion,
            "12" => Self::AjustesIvap,
            "13" => Self::CorreccionMontoNetoOFechasPago,
            otro => Self::Otro(otro.to_string()),
        }
    }
}

impl fmt::Display for CpeTipoNotaCredito {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
