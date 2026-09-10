//! # sunat_cpe
//!
//! Librería en Rust de alto rendimiento para la generación, firma digital,
//! validación y comunicación con los servicios web de la **SUNAT** (Perú)
//! para **Comprobantes de Pago Electrónicos (CPE)**.
//!
//! Para detalles completos de arquitectura y algoritmos consulte [`docs/MANUAL_TECNICO.md`](../docs/MANUAL_TECNICO.md).
//!
//! ## Módulos Principales
//!
//! - [`cpe_catalogos`]: Tablas y catálogos normativos oficiales de SUNAT (01, 02, 06, 07, 08, 09, 10, 17, 52, 59).
//! - [`cpe_modelos`]: Representación tipada de Facturas, Boletas, Notas y Resúmenes con precisión `rust_decimal::Decimal`.
//! - [`cpe_ubl`]: Generador de documentos XML compatibles con OASIS UBL 2.1 y UBL 2.0 vía [`cpe_ubl::CpeUblSerializador`].
//! - [`cpe_firma`]: Motor criptográfico 100% Rust nativo para firma digital XMLDSig (RSA-SHA256).
//! - [`cpe_empaquetado`]: Compresión ZIP con la nomenclatura exigida por SUNAT y conversiones Base64.
//! - [`cpe_ws`]: Conexión con servicios SOAP (`billService`) para ambientes Beta y Producción con variantes `_async` y `_sync`.
//! - [`cpe_cdr`]: Procesamiento, lectura y validación de Constancias de Recepción (CDR).
//! - [`cpe_error`]: Tipado exhaustivo de errores mediante [`cpe_error::CpeError`].
//!
//! ## Copyright y Licencia
//!
//! Copyright &copy; 2026 César A Vergara Buenaventura <cesarvergarab@gmail.com>
//!
//! Licenciado bajo la Licencia Apache, Versión 2.0 o la Licencia MIT a su elección.

pub mod cpe_catalogos;
pub mod cpe_cdr;
pub mod cpe_empaquetado;
pub mod cpe_error;
pub mod cpe_firma;
pub mod cpe_modelos;
pub mod cpe_ubl;
pub mod cpe_ws;

// Re-exportaciones de conveniencia
pub use cpe_cdr::{CpeCdr, CpeEstadoCdr};
pub use cpe_error::{CpeError, CpeResult};
pub use cpe_firma::{CpeCertificadoDigital, CpeFirmador};
pub use cpe_modelos::{CpeBoleta, CpeFactura, CpeNotaCredito, CpeNotaDebito, CpeResumenDiario};
pub use cpe_ubl::CpeUblSerializador;
pub use cpe_ws::{CpeAmbiente, CpeCredencialesSol};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpe_catalogos::*;
    use crate::cpe_modelos::*;
    use rsa::rand_core::OsRng;
    use rsa::RsaPrivateKey;
    use rust_decimal_macros::dec;

    #[test]
    fn test_generacion_factura_ubl() {
        let emisor = CpeEmisor {
            ruc: "20123456789".to_string(),
            razon_social: "EMPRESA DE PRUEBA S.A.C.".to_string(),
            nombre_comercial: Some("MI NEGOCIO".to_string()),
            direccion: Some(CpeDireccion {
                ubigeo: Some("150101".to_string()),
                departamento: Some("LIMA".to_string()),
                provincia: Some("LIMA".to_string()),
                distrito: Some("LIMA".to_string()),
                direccion_detallada: "AV. PRINCIPAL 123".to_string(),
                codigo_pais: "PE".to_string(),
            }),
        };

        let receptor = CpeReceptor {
            tipo_documento: CpeTipoDocumentoIdentidad::RUC,
            numero_documento: "20987654321".to_string(),
            razon_social: "CLIENTE VIP S.A.".to_string(),
            direccion: None,
            correo_electronico: Some("contacto@clientevip.com".to_string()),
        };

        let item = CpeItem {
            numero_orden: 1,
            codigo_producto: Some("PROD001".to_string()),
            codigo_producto_sunat: None,
            unidad_medida: "NIU".to_string(),
            cantidad: dec!(2.0),
            descripcion: "DESARROLLO DE SOFTWARE EN RUST".to_string(),
            valor_unitario: dec!(100.00),
            precio_unitario: dec!(118.00),
            tipo_afectacion_igv: CpeTipoAfectacionIgv::GravadoOperacionOnerosa,
            porcentaje_igv: dec!(18.00),
            monto_igv: dec!(36.00),
            base_imponible_igv: dec!(200.00),
            total_valor_venta: dec!(200.00),
            monto_descuento: None,
        };

        let factura = CpeFactura {
            serie: "F001".to_string(),
            correlativo: 1,
            fecha_emision: "2026-09-10".to_string(),
            hora_emision: Some("12:00:00".to_string()),
            fecha_vencimiento: None,
            tipo_operacion: CpeTipoOperacion::VentaInterna,
            moneda: CpeTipoMoneda::PEN,
            emisor,
            receptor,
            forma_pago: CpeFormaPago::Contado,
            informacion_pago: None,
            items: vec![item],
            totales: CpeTotales {
                total_operaciones_gravadas: dec!(200.00),
                total_igv: dec!(36.00),
                importe_total_pagar: dec!(236.00),
                ..Default::default()
            },
            leyendas: vec![CpeLeyenda {
                codigo: CpeCodigoLeyenda::MontoEnLetras,
                texto: "DOSCIENTOS TREINTA Y SEIS CON 00/100 SOLES".to_string(),
            }],
            documentos_referencia: vec![],
        };

        assert_eq!(factura.nombre_base(), "20123456789-01-F001-00000001");
        assert_eq!(factura.nombre_archivo_xml(), "20123456789-01-F001-00000001.xml");
        assert_eq!(factura.nombre_archivo_zip(), "20123456789-01-F001-00000001.zip");

        let xml = factura.a_xml_ubl().expect("Debe serializar XML UBL");
        assert!(xml.contains("<cbc:ID>F001-00000001</cbc:ID>"));
        assert!(xml.contains("<cbc:InvoiceTypeCode listID=\"0101\">01</cbc:InvoiceTypeCode>"));
        assert!(xml.contains("<cbc:PayableAmount currencyID=\"PEN\">236.00</cbc:PayableAmount>"));
    }

    #[test]
    fn test_firma_digital_xmldsig_nativa() {
        let mut rng = OsRng;
        let clave_privada = RsaPrivateKey::new(&mut rng, 2048).expect("Generar clave RSA");
        let certificado = CpeCertificadoDigital::nuevo(
            clave_privada,
            "MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAz...".to_string(),
        );

        let xml_sin_firmar = r#"<Invoice><ext:UBLExtensions><ext:UBLExtension><ext:ExtensionContent/></ext:UBLExtension></ext:UBLExtensions><cbc:ID>F001-1</cbc:ID></Invoice>"#;
        let xml_firmado = certificado.cpe_firmar_xml(xml_sin_firmar).expect("Debe firmar");

        assert!(xml_firmado.contains("<ds:Signature xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\" Id=\"SignSUNAT\">"));
        assert!(xml_firmado.contains("<ds:SignatureValue>"));
        assert!(xml_firmado.contains("<ds:X509Certificate>"));
    }

    #[test]
    fn test_empaquetado_zip_y_descompresion() {
        let nombre = "20123456789-01-F001-00000001.xml";
        let contenido = b"<Invoice>test</Invoice>";

        let zip_bytes = cpe_empaquetado::cpe_comprimir_xml_a_zip(nombre, contenido)
            .expect("Comprimir a zip");
        assert!(!zip_bytes.is_empty());

        let (nombre_extraido, contenido_extraido) =
            cpe_empaquetado::cpe_descomprimir_primer_archivo(&zip_bytes)
                .expect("Descomprimir zip");
        assert_eq!(nombre_extraido, nombre);
        assert_eq!(contenido_extraido, contenido);
    }

    #[test]
    fn test_parser_cdr_aceptado() {
        let xml_cdr = r#"<?xml version="1.0" encoding="UTF-8"?>
        <ApplicationResponse xmlns="urn:oasis:names:specification:ubl:schema:xsd:ApplicationResponse-2"
            xmlns:cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
            xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2">
            <cbc:ID>R-20123456789-01-F001-00000001</cbc:ID>
            <cbc:ResponseDate>2026-09-10</cbc:ResponseDate>
            <cbc:ResponseTime>12:05:00</cbc:ResponseTime>
            <cac:DocumentResponse>
                <cac:Response>
                    <cbc:ReferenceID>F001-00000001</cbc:ReferenceID>
                    <cbc:ResponseCode>0</cbc:ResponseCode>
                    <cbc:Description>La Factura numero F001-00000001, ha sido aceptada</cbc:Description>
                </cac:Response>
            </cac:DocumentResponse>
        </ApplicationResponse>"#;

        let cdr = CpeCdr::desde_xml(xml_cdr).expect("Parsear CDR");
        assert_eq!(cdr.codigo_respuesta, "0");
        assert_eq!(cdr.estado, CpeEstadoCdr::Aceptado);
        assert!(cdr.es_aceptado());
    }
}
