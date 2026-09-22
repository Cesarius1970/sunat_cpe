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
//! - [`cpe_validador`]: Validación pre-flight sintáctica, RUC (Módulo 11), DNI, series y cuadre de balances sin punto flotante.
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
pub mod cpe_validador;
pub mod cpe_ws;

// Re-exportaciones de conveniencia
pub use cpe_cdr::{CpeCdr, CpeEstadoCdr};
pub use cpe_error::{CpeError, CpeResult};
pub use cpe_firma::{CpeCertificadoDigital, CpeFirmador, cpe_extraer_hash_resumen};
pub use cpe_modelos::{
    CpeBoleta, CpeFactura, CpeNotaCredito, CpeNotaDebito, CpeRepresentacionImpresa,
    CpeResumenDiario,
};
pub use cpe_ubl::CpeUblSerializador;
pub use cpe_validador::CpeValidador;
pub use cpe_ws::{CpeAmbiente, CpeCredencialesSol};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpe_catalogos::*;
    use crate::cpe_modelos::*;
    use rsa::RsaPrivateKey;
    use rsa::rand_core::OsRng;
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
        assert_eq!(
            factura.nombre_archivo_xml(),
            "20123456789-01-F001-00000001.xml"
        );
        assert_eq!(
            factura.nombre_archivo_zip(),
            "20123456789-01-F001-00000001.zip"
        );

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
        let xml_firmado = certificado
            .cpe_firmar_xml(xml_sin_firmar)
            .expect("Debe firmar");

        assert!(xml_firmado.contains(
            "<ds:Signature xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\" Id=\"SignSUNAT\">"
        ));
        assert!(xml_firmado.contains("<ds:SignatureValue>"));
        assert!(xml_firmado.contains("<ds:X509Certificate>"));
    }

    #[test]
    fn test_empaquetado_zip_y_descompresion() {
        let nombre = "20123456789-01-F001-00000001.xml";
        let contenido = b"<Invoice>test</Invoice>";

        let zip_bytes =
            cpe_empaquetado::cpe_comprimir_xml_a_zip(nombre, contenido).expect("Comprimir a zip");
        assert!(!zip_bytes.is_empty());

        let (nombre_extraido, contenido_extraido) =
            cpe_empaquetado::cpe_descomprimir_primer_archivo(&zip_bytes).expect("Descomprimir zip");
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

    #[test]
    fn test_generacion_boleta_ubl_operacion_exportacion() {
        let boleta = CpeBoleta {
            serie: "B001".to_string(),
            correlativo: 50,
            fecha_emision: "2026-09-22".to_string(),
            hora_emision: Some("14:30:00".to_string()),
            fecha_vencimiento: None,
            tipo_operacion: CpeTipoOperacion::ExportacionServicios,
            moneda: CpeTipoMoneda::USD,
            emisor: CpeEmisor {
                ruc: "20123456789".to_string(),
                razon_social: "EMPRESA EXPORTADORA S.A.C.".to_string(),
                nombre_comercial: None,
                direccion: None,
            },
            receptor: CpeReceptor {
                tipo_documento: CpeTipoDocumentoIdentidad::Pasaporte,
                numero_documento: "A12345678".to_string(),
                razon_social: "JOHN DOE".to_string(),
                direccion: None,
                correo_electronico: None,
            },
            forma_pago: CpeFormaPago::Contado,
            informacion_pago: None,
            items: vec![],
            totales: CpeTotales::default(),
            leyendas: vec![],
            documentos_referencia: vec![],
        };

        let xml = boleta.a_xml_ubl().expect("Serializar boleta UBL");
        assert!(xml.contains("<cbc:InvoiceTypeCode listID=\"0200\">03</cbc:InvoiceTypeCode>"));
        assert!(xml.contains("<cbc:ID>B001-00000050</cbc:ID>"));
    }

    #[test]
    fn test_parser_cdr_con_observaciones() {
        let xml_cdr = r#"<?xml version="1.0" encoding="UTF-8"?>
        <ApplicationResponse xmlns="urn:oasis:names:specification:ubl:schema:xsd:ApplicationResponse-2"
            xmlns:cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
            xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2">
            <cbc:ID>R-20123456789-01-F001-00000002</cbc:ID>
            <cbc:ResponseDate>2026-09-22</cbc:ResponseDate>
            <cac:DocumentResponse>
                <cac:Response>
                    <cbc:ReferenceID>F001-00000002</cbc:ReferenceID>
                    <cbc:ResponseCode>0</cbc:ResponseCode>
                    <cbc:Description>La Factura numero F001-00000002, ha sido aceptada</cbc:Description>
                </cac:Response>
            </cac:DocumentResponse>
            <cbc:Note>4000: El comprobante fue registrado con observaciones tributarias</cbc:Note>
            <cbc:Note>4001: Verifique el catálogo de afectación IGV</cbc:Note>
        </ApplicationResponse>"#;

        let cdr = CpeCdr::desde_xml(xml_cdr).expect("Parsear CDR con observaciones");
        assert_eq!(cdr.codigo_respuesta, "0");
        assert!(cdr.es_aceptado());
        assert_eq!(cdr.observaciones.len(), 2);
        assert!(matches!(
            cdr.estado,
            CpeEstadoCdr::AceptadoConObservaciones(_)
        ));
    }

    #[test]
    fn test_generacion_nota_credito_ubl() {
        let nota_credito = CpeNotaCredito {
            serie: "FC01".to_string(),
            correlativo: 12,
            fecha_emision: "2026-09-22".to_string(),
            hora_emision: Some("15:00:00".to_string()),
            moneda: CpeTipoMoneda::PEN,
            tipo_nota_credito: CpeTipoNotaCredito::AnulacionOperacion,
            sustento_motivo: "ERROR EN RUC DEL CLIENTE".to_string(),
            documento_modificado: CpeDocumentoReferencia {
                serie_numero: "F001-00000001".to_string(),
                tipo_documento: CpeTipoDocumento::Factura,
                fecha_emision: Some("2026-09-10".to_string()),
            },
            emisor: CpeEmisor {
                ruc: "20123456789".to_string(),
                razon_social: "EMPRESA DE PRUEBA S.A.C.".to_string(),
                nombre_comercial: None,
                direccion: None,
            },
            receptor: CpeReceptor {
                tipo_documento: CpeTipoDocumentoIdentidad::RUC,
                numero_documento: "20987654321".to_string(),
                razon_social: "CLIENTE VIP S.A.".to_string(),
                direccion: None,
                correo_electronico: None,
            },
            items: vec![],
            totales: CpeTotales::default(),
            leyendas: vec![],
        };

        let xml = nota_credito
            .a_xml_ubl()
            .expect("Serializar nota de crédito");
        assert!(xml.contains(
            "<CreditNote xmlns=\"urn:oasis:names:specification:ubl:schema:xsd:CreditNote-2\""
        ));
        assert!(xml.contains("<cbc:ID>FC01-00000012</cbc:ID>"));
        assert!(xml.contains("<cbc:ResponseCode>01</cbc:ResponseCode>"));
        assert!(xml.contains("<cbc:ReferenceID>F001-00000001</cbc:ReferenceID>"));
        assert!(xml.contains("<cbc:DocumentTypeCode>01</cbc:DocumentTypeCode>"));
    }

    #[test]
    fn test_generacion_nota_debito_ubl() {
        let nota_debito = CpeNotaDebito {
            serie: "FD01".to_string(),
            correlativo: 5,
            fecha_emision: "2026-09-22".to_string(),
            hora_emision: Some("15:10:00".to_string()),
            moneda: CpeTipoMoneda::PEN,
            tipo_nota_debito: CpeTipoNotaDebito::InteresesPorMora,
            sustento_motivo: "INTERESES POR PAGO FUERA DE PLAZO".to_string(),
            documento_modificado: CpeDocumentoReferencia {
                serie_numero: "F001-00000001".to_string(),
                tipo_documento: CpeTipoDocumento::Factura,
                fecha_emision: Some("2026-09-10".to_string()),
            },
            emisor: CpeEmisor {
                ruc: "20123456789".to_string(),
                razon_social: "EMPRESA DE PRUEBA S.A.C.".to_string(),
                nombre_comercial: None,
                direccion: None,
            },
            receptor: CpeReceptor {
                tipo_documento: CpeTipoDocumentoIdentidad::RUC,
                numero_documento: "20987654321".to_string(),
                razon_social: "CLIENTE VIP S.A.".to_string(),
                direccion: None,
                correo_electronico: None,
            },
            items: vec![],
            totales: CpeTotales::default(),
            leyendas: vec![],
        };

        let xml = nota_debito.a_xml_ubl().expect("Serializar nota de débito");
        assert!(xml.contains(
            "<DebitNote xmlns=\"urn:oasis:names:specification:ubl:schema:xsd:DebitNote-2\""
        ));
        assert!(xml.contains("<cbc:ID>FD01-00000005</cbc:ID>"));
        assert!(xml.contains("<cbc:ResponseCode>01</cbc:ResponseCode>"));
        assert!(xml.contains("<cbc:ReferenceID>F001-00000001</cbc:ReferenceID>"));
        assert!(xml.contains("<cac:RequestedMonetaryTotal>"));
    }

    #[test]
    fn test_validador_ruc_y_dni() {
        // RUCs válidos oficiales (Módulo 11)
        assert!(CpeValidador::validar_ruc("20131312955").is_ok()); // SUNAT
        assert!(CpeValidador::validar_ruc("20000000001").is_ok());

        // RUC con dígito verificador erróneo
        assert!(CpeValidador::validar_ruc("20131312954").is_err());
        // RUC con longitud incorrecta
        assert!(CpeValidador::validar_ruc("2013131295").is_err());
        // RUC con prefijo no tributario
        assert!(CpeValidador::validar_ruc("30131312955").is_err());

        // DNIs
        assert!(CpeValidador::validar_dni("44556677").is_ok());
        assert!(CpeValidador::validar_dni("4455667").is_err());
        assert!(CpeValidador::validar_dni("4455667A").is_err());
    }

    #[test]
    fn test_validador_cuadre_factura() {
        let mut totales = CpeTotales::default();
        totales.total_operaciones_gravadas = dec!(100.00);
        totales.total_igv = dec!(18.00);
        totales.importe_total_pagar = dec!(118.00);

        assert!(CpeValidador::validar_cuadre_totales(&totales).is_ok());

        // Descuadre artificial
        totales.importe_total_pagar = dec!(125.00);
        assert!(CpeValidador::validar_cuadre_totales(&totales).is_err());
    }

    #[test]
    fn test_representacion_impresa_qr() {
        let factura = CpeFactura {
            serie: "F001".to_string(),
            correlativo: 42,
            fecha_emision: "2026-09-22".to_string(),
            hora_emision: None,
            moneda: CpeTipoMoneda::PEN,
            tipo_operacion: CpeTipoOperacion::VentaInterna,
            fecha_vencimiento: None,
            emisor: CpeEmisor {
                ruc: "20131312955".to_string(),
                razon_social: "EMPRESA PRUEBA S.A.".to_string(),
                nombre_comercial: None,
                direccion: None,
            },
            receptor: CpeReceptor {
                tipo_documento: CpeTipoDocumentoIdentidad::RUC,
                numero_documento: "20000000001".to_string(),
                razon_social: "CLIENTE S.A.".to_string(),
                direccion: None,
                correo_electronico: None,
            },
            items: vec![],
            totales: CpeTotales {
                total_operaciones_gravadas: dec!(100.00),
                total_igv: dec!(18.00),
                importe_total_pagar: dec!(118.00),
                ..Default::default()
            },
            forma_pago: CpeFormaPago::Contado,
            informacion_pago: None,
            leyendas: vec![],
            documentos_referencia: vec![],
        };

        let hash_ejemplo = "aBcDeFgHiJkLmNoPqRsTuVwXyZ0=";
        let qr = factura.cpe_generar_cadena_qr(hash_ejemplo);

        assert_eq!(
            qr,
            "20131312955|01|F001|42|18.00|118.00|2026-09-22|6|20000000001|aBcDeFgHiJkLmNoPqRsTuVwXyZ0=|"
        );
        // Validar que tenga exactamente 10 delimitadores '|'
        assert_eq!(qr.matches('|').count(), 10);
    }

    #[test]
    fn test_extraccion_hash_resumen() {
        let xml_firmado_ejemplo = r#"<Invoice xmlns="urn:oasis:names:specification:ubl:schema:xsd:Invoice-2">
            <ext:UBLExtensions>
                <ext:UBLExtension>
                    <ext:ExtensionContent>
                        <ds:Signature xmlns:ds="http://www.w3.org/2000/09/xmldsig#">
                            <SignedInfo>
                                <Reference URI="">
                                    <DigestMethod Algorithm="http://www.w3.org/2001/04/xmlenc#sha256"/>
                                    <DigestValue>+8yABC123DigestValuePrueba==</DigestValue>
                                </Reference>
                            </SignedInfo>
                        </ds:Signature>
                    </ext:ExtensionContent>
                </ext:UBLExtension>
            </ext:UBLExtensions>
        </Invoice>"#;

        let hash = cpe_extraer_hash_resumen(xml_firmado_ejemplo).expect("Extraer DigestValue");
        assert_eq!(hash, "+8yABC123DigestValuePrueba==");
    }
}
