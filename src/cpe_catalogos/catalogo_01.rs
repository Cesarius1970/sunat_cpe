//! # Catálogo No. 01: Código de Tipo de Documento
//!
//! Especificación oficial de SUNAT para los tipos de comprobantes de pago y documentos relacionados.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Códigos de tipo de documento según el Catálogo 01 de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpeTipoDocumento {
    /// 01: Factura
    Factura,
    /// 03: Boleta de Venta
    BoletaVenta,
    /// 07: Nota de Crédito
    NotaCredito,
    /// 08: Nota de Débito
    NotaDebito,
    /// 09: Guía de Remisión Remitente
    GuiaRemisionRemitente,
    /// 12: Ticket o cinta emitido por máquina registradora
    TicketMaquinaRegistradora,
    /// 13: Documento emitido por bancos, instituciones financieras
    DocumentoBancario,
    /// 14: Recibo por servicios públicos
    ReciboServiciosPublicos,
    /// 20: Comprobante de Retención
    ComprobanteRetencion,
    /// 31: Guía de Remisión Transportista
    GuiaRemisionTransportista,
    /// 40: Comprobante de Percepción
    ComprobantePercepcion,
    /// RC: Resumen Diario de Boletas de Venta y Notas vinculadas
    ResumenDiario,
    /// RA: Comunicación de Baja
    ComunicacionBaja,
    /// Cualquier otro código nuevo publicado por SUNAT para compatibilidad hacia adelante.
    Desconocido(String),
}

impl CpeTipoDocumento {
    /// Retorna el código oficial de SUNAT.
    #[must_use]
    pub fn codigo(&self) -> &str {
        match self {
            Self::Factura => "01",
            Self::BoletaVenta => "03",
            Self::NotaCredito => "07",
            Self::NotaDebito => "08",
            Self::GuiaRemisionRemitente => "09",
            Self::TicketMaquinaRegistradora => "12",
            Self::DocumentoBancario => "13",
            Self::ReciboServiciosPublicos => "14",
            Self::ComprobanteRetencion => "20",
            Self::GuiaRemisionTransportista => "31",
            Self::ComprobantePercepcion => "40",
            Self::ResumenDiario => "RC",
            Self::ComunicacionBaja => "RA",
            Self::Desconocido(c) => c.as_str(),
        }
    }

    /// Retorna la descripción oficial según el catálogo de SUNAT.
    #[must_use]
    pub fn descripcion(&self) -> &str {
        match self {
            Self::Factura => "Factura",
            Self::BoletaVenta => "Boleta de Venta",
            Self::NotaCredito => "Nota de Crédito",
            Self::NotaDebito => "Nota de Débito",
            Self::GuiaRemisionRemitente => "Guía de Remisión Remitente",
            Self::TicketMaquinaRegistradora => "Ticket o cinta emitida por máquina registradora",
            Self::DocumentoBancario => "Documento emitido por bancos y entidades financieras",
            Self::ReciboServiciosPublicos => "Recibo emitido por los servicios públicos",
            Self::ComprobanteRetencion => "Comprobante de Retención",
            Self::GuiaRemisionTransportista => "Guía de Remisión Transportista",
            Self::ComprobantePercepcion => "Comprobante de Percepción",
            Self::ResumenDiario => "Resumen Diario de Boletas y Notas",
            Self::ComunicacionBaja => "Comunicación de Baja",
            Self::Desconocido(_) => "Tipo de Documento No Catalogado / Extensión SUNAT",
        }
    }

    /// Parsea una cadena de texto al tipo correspondiente.
    #[must_use]
    pub fn desde_codigo(codigo: &str) -> Self {
        match codigo.trim() {
            "01" => Self::Factura,
            "03" => Self::BoletaVenta,
            "07" => Self::NotaCredito,
            "08" => Self::NotaDebito,
            "09" => Self::GuiaRemisionRemitente,
            "12" => Self::TicketMaquinaRegistradora,
            "13" => Self::DocumentoBancario,
            "14" => Self::ReciboServiciosPublicos,
            "20" => Self::ComprobanteRetencion,
            "31" => Self::GuiaRemisionTransportista,
            "40" => Self::ComprobantePercepcion,
            "RC" => Self::ResumenDiario,
            "RA" => Self::ComunicacionBaja,
            otro => Self::Desconocido(otro.to_string()),
        }
    }
}

impl fmt::Display for CpeTipoDocumento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
