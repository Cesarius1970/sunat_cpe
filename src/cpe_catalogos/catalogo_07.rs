//! # Catálogo No. 07: Códigos de Tipo de Afectación al IGV
//!
//! Clasificación tributaria del Impuesto General a las Ventas (IGV) para cada ítem de un CPE.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Tipo de Afectación al IGV según Catálogo 07 de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpeTipoAfectacionIgv {
    // --- Operaciones Gravadas (10 - 17) ---
    /// 10: Gravado - Operación Onerosa
    GravadoOperacionOnerosa,
    /// 11: Gravado - Retiro por premio
    GravadoRetiroPremio,
    /// 12: Gravado - Retiro por donación
    GravadoRetiroDonacion,
    /// 13: Gravado - Retiro
    GravadoRetiro,
    /// 14: Gravado - Retiro por publicidad
    GravadoRetiroPublicidad,
    /// 15: Gravado - Bonificaciones
    GravadoBonificaciones,
    /// 16: Gravado - Retiro por entrega a trabajadores
    GravadoRetiroTrabajadores,
    /// 17: Gravado - IVAP
    GravadoIvap,

    // --- Operaciones Exoneradas (20 - 21) ---
    /// 20: Exonerado - Operación Onerosa
    ExoneradoOperacionOnerosa,
    /// 21: Exonerado - Transferencia Gratuita
    ExoneradoTransferenciaGratuita,

    // --- Operaciones Inafectas (30 - 37) ---
    /// 30: Inafecto - Operación Onerosa
    InafectoOperacionOnerosa,
    /// 31: Inafecto - Retiro por Bonificación
    InafectoRetiroBonificacion,
    /// 32: Inafecto - Retiro
    InafectoRetiro,
    /// 33: Inafecto - Retiro por Muestras Médicas
    InafectoRetiroMuestrasMedicas,
    /// 34: Inafecto - Retiro por Convenio Colectivo
    InafectoRetiroConvenioColectivo,
    /// 35: Inafecto - Retiro por premio
    InafectoRetiroPremio,
    /// 36: Inafecto - Retiro por publicidad
    InafectoRetiroPublicidad,
    /// 37: Inafecto - Transferencia Gratuita
    InafectoTransferenciaGratuita,

    // --- Exportación (40) ---
    /// 40: Exportación de Bienes o Servicios
    Exportacion,

    /// Otro código para extensibilidad
    Otro(String),
}

impl CpeTipoAfectacionIgv {
    /// Retorna el código numérico de 2 dígitos asignado por SUNAT.
    #[must_use]
    pub fn codigo(&self) -> &str {
        match self {
            Self::GravadoOperacionOnerosa => "10",
            Self::GravadoRetiroPremio => "11",
            Self::GravadoRetiroDonacion => "12",
            Self::GravadoRetiro => "13",
            Self::GravadoRetiroPublicidad => "14",
            Self::GravadoBonificaciones => "15",
            Self::GravadoRetiroTrabajadores => "16",
            Self::GravadoIvap => "17",
            Self::ExoneradoOperacionOnerosa => "20",
            Self::ExoneradoTransferenciaGratuita => "21",
            Self::InafectoOperacionOnerosa => "30",
            Self::InafectoRetiroBonificacion => "31",
            Self::InafectoRetiro => "32",
            Self::InafectoRetiroMuestrasMedicas => "33",
            Self::InafectoRetiroConvenioColectivo => "34",
            Self::InafectoRetiroPremio => "35",
            Self::InafectoRetiroPublicidad => "36",
            Self::InafectoTransferenciaGratuita => "37",
            Self::Exportacion => "40",
            Self::Otro(c) => c.as_str(),
        }
    }

    /// Retorna el código tributario internacional UBL (VAT / FRE / INA / EXP).
    #[must_use]
    pub fn codigo_tributo_ubl(&self) -> &str {
        match self {
            Self::GravadoOperacionOnerosa
            | Self::GravadoRetiroPremio
            | Self::GravadoRetiroDonacion
            | Self::GravadoRetiro
            | Self::GravadoRetiroPublicidad
            | Self::GravadoBonificaciones
            | Self::GravadoRetiroTrabajadores
            | Self::GravadoIvap => "1000", // IGV
            Self::ExoneradoOperacionOnerosa | Self::ExoneradoTransferenciaGratuita => "9997", // EXONERADO
            Self::InafectoOperacionOnerosa
            | Self::InafectoRetiroBonificacion
            | Self::InafectoRetiro
            | Self::InafectoRetiroMuestrasMedicas
            | Self::InafectoRetiroConvenioColectivo
            | Self::InafectoRetiroPremio
            | Self::InafectoRetiroPublicidad
            | Self::InafectoTransferenciaGratuita => "9998", // INAFECTO
            Self::Exportacion => "9995", // EXPORTACION
            Self::Otro(_) => "9996",     // GRATUITO / OTRO
        }
    }

    /// Determina si la operación es onerosa (con cobro comercial efectivo).
    #[must_use]
    pub fn es_operacion_onerosa(&self) -> bool {
        matches!(
            self,
            Self::GravadoOperacionOnerosa
                | Self::ExoneradoOperacionOnerosa
                | Self::InafectoOperacionOnerosa
                | Self::Exportacion
        )
    }

    /// Determina si la operación es a título gratuito (requiere leyenda y valor referencial).
    #[must_use]
    pub fn es_gratuita(&self) -> bool {
        !self.es_operacion_onerosa()
    }

    /// Parsea el código de 2 dígitos.
    #[must_use]
    pub fn desde_codigo(codigo: &str) -> Self {
        match codigo.trim() {
            "10" => Self::GravadoOperacionOnerosa,
            "11" => Self::GravadoRetiroPremio,
            "12" => Self::GravadoRetiroDonacion,
            "13" => Self::GravadoRetiro,
            "14" => Self::GravadoRetiroPublicidad,
            "15" => Self::GravadoBonificaciones,
            "16" => Self::GravadoRetiroTrabajadores,
            "17" => Self::GravadoIvap,
            "20" => Self::ExoneradoOperacionOnerosa,
            "21" => Self::ExoneradoTransferenciaGratuita,
            "30" => Self::InafectoOperacionOnerosa,
            "31" => Self::InafectoRetiroBonificacion,
            "32" => Self::InafectoRetiro,
            "33" => Self::InafectoRetiroMuestrasMedicas,
            "34" => Self::InafectoRetiroConvenioColectivo,
            "35" => Self::InafectoRetiroPremio,
            "36" => Self::InafectoRetiroPublicidad,
            "37" => Self::InafectoTransferenciaGratuita,
            "40" => Self::Exportacion,
            otro => Self::Otro(otro.to_string()),
        }
    }
}

impl fmt::Display for CpeTipoAfectacionIgv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
