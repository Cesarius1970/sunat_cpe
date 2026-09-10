//! # Catálogo No. 10: Códigos de Tipo de Nota de Débito Electrónica
//!
//! Motivos autorizados por SUNAT para la emisión de Notas de Débito vinculadas a Facturas o Boletas.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Tipo de Nota de Débito Electrónica según Catálogo 10 de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpeTipoNotaDebito {
    /// 01: Intereses por mora
    InteresesPorMora,
    /// 02: Aumento en el valor
    AumentoEnValor,
    /// 03: Penalidades / otros conceptos
    PenalidadesOtrosConceptos,
    /// 10: Ajustes de operaciones de exportación
    AjustesExportacion,
    /// 11: Ajustes afectos al IVAP
    AjustesIvap,
    /// Otro código para compatibilidad hacia adelante
    Otro(String),
}

impl CpeTipoNotaDebito {
    /// Retorna el código de 2 caracteres.
    #[must_use]
    pub fn codigo(&self) -> &str {
        match self {
            Self::InteresesPorMora => "01",
            Self::AumentoEnValor => "02",
            Self::PenalidadesOtrosConceptos => "03",
            Self::AjustesExportacion => "10",
            Self::AjustesIvap => "11",
            Self::Otro(c) => c.as_str(),
        }
    }

    /// Retorna la descripción del motivo de la Nota de Débito.
    #[must_use]
    pub fn descripcion(&self) -> &str {
        match self {
            Self::InteresesPorMora => "Intereses por mora",
            Self::AumentoEnValor => "Aumento en el valor",
            Self::PenalidadesOtrosConceptos => "Penalidades / otros conceptos",
            Self::AjustesExportacion => "Ajustes de operaciones de exportación",
            Self::AjustesIvap => "Ajustes afectos al IVAP",
            Self::Otro(_) => "Otro motivo de Nota de Débito",
        }
    }

    /// Parsea el código numérico.
    #[must_use]
    pub fn desde_codigo(codigo: &str) -> Self {
        match codigo.trim() {
            "01" => Self::InteresesPorMora,
            "02" => Self::AumentoEnValor,
            "03" => Self::PenalidadesOtrosConceptos,
            "10" => Self::AjustesExportacion,
            "11" => Self::AjustesIvap,
            otro => Self::Otro(otro.to_string()),
        }
    }
}

impl fmt::Display for CpeTipoNotaDebito {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
