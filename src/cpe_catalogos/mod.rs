//! # Módulo de Catálogos Oficiales de SUNAT - `cpe_catalogos`
//!
//! Implementación fuertemente tipada de las tablas y catálogos normativos publicados
//! por la SUNAT en el portal oficial de Comprobantes de Pago Electrónicos (CPE).
//!
//! Cada catálogo incluye:
//! - Conversión bidireccional entre códigos oficiales (`&str`) y tipos Rust.
//! - Descripción oficial del concepto tributario.
//! - Variante `Desconocido(String)` / `Otro(String)` para compatibilidad hacia adelante ante nuevas resoluciones.

pub mod catalogo_01;
pub mod catalogo_02;
pub mod catalogo_06;
pub mod catalogo_07;
pub mod catalogo_08;
pub mod catalogo_09;
pub mod catalogo_10;
pub mod catalogo_17;
pub mod catalogo_52;
pub mod catalogo_59;

pub use catalogo_01::CpeTipoDocumento;
pub use catalogo_02::CpeTipoMoneda;
pub use catalogo_06::CpeTipoDocumentoIdentidad;
pub use catalogo_07::CpeTipoAfectacionIgv;
pub use catalogo_08::CpeTipoSistemaIsc;
pub use catalogo_09::CpeTipoNotaCredito;
pub use catalogo_10::CpeTipoNotaDebito;
pub use catalogo_17::CpeTipoOperacion;
pub use catalogo_52::CpeCodigoLeyenda;
pub use catalogo_59::CpeMedioPago;
