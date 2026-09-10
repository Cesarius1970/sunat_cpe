//! # Catálogo No. 06: Códigos de Tipos de Documentos de Identidad
//!
//! Documentos de identidad reconocidos por SUNAT para emisores y receptores de CPE.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Códigos de tipos de documento de identidad según Catálogo 06 de SUNAT.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CpeTipoDocumentoIdentidad {
    /// 0: Documento tribal / Sin documento
    SinDocumento,
    /// 1: Documento Nacional de Identidad (DNI)
    DNI,
    /// 4: Carnet de Extranjería
    CarnetExtranjeria,
    /// 6: Registro Único de Contribuyentes (RUC)
    RUC,
    /// 7: Pasaporte
    Pasaporte,
    /// A: Cédula Diplomática de Identidad
    CedulaDiplomatica,
    /// B: Doc. Identidad País Residencia No Domiciliado
    DocPaisResidenciaNoDomiciliado,
    /// C: Tax Identification Number (TIN / RFC)
    TaxIdentificationNumber,
    /// G: Salvoconducto
    Salvoconducto,
    /// Código adicional para compatibilidad
    Otro(String),
}

impl CpeTipoDocumentoIdentidad {
    /// Retorna el código asignado por SUNAT.
    #[must_use]
    pub fn codigo(&self) -> &str {
        match self {
            Self::SinDocumento => "0",
            Self::DNI => "1",
            Self::CarnetExtranjeria => "4",
            Self::RUC => "6",
            Self::Pasaporte => "7",
            Self::CedulaDiplomatica => "A",
            Self::DocPaisResidenciaNoDomiciliado => "B",
            Self::TaxIdentificationNumber => "C",
            Self::Salvoconducto => "G",
            Self::Otro(c) => c.as_str(),
        }
    }

    /// Retorna la descripción oficial.
    #[must_use]
    pub fn descripcion(&self) -> &str {
        match self {
            Self::SinDocumento => "Doc.trib.no.dom.sin.ruc / Sin documento",
            Self::DNI => "Documento Nacional de Identidad (DNI)",
            Self::CarnetExtranjeria => "Carnet de Extranjería",
            Self::RUC => "Registro Único de Contribuyentes (RUC)",
            Self::Pasaporte => "Pasaporte",
            Self::CedulaDiplomatica => "Cédula Diplomática de Identidad",
            Self::DocPaisResidenciaNoDomiciliado => "Doc. Identidad País Residencia-No.Domiciliado",
            Self::TaxIdentificationNumber => "Tax Identification Number - TIN / Doc Trib",
            Self::Salvoconducto => "Salvoconducto",
            Self::Otro(_) => "Otro Documento de Identidad",
        }
    }

    /// Parsea el código a la variante correspondiente.
    #[must_use]
    pub fn desde_codigo(codigo: &str) -> Self {
        match codigo.trim() {
            "0" => Self::SinDocumento,
            "1" => Self::DNI,
            "4" => Self::CarnetExtranjeria,
            "6" => Self::RUC,
            "7" => Self::Pasaporte,
            "A" | "a" => Self::CedulaDiplomatica,
            "B" | "b" => Self::DocPaisResidenciaNoDomiciliado,
            "C" | "c" => Self::TaxIdentificationNumber,
            "G" | "g" => Self::Salvoconducto,
            otro => Self::Otro(otro.to_string()),
        }
    }
}

impl fmt::Display for CpeTipoDocumentoIdentidad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.codigo())
    }
}
