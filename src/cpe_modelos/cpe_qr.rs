//! # Módulo de Representación Impresa y Código QR - `cpe_qr`
//!
//! Generación de la cadena de texto oficial para el código de barras bidimensional (Código QR)
//! exigido por la Resolución de Superintendencia N.° 097-2012/SUNAT y normas complementarias.

use super::{CpeBoleta, CpeFactura, CpeNotaCredito, CpeNotaDebito};

/// Trait para comprobantes que pueden generar la cadena oficial para Código QR.
pub trait CpeRepresentacionImpresa {
    /// Genera la cadena de 10 campos normalizada para el Código QR según el estándar SUNAT:
    /// `{RUC}|{TipoCPE}|{Serie}|{Numero}|{MontoIGV}|{ImporteTotal}|{FechaEmision}|{TipoDocReceptor}|{NumDocReceptor}|{CodigoHash}|`
    fn cpe_generar_cadena_qr(&self, codigo_hash: &str) -> String;
}

impl CpeRepresentacionImpresa for CpeFactura {
    fn cpe_generar_cadena_qr(&self, codigo_hash: &str) -> String {
        format!(
            "{}|{}|{}|{}|{:.2}|{:.2}|{}|{}|{}|{}|",
            self.emisor.ruc.trim(),
            self.tipo_documento().codigo(),
            self.serie.trim(),
            self.correlativo,
            self.totales.total_igv,
            self.totales.importe_total_pagar,
            self.fecha_emision.trim(),
            self.receptor.tipo_documento.codigo(),
            self.receptor.numero_documento.trim(),
            codigo_hash.trim()
        )
    }
}

impl CpeRepresentacionImpresa for CpeBoleta {
    fn cpe_generar_cadena_qr(&self, codigo_hash: &str) -> String {
        format!(
            "{}|{}|{}|{}|{:.2}|{:.2}|{}|{}|{}|{}|",
            self.emisor.ruc.trim(),
            self.tipo_documento().codigo(),
            self.serie.trim(),
            self.correlativo,
            self.totales.total_igv,
            self.totales.importe_total_pagar,
            self.fecha_emision.trim(),
            self.receptor.tipo_documento.codigo(),
            self.receptor.numero_documento.trim(),
            codigo_hash.trim()
        )
    }
}

impl CpeRepresentacionImpresa for CpeNotaCredito {
    fn cpe_generar_cadena_qr(&self, codigo_hash: &str) -> String {
        format!(
            "{}|{}|{}|{}|{:.2}|{:.2}|{}|{}|{}|{}|",
            self.emisor.ruc.trim(),
            self.tipo_documento().codigo(),
            self.serie.trim(),
            self.correlativo,
            self.totales.total_igv,
            self.totales.importe_total_pagar,
            self.fecha_emision.trim(),
            self.receptor.tipo_documento.codigo(),
            self.receptor.numero_documento.trim(),
            codigo_hash.trim()
        )
    }
}

impl CpeRepresentacionImpresa for CpeNotaDebito {
    fn cpe_generar_cadena_qr(&self, codigo_hash: &str) -> String {
        format!(
            "{}|{}|{}|{}|{:.2}|{:.2}|{}|{}|{}|{}|",
            self.emisor.ruc.trim(),
            self.tipo_documento().codigo(),
            self.serie.trim(),
            self.correlativo,
            self.totales.total_igv,
            self.totales.importe_total_pagar,
            self.fecha_emision.trim(),
            self.receptor.tipo_documento.codigo(),
            self.receptor.numero_documento.trim(),
            codigo_hash.trim()
        )
    }
}
