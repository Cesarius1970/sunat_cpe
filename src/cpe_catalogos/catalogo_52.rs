//! # Catálogo No. 52: Códigos de Leyendas
//!
//! Leyendas tributarias obligatorias en Comprobantes de Pago Electrónicos.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Códigos de leyendas según Catálogo 52 de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpeCodigoLeyenda {
    /// 1000: Monto en Letras
    MontoEnLetras,
    /// 1002: TRANSFERENCIA GRATUITA DE UN BIEN Y/O SERVICIO PRESTADO GRATUITAMENTE
    TransferenciaGratuita,
    /// 2000: COMPROBANTE DE PAGO SUJETO A DETRACCION
    ComprobanteSujetoADetraccion,
    /// 2001: BIENES TRANSFERIDOS EN LA AMAZONIA REGION SELVA PARA SER CONSUMIDOS EN LA MISMA
    BienesAmazonia,
    /// 2002: SERVICIOS PRESTADOS EN LA AMAZONIA REGION SELVA PARA SER CONSUMIDOS EN LA MISMA
    ServiciosAmazonia,
    /// 2003: CONTRATOS DE CONSTRUCCION EJECUTADOS EN LA AMAZONIA REGION SELVA
    ContratosConstruccionAmazonia,
    /// 2004: Agencia de Viaje - Paquete turístico
    AgenciaViajePaqueteTuristico,
    /// 2005: Venta realizada por emisor itinerante
    VentaEmisorItinerante,
    /// 2006: Operación sujeta a detracción - Recursos Hidrobiológicos
    DetraccionRecursosHidrobiologicos,
    /// 2007: Operación sujeta a detracción - Servicios de Transporte de Pasajeros
    DetraccionTransportePasajeros,
    /// 2008: Operación sujeta a detracción - Servicios de Transporte de Carga
    DetraccionTransporteCarga,
    /// 3000: Código de Leyenda adicional / Extensión
    Otro(String),
}

impl CpeCodigoLeyenda {
    /// Retorna el código numérico de 4 dígitos.
    #[must_use]
    pub fn codigo(&self) -> &str {
        match self {
            Self::MontoEnLetras => "1000",
            Self::TransferenciaGratuita => "1002",
            Self::ComprobanteSujetoADetraccion => "2000",
            Self::BienesAmazonia => "2001",
            Self::ServiciosAmazonia => "2002",
            Self::ContratosConstruccionAmazonia => "2003",
            Self::AgenciaViajePaqueteTuristico => "2004",
            Self::VentaEmisorItinerante => "2005",
            Self::DetraccionRecursosHidrobiologicos => "2006",
            Self::DetraccionTransportePasajeros => "2007",
            Self::DetraccionTransporteCarga => "2008",
            Self::Otro(c) => c.as_str(),
        }
    }

    /// Retorna la descripción oficial.
    #[must_use]
    pub fn descripcion(&self) -> &str {
        match self {
            Self::MontoEnLetras => "Monto en Letras",
            Self::TransferenciaGratuita => "TRANSFERENCIA GRATUITA DE UN BIEN Y/O SERVICIO",
            Self::ComprobanteSujetoADetraccion => "COMPROBANTE DE PAGO SUJETO A DETRACCION",
            Self::BienesAmazonia => "BIENES TRANSFERIDOS EN LA AMAZONIA",
            Self::ServiciosAmazonia => "SERVICIOS PRESTADOS EN LA AMAZONIA",
            Self::ContratosConstruccionAmazonia => "CONTRATOS DE CONSTRUCCION EN LA AMAZONIA",
            Self::AgenciaViajePaqueteTuristico => "Agencia de Viaje - Paquete turístico",
            Self::VentaEmisorItinerante => "Venta realizada por emisor itinerante",
            Self::DetraccionRecursosHidrobiologicos => "Sujeto a detracción - Recursos Hidrobiológicos",
            Self::DetraccionTransportePasajeros => "Sujeto a detracción - Transporte Pasajeros",
            Self::DetraccionTransporteCarga => "Sujeto a detracción - Transporte de Carga",
            Self::Otro(_) => "Otra Leyenda Tributaria",
        }
    }

    /// Parsea el código de 4 caracteres.
    #[must_use]
    pub fn desde_codigo(codigo: &str) -> Self {
        match codigo.trim() {
            "1000" => Self::MontoEnLetras,
            "1002" => Self::TransferenciaGratuita,
            "2000" => Self::ComprobanteSujetoADetraccion,
            "2001" => Self::BienesAmazonia,
            "2002" => Self::ServiciosAmazonia,
            "2003" => Self::ContratosConstruccionAmazonia,
            "2004" => Self::AgenciaViajePaqueteTuristico,
            "2005" => Self::VentaEmisorItinerante,
            "2006" => Self::DetraccionRecursosHidrobiologicos,
            "2007" => Self::DetraccionTransportePasajeros,
            "2008" => Self::DetraccionTransporteCarga,
            otro => Self::Otro(otro.to_string()),
        }
    }
}

impl fmt::Display for CpeCodigoLeyenda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
