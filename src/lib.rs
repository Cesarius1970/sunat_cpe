//! # sunat_cpe
//!
//! Librería en Rust para la generación, firma digital, validación y envío de
//! Comprobantes de Pago Electrónicos (CPE) bajo la normativa de la SUNAT (Perú).
//!
//! ## Copyright y Licencia
//!
//! Copyright &copy; 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
//!
//! Licenciado bajo la Licencia Apache, Versión 2.0 o la Licencia MIT a su elección.

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
