//! # Módulo Validador Pre-Vuelo - `cpe_validador`
//!
//! Validaciones de integridad, formato y consistencia aritmética de comprobantes electrónicos
//! antes de su firma y envío a los servidores de SUNAT.
//!
//! Previene rechazos tempranos, cuotas de red desperdiciadas y discrepancias tributarias.

use crate::cpe_catalogos::{CpeTipoDocumento, CpeTipoDocumentoIdentidad};
use crate::cpe_error::{CpeError, CpeResult};
use crate::cpe_modelos::{CpeBoleta, CpeFactura, CpeNotaCredito, CpeNotaDebito, CpeTotales};
use rust_decimal_macros::dec;

/// Validador estático de reglas tributarias de SUNAT y consistencia estructural de CPEs.
pub struct CpeValidador;

impl CpeValidador {
    /// Valida un número de RUC peruano según el algoritmo oficial de Módulo 11 de SUNAT.
    ///
    /// # Reglas
    /// - Debe contener exactamente 11 dígitos numéricos.
    /// - Debe iniciar con un prefijo válido: 10, 15, 17, 20.
    /// - El último dígito debe coincidir con el dígito verificador calculado.
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorValidacion` si el RUC no cumple con el algoritmo.
    pub fn validar_ruc(ruc: &str) -> CpeResult<()> {
        let ruc = ruc.trim();
        if ruc.len() != 11 || !ruc.chars().all(|c| c.is_ascii_digit()) {
            return Err(CpeError::ErrorValidacion(format!(
                "El RUC '{ruc}' es inválido: debe contener exactamente 11 dígitos numéricos."
            )));
        }

        let prefijo = &ruc[..2];
        if !matches!(prefijo, "10" | "15" | "17" | "20") {
            return Err(CpeError::ErrorValidacion(format!(
                "El RUC '{ruc}' tiene un prefijo inválido '{prefijo}'. Prefijos válidos: 10, 15, 17, 20."
            )));
        }

        let factores = [5, 4, 3, 2, 7, 6, 5, 4, 3, 2];
        let digitos: Vec<u32> = ruc.chars().map(|c| c.to_digit(10).unwrap()).collect();

        let suma: u32 = digitos[..10]
            .iter()
            .zip(factores.iter())
            .map(|(&d, &f)| d * f)
            .sum();

        let residuo = 11 - (suma % 11);
        let digito_esperado = match residuo {
            10 => 0,
            11 => 1,
            otro => otro,
        };

        if digito_esperado != digitos[10] {
            return Err(CpeError::ErrorValidacion(format!(
                "Dígito verificador del RUC '{ruc}' inválido. Esperado: {digito_esperado}, Encontrado: {}.",
                digitos[10]
            )));
        }

        Ok(())
    }

    /// Valida un número de DNI peruano (8 dígitos numéricos).
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorValidacion` si el DNI no contiene 8 dígitos numéricos.
    pub fn validar_dni(dni: &str) -> CpeResult<()> {
        let dni = dni.trim();
        if dni.len() != 8 || !dni.chars().all(|c| c.is_ascii_digit()) {
            return Err(CpeError::ErrorValidacion(format!(
                "El DNI '{dni}' es inválido: debe contener exactamente 8 dígitos numéricos."
            )));
        }
        Ok(())
    }

    /// Valida el formato y la serie de un comprobante según el tipo de documento.
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorValidacion` si la serie no cumple con la nomenclatura de SUNAT.
    pub fn validar_serie(serie: &str, tipo: CpeTipoDocumento) -> CpeResult<()> {
        let serie = serie.trim();
        if serie.len() != 4 {
            return Err(CpeError::ErrorValidacion(format!(
                "La serie '{serie}' debe tener exactamente 4 caracteres alfanuméricos."
            )));
        }

        match tipo {
            CpeTipoDocumento::Factura => {
                let primera_letra = serie.chars().next().unwrap();
                if primera_letra != 'F' && primera_letra != 'E' {
                    return Err(CpeError::ErrorValidacion(format!(
                        "La serie de Factura '{serie}' debe iniciar con 'F' o 'E'."
                    )));
                }
            }
            CpeTipoDocumento::BoletaVenta => {
                let primera_letra = serie.chars().next().unwrap();
                if primera_letra != 'B' && primera_letra != 'E' {
                    return Err(CpeError::ErrorValidacion(format!(
                        "La serie de Boleta '{serie}' debe iniciar con 'B' o 'E'."
                    )));
                }
            }
            CpeTipoDocumento::NotaCredito | CpeTipoDocumento::NotaDebito => {
                let primera_letra = serie.chars().next().unwrap();
                if primera_letra != 'F' && primera_letra != 'B' && primera_letra != 'E' {
                    return Err(CpeError::ErrorValidacion(format!(
                        "La serie de Nota '{serie}' debe iniciar con 'F', 'B' o 'E'."
                    )));
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Valida la consistencia aritmética de los totales del comprobante sin punto flotante.
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorValidacion` si la sumatoria de bases imponibles, tributos y descuentos
    /// difiere del importe total a pagar en más de un céntimo (tolerancia por redondeo de ítems).
    pub fn validar_cuadre_totales(totales: &CpeTotales) -> CpeResult<()> {
        let suma_bases = totales.total_operaciones_gravadas
            + totales.total_operaciones_inafectas
            + totales.total_operaciones_exoneradas
            + totales.total_operaciones_exportacion;

        let suma_tributos = totales.total_igv
            + totales.total_isc
            + totales.total_icbper
            + totales.total_otros_tributos;

        let total_calculado = suma_bases + suma_tributos - totales.total_descuentos_globales;
        let diferencia = (total_calculado - totales.importe_total_pagar).abs();

        if diferencia > dec!(0.01) {
            return Err(CpeError::ErrorValidacion(format!(
                "Descuadre en totales del CPE: Suma calculada ({total_calculado}) != Importe a pagar ({}), diferencia de {diferencia}.",
                totales.importe_total_pagar
            )));
        }

        Ok(())
    }

    /// Valida integralmente una Factura Electrónica antes de su emisión.
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorValidacion` si algún campo obligatorio o regla de negocio no se cumple.
    pub fn validar_factura(factura: &CpeFactura) -> CpeResult<()> {
        Self::validar_ruc(&factura.emisor.ruc)?;

        // El receptor de factura debe tener RUC obligatorio según normativa SUNAT
        if factura.receptor.tipo_documento != CpeTipoDocumentoIdentidad::RUC {
            return Err(CpeError::ErrorValidacion(format!(
                "El receptor de una Factura debe poseer RUC (código 6). Tipo encontrado: {:?}",
                factura.receptor.tipo_documento
            )));
        }
        Self::validar_ruc(&factura.receptor.numero_documento)?;

        Self::validar_serie(&factura.serie, CpeTipoDocumento::Factura)?;

        if factura.correlativo == 0 || factura.correlativo > 99_999_999 {
            return Err(CpeError::ErrorValidacion(format!(
                "Correlativo inválido '{}': debe estar entre 1 y 99999999.",
                factura.correlativo
            )));
        }

        if factura.items.is_empty() {
            return Err(CpeError::ErrorValidacion(
                "La Factura debe contener al menos un ítem de detalle.".to_string(),
            ));
        }

        Self::validar_cuadre_totales(&factura.totales)?;

        Ok(())
    }

    /// Valida integralmente una Boleta de Venta Electrónica antes de su emisión.
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorValidacion` si algún campo obligatorio o regla de negocio no se cumple.
    pub fn validar_boleta(boleta: &CpeBoleta) -> CpeResult<()> {
        Self::validar_ruc(&boleta.emisor.ruc)?;
        Self::validar_serie(&boleta.serie, CpeTipoDocumento::BoletaVenta)?;

        if boleta.correlativo == 0 || boleta.correlativo > 99_999_999 {
            return Err(CpeError::ErrorValidacion(format!(
                "Correlativo inválido '{}': debe estar entre 1 y 99999999.",
                boleta.correlativo
            )));
        }

        // Si el importe total excede S/ 700.00, el receptor no puede ser anónimo
        if boleta.totales.importe_total_pagar > dec!(700.00)
            && matches!(
                boleta.receptor.tipo_documento,
                CpeTipoDocumentoIdentidad::SinDocumento
            )
        {
            return Err(CpeError::ErrorValidacion(
                "Para Boletas de Venta con importe superior a S/ 700.00 es obligatorio identificar al cliente con DNI, RUC u otro documento."
                    .to_string(),
            ));
        }

        if boleta.receptor.tipo_documento == CpeTipoDocumentoIdentidad::DNI {
            Self::validar_dni(&boleta.receptor.numero_documento)?;
        }

        Self::validar_cuadre_totales(&boleta.totales)?;

        Ok(())
    }

    /// Valida integralmente una Nota de Crédito Electrónica antes de su emisión.
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorValidacion` si la estructura o referencias son inválidas.
    pub fn validar_nota_credito(nota: &CpeNotaCredito) -> CpeResult<()> {
        Self::validar_ruc(&nota.emisor.ruc)?;
        Self::validar_serie(&nota.serie, CpeTipoDocumento::NotaCredito)?;

        if nota.documento_modificado.serie_numero.trim().is_empty() {
            return Err(CpeError::ErrorValidacion(
                "La Nota de Crédito debe especificar el documento modificado (serie y número)."
                    .to_string(),
            ));
        }

        if nota.sustento_motivo.trim().is_empty() {
            return Err(CpeError::ErrorValidacion(
                "La Nota de Crédito debe indicar el sustento o motivo detallado de emisión."
                    .to_string(),
            ));
        }

        Self::validar_cuadre_totales(&nota.totales)?;

        Ok(())
    }

    /// Valida integralmente una Nota de Débito Electrónica antes de su emisión.
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorValidacion` si la estructura o referencias son inválidas.
    pub fn validar_nota_debito(nota: &CpeNotaDebito) -> CpeResult<()> {
        Self::validar_ruc(&nota.emisor.ruc)?;
        Self::validar_serie(&nota.serie, CpeTipoDocumento::NotaDebito)?;

        if nota.documento_modificado.serie_numero.trim().is_empty() {
            return Err(CpeError::ErrorValidacion(
                "La Nota de Débito debe especificar el documento modificado (serie y número)."
                    .to_string(),
            ));
        }

        if nota.sustento_motivo.trim().is_empty() {
            return Err(CpeError::ErrorValidacion(
                "La Nota de Débito debe indicar el sustento o motivo detallado de emisión."
                    .to_string(),
            ));
        }

        Self::validar_cuadre_totales(&nota.totales)?;

        Ok(())
    }
}
