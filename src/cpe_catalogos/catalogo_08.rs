//! # Catálogo No. 08: Códigos de Tipos de Sistema de Cálculo del ISC
//!
//! Sistemas de cálculo para el Impuesto Selectivo al Consumo (ISC).

use serde::{Deserialize, Serialize};
use std::fmt;

/// Tipo de Sistema de Cálculo del ISC según Catálogo 08 de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpeTipoSistemaIsc {
    /// 01: Sistema al Valor (Apéndice IV, lit. A - T.U.O IGV e ISC)
    SistemaAlValor,
    /// 02: Aplicación del Monto Fijo (Apéndice IV, lit. B - T.U.O IGV e ISC)
    AplicacionMontoFijo,
    /// 03: Sistema de Precios de Venta al Público (Apéndice IV, lit. C - T.U.O IGV e ISC)
    SistemaPreciosVentaAlPublico,
    /// Otro código para extensibilidad
    Otro(String),
}

impl CpeTipoSistemaIsc {
    /// Retorna el código asignado por SUNAT.
    #[must_use]
    pub fn codigo(&self) -> &str {
        match self {
            Self::SistemaAlValor => "01",
            Self::AplicacionMontoFijo => "02",
            Self::SistemaPreciosVentaAlPublico => "03",
            Self::Otro(c) => c.as_str(),
        }
    }

    /// Retorna la descripción oficial.
    #[must_use]
    pub fn descripcion(&self) -> &str {
        match self {
            Self::SistemaAlValor => "Sistema al Valor",
            Self::AplicacionMontoFijo => "Aplicación del Monto Fijo (Específico)",
            Self::SistemaPreciosVentaAlPublico => "Sistema de Precios de Venta al Público",
            Self::Otro(_) => "Otro Sistema de Cálculo ISC",
        }
    }

    /// Parsea el código a la variante correspondiente.
    #[must_use]
    pub fn desde_codigo(codigo: &str) -> Self {
        match codigo.trim() {
            "01" => Self::SistemaAlValor,
            "02" => Self::AplicacionMontoFijo,
            "03" => Self::SistemaPreciosVentaAlPublico,
            otro => Self::Otro(otro.to_string()),
        }
    }
}

impl fmt::Display for CpeTipoSistemaIsc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
