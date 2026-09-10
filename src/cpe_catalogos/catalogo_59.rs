//! # Catálogo No. 59: Códigos de Medios de Pago
//!
//! Medios de pago autorizados por la SUNAT en el marco de la bancarización.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Códigos de medios de pago según Catálogo 59 de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpeMedioPago {
    /// 001: Depósito en cuenta
    DepositoEnCuenta,
    /// 002: Giro
    Giro,
    /// 003: Transferencia de fondos
    TransferenciaFondos,
    /// 004: Orden de pago
    OrdenPago,
    /// 005: Tarjeta de débito
    TarjetaDebito,
    /// 006: Tarjeta de crédito
    TarjetaCredito,
    /// 007: Cheques con cláusula "no negociable"
    ChequeNoNegociable,
    /// 008: Efectivo por operaciones sin obligación de bancarizar
    Efectivo,
    /// 009: Efectivo en los demás casos
    EfectivoOtros,
    /// 010: Medios de pago de comercio exterior
    ComercioExterior,
    /// Otro medio de pago
    Otro(String),
}

impl CpeMedioPago {
    /// Retorna el código numérico de 3 dígitos.
    #[must_use]
    pub fn codigo(&self) -> &str {
        match self {
            Self::DepositoEnCuenta => "001",
            Self::Giro => "002",
            Self::TransferenciaFondos => "003",
            Self::OrdenPago => "004",
            Self::TarjetaDebito => "005",
            Self::TarjetaCredito => "006",
            Self::ChequeNoNegociable => "007",
            Self::Efectivo => "008",
            Self::EfectivoOtros => "009",
            Self::ComercioExterior => "010",
            Self::Otro(c) => c.as_str(),
        }
    }

    /// Retorna la descripción oficial.
    #[must_use]
    pub fn descripcion(&self) -> &str {
        match self {
            Self::DepositoEnCuenta => "Depósito en cuenta",
            Self::Giro => "Giro",
            Self::TransferenciaFondos => "Transferencia de fondos",
            Self::OrdenPago => "Orden de pago",
            Self::TarjetaDebito => "Tarjeta de débito",
            Self::TarjetaCredito => "Tarjeta de crédito",
            Self::ChequeNoNegociable => "Cheques con la cláusula 'no negociable'",
            Self::Efectivo => "Efectivo",
            Self::EfectivoOtros => "Efectivo en los demás casos",
            Self::ComercioExterior => "Medios de pago usados en comercio exterior",
            Self::Otro(_) => "Otro Medio de Pago",
        }
    }

    /// Parsea el código de 3 dígitos.
    #[must_use]
    pub fn desde_codigo(codigo: &str) -> Self {
        match codigo.trim() {
            "001" => Self::DepositoEnCuenta,
            "002" => Self::Giro,
            "003" => Self::TransferenciaFondos,
            "004" => Self::OrdenPago,
            "005" => Self::TarjetaDebito,
            "006" => Self::TarjetaCredito,
            "007" => Self::ChequeNoNegociable,
            "008" => Self::Efectivo,
            "009" => Self::EfectivoOtros,
            "010" => Self::ComercioExterior,
            otro => Self::Otro(otro.to_string()),
        }
    }
}

impl fmt::Display for CpeMedioPago {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
