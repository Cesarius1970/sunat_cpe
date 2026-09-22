//! # Catálogo No. 51 / Catálogo No. 17: Código de Tipo de Operación
//!
//! Clasificación de la naturaleza económica o tributaria de la operación comercial.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Códigos de tipo de operación según Catálogo 51 / 17 de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpeTipoOperacion {
    /// 0101: Venta interna (General)
    VentaInterna,
    /// 0102: Exportación de bienes
    ExportacionBienes,
    /// 0103: No domiciliados
    NoDomiciliados,
    /// 0104: Venta interna - Anticipos
    VentaInternaAnticipos,
    /// 0112: Venta interna - Itinerante
    VentaInternaItinerante,
    /// 0113: Venta interna a través de comisionistas o consignatarios
    VentaInternaComisionistas,
    /// 0200: Exportación de servicios
    ExportacionServicios,
    /// 0201: Exportación de servicios - Hospedaje y alimentación a sujetos no domiciliados
    ExportacionServiciosHospedaje,
    /// 0202: Exportación de servicios - Transporte internacional de carga
    ExportacionServiciosTransporte,
    /// 0401: Venta no domiciliada que no califica como exportación
    VentaNoDomiciliadaNoExportacion,
    /// 1001: Operación sujeta a Detracción
    OperacionSujetaADetraccion,
    /// 1002: Operación sujeta a Detracción - Recursos Hidrobiológicos
    OperacionSujetaADetraccionHidrobiologicos,
    /// 1003: Operación sujeta a Detracción - Servicios de Transporte de Pasajeros
    OperacionSujetaADetraccionTransportePasajeros,
    /// 1004: Operación sujeta a Detracción - Servicios de Transporte de Carga
    OperacionSujetaADetraccionTransporteCarga,
    /// 2001: Operación sujeta a Percepción
    OperacionSujetaAPercepcion,
    /// Código adicional para compatibilidad
    Otro(String),
}

impl CpeTipoOperacion {
    /// Retorna el código numérico de 4 dígitos.
    #[must_use]
    pub fn codigo(&self) -> &str {
        match self {
            Self::VentaInterna => "0101",
            Self::ExportacionBienes => "0102",
            Self::NoDomiciliados => "0103",
            Self::VentaInternaAnticipos => "0104",
            Self::VentaInternaItinerante => "0112",
            Self::VentaInternaComisionistas => "0113",
            Self::ExportacionServicios => "0200",
            Self::ExportacionServiciosHospedaje => "0201",
            Self::ExportacionServiciosTransporte => "0202",
            Self::VentaNoDomiciliadaNoExportacion => "0401",
            Self::OperacionSujetaADetraccion => "1001",
            Self::OperacionSujetaADetraccionHidrobiologicos => "1002",
            Self::OperacionSujetaADetraccionTransportePasajeros => "1003",
            Self::OperacionSujetaADetraccionTransporteCarga => "1004",
            Self::OperacionSujetaAPercepcion => "2001",
            Self::Otro(c) => c.as_str(),
        }
    }

    /// Retorna la descripción oficial.
    #[must_use]
    pub fn descripcion(&self) -> &str {
        match self {
            Self::VentaInterna => "Venta interna",
            Self::ExportacionBienes => "Exportación de bienes",
            Self::NoDomiciliados => "No domiciliados",
            Self::VentaInternaAnticipos => "Venta interna - Anticipos",
            Self::VentaInternaItinerante => "Venta interna - Itinerante",
            Self::VentaInternaComisionistas => "Venta interna a través de comisionistas",
            Self::ExportacionServicios => "Exportación de servicios",
            Self::ExportacionServiciosHospedaje => {
                "Exportación de servicios - Hospedaje a no domiciliados"
            }
            Self::ExportacionServiciosTransporte => {
                "Exportación de servicios - Transporte internacional de carga"
            }
            Self::VentaNoDomiciliadaNoExportacion => {
                "Venta no domiciliada que no califica como exp."
            }
            Self::OperacionSujetaADetraccion => "Operación Sujeta a Detracción",
            Self::OperacionSujetaADetraccionHidrobiologicos => {
                "Operación Sujeta a Detracción - Recursos Hidrobiológicos"
            }
            Self::OperacionSujetaADetraccionTransportePasajeros => {
                "Operación Sujeta a Detracción - Servicios de Transporte de Pasajeros"
            }
            Self::OperacionSujetaADetraccionTransporteCarga => {
                "Operación Sujeta a Detracción - Servicios de Transporte de Carga"
            }
            Self::OperacionSujetaAPercepcion => "Operación Sujeta a Percepción",
            Self::Otro(_) => "Otro Tipo de Operación",
        }
    }

    /// Parsea el código de 4 caracteres.
    #[must_use]
    pub fn desde_codigo(codigo: &str) -> Self {
        match codigo.trim() {
            "0101" => Self::VentaInterna,
            "0102" => Self::ExportacionBienes,
            "0103" => Self::NoDomiciliados,
            "0104" => Self::VentaInternaAnticipos,
            "0112" => Self::VentaInternaItinerante,
            "0113" => Self::VentaInternaComisionistas,
            "0200" => Self::ExportacionServicios,
            "0201" => Self::ExportacionServiciosHospedaje,
            "0202" => Self::ExportacionServiciosTransporte,
            "0401" => Self::VentaNoDomiciliadaNoExportacion,
            "1001" => Self::OperacionSujetaADetraccion,
            "1002" => Self::OperacionSujetaADetraccionHidrobiologicos,
            "1003" => Self::OperacionSujetaADetraccionTransportePasajeros,
            "1004" => Self::OperacionSujetaADetraccionTransporteCarga,
            "2001" => Self::OperacionSujetaAPercepcion,
            otro => Self::Otro(otro.to_string()),
        }
    }
}

impl fmt::Display for CpeTipoOperacion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
