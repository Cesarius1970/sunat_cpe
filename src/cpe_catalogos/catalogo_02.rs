//! # Catálogo No. 02: Código de Tipo de Moneda
//!
//! Monedas autorizadas por SUNAT según estándar internacional ISO 4217.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Código de moneda según Catálogo 02 de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpeTipoMoneda {
    /// Soles (Perú)
    PEN,
    /// Dólares Americanos (EE.UU.)
    USD,
    /// Euros (Unión Europea)
    EUR,
    /// Libras Esterlinas (Reino Unido)
    GBP,
    /// Dólar Canadiense
    CAD,
    /// Franco Suizo
    CHF,
    /// Yen Japonés
    JPY,
    /// Otra moneda ISO 4217
    Otra(String),
}

impl CpeTipoMoneda {
    /// Retorna el código ISO de 3 caracteres.
    #[must_use]
    pub fn codigo(&self) -> &str {
        match self {
            Self::PEN => "PEN",
            Self::USD => "USD",
            Self::EUR => "EUR",
            Self::GBP => "GBP",
            Self::CAD => "CAD",
            Self::CHF => "CHF",
            Self::JPY => "JPY",
            Self::Otra(c) => c.as_str(),
        }
    }

    /// Retorna el nombre descriptivo de la moneda.
    #[must_use]
    pub fn descripcion(&self) -> &str {
        match self {
            Self::PEN => "Soles",
            Self::USD => "Dólares Americanos",
            Self::EUR => "Euros",
            Self::GBP => "Libras Esterlinas",
            Self::CAD => "Dólar Canadiense",
            Self::CHF => "Franco Suizo",
            Self::JPY => "Yen Japonés",
            Self::Otra(_) => "Moneda Extranjera",
        }
    }

    /// Parsea una cadena de texto al tipo de moneda.
    #[must_use]
    pub fn desde_codigo(codigo: &str) -> Self {
        match codigo.trim().to_uppercase().as_str() {
            "PEN" => Self::PEN,
            "USD" => Self::USD,
            "EUR" => Self::EUR,
            "GBP" => Self::GBP,
            "CAD" => Self::CAD,
            "CHF" => Self::CHF,
            "JPY" => Self::JPY,
            otro => Self::Otra(otro.to_string()),
        }
    }
}

impl fmt::Display for CpeTipoMoneda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
