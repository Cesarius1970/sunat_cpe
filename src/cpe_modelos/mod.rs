//! # Módulo de Modelos de Dominio - `cpe_modelos`
//!
//! Estructuras fuertemente tipadas que representan cada uno de los Comprobantes de Pago
//! Electrónicos reconocidos por SUNAT, garantizando precisión decimal sin punto flotante.

pub mod cpe_boleta;
pub mod cpe_comun;
pub mod cpe_factura;
pub mod cpe_nota_credito;
pub mod cpe_nota_debito;
pub mod cpe_qr;
pub mod cpe_resumen_diario;

pub use cpe_boleta::CpeBoleta;
pub use cpe_comun::{
    CpeCuotaPago, CpeDireccion, CpeDocumentoReferencia, CpeEmisor, CpeFormaPago,
    CpeInformacionPago, CpeItem, CpeLeyenda, CpeReceptor, CpeTotales,
};
pub use cpe_factura::CpeFactura;
pub use cpe_nota_credito::CpeNotaCredito;
pub use cpe_nota_debito::CpeNotaDebito;
pub use cpe_qr::CpeRepresentacionImpresa;
pub use cpe_resumen_diario::{
    CpeComunicacionBaja, CpeItemComunicacionBaja, CpeItemResumenDiario, CpeResumenDiario,
};
