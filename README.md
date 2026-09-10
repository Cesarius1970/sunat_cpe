# sunat_cpe

[![Crates.io](https://img.shields.io/badge/crates.io-v0.1.0-orange.svg)](https://crates.io/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#licencia)

Librería en Rust de alto rendimiento para la generación, firma digital XML (XMLDSig), validación y comunicación con los servicios web (SOAP / REST) de la **SUNAT** (Superintendencia Nacional de Aduanas y de Administración Tributaria del Perú) para Comprobantes de Pago Electrónicos (CPE).

## Características Principales (En Desarrollo)

- **Estándares UBL**: Soporte para UBL 2.0 y UBL 2.1 según la normativa técnica de SUNAT.
- **Tipos de Comprobantes**: Factura, Boleta de Venta, Notas de Crédito, Notas de Débito, Guías de Remisión y Resúmenes Diarios / Bajas.
- **Firma Digital**: Firma XMLDSig mediante certificados digitales (`.pfx` / `.p12` o claves PEM).
- **Rendimiento y Seguridad**: Tipado estricto en Rust, validaciones en tiempo de compilación y cero costo en abstracciones críticas.

## Licencia

Este proyecto está licenciado bajo cualquiera de las siguientes licencias, a su elección (estándar oficial del ecosistema de Rust):

* [Licencia Apache, Versión 2.0](LICENSE-APACHE) ([http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
* [Licencia MIT](LICENSE-MIT) ([http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

### Copyright

Copyright &copy; 2026 **César A Vergara Buenaventura** &lt;cesarvergarab@gmail.com&gt;.
