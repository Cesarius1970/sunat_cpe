//! # sunat_cpe
//!
//! Librería en Rust para la generación, firma digital, validación y envío de
//! Comprobantes de Pago Electrónicos (CPE) bajo la normativa técnica de la
//! **SUNAT** (Superintendencia Nacional de Aduanas y de Administración Tributaria del Perú).
//!
//! Para detalles completos de arquitectura, especificaciones de esquemas UBL y algoritmos
//! criptográficos, consulte el manual técnico en [`docs/MANUAL_TECNICO.md`](../docs/MANUAL_TECNICO.md).
//!
//! ## Flujo General del Sistema
//!
//! 1. **Modelado**: Construcción de estructuras fuertemente tipadas validadas según catálogos SUNAT.
//! 2. **Generación UBL**: Transformación a XML conforme a estándares OASIS UBL 2.0 / 2.1.
//! 3. **Firma Digital (XMLDSig)**: Canonicalización C14N y firma con clave privada RSA (SHA-256) usando certificados X.509.
//! 4. **Empaquetado ZIP**: Compresión con nomenclatura oficial `{RUC}-{TIPO}-{SERIE}-{NUMERO}.zip`.
//! 5. **Comunicación**: Envío a servicios web SUNAT (SOAP `sendBill`/`sendSummary` o API REST).
//! 6. **Respuesta (CDR)**: Descompresión y validación del XML de Constancia de Recepción.
//!
//! ## Copyright y Licencia
//!
//! Copyright &copy; 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
//!
//! Licenciado bajo la Licencia Apache, Versión 2.0 o la Licencia MIT a su elección.

/// Función temporal de verificación para pruebas iniciales del crate.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
