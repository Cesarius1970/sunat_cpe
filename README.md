# sunat_cpe

[![Crates.io](https://img.shields.io/crates/v/sunat_cpe.svg)](https://crates.io/crates/sunat_cpe)
[![Documentation](https://docs.rs/sunat_cpe/badge.svg)](https://docs.rs/sunat_cpe)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#licencia)
[![Rust](https://img.shields.io/badge/rustc-1.85%2B-brightgreen.svg)](https://www.rust-lang.org/)

Librería en Rust de alto rendimiento para la generación, firma digital XML (XMLDSig), validación pre-vuelo y comunicación con los servicios web (SOAP / REST) de la **SUNAT** (Superintendencia Nacional de Aduanas y de Administración Tributaria del Perú) para **Comprobantes de Pago Electrónicos (CPE)**.

Para detalles completos de algoritmos y arquitectura, consulte [`docs/MANUAL_TECNICO.md`](docs/MANUAL_TECNICO.md).

---

## Características Principales

- **100% Rust Nativo**: Cero dependencias dinámicas o estáticas de OpenSSL / C. La criptografía RSA-SHA256, canonicalización C14N y el parsing PKCS#12 (`.p12` / `.pfx`) se ejecutan puramente en Rust.
- **Aritmética Decimal Exacta (Antifloating-Point)**: Prohibición estricta de punto flotante binario (`f32`/`f64`) para importes monetarios. Todos los cálculos se realizan con precisión exacta de 128 bits vía `rust_decimal::Decimal`.
- **Esquemas OASIS UBL 2.1**:
  - Factura Electrónica (`01` - `Invoice-2`)
  - Boleta de Venta Electrónica (`03` - `Invoice-2`)
  - Nota de Crédito Electrónica (`07` - `CreditNote-2`) con `cac:DiscrepancyResponse` y `cac:BillingReference`
  - Nota de Débito Electrónica (`08` - `DebitNote-2`)
- **Validador Pre-Vuelo (`CpeValidador`)**:
  - Verificación oficial de RUC mediante algoritmo de **Módulo 11** de SUNAT.
  - Validación de DNI (8 dígitos) y consistencia de series (`F`/`B`/`E`).
  - Cuadre aritmético estricto de bases imponibles, tributos, descuentos y total a pagar.
  - Validación de reglas comerciales (ej. obligatoriedad de receptor para boletas > S/ 700.00).
- **Representación Impresa y Código QR (`CpeRepresentacionImpresa`)**: Generación estandarizada de la cadena oficial de 10 campos delimitada por pipes (`|`) requerida por la R.S. N.° 097-2012/SUNAT.
- **Extracción de DigestValue (`cpe_extraer_hash_resumen`)**: Extracción directa del valor hash SHA-256 en Base64 para impresión en tickets y códigos de barras.
- **Parser de Constancias de Recepción CDR (`CpeCdr`)**: Lectura semántica basada en eventos `quick-xml`, tolerante a namespaces y con captura exhaustiva de observaciones y advertencias (`<cbc:Note>`).
- **Conectividad SOAP Dual**:
  - Variante asíncrona (`_async`) impulsada por `tokio` y `reqwest`.
  - Variante síncrona bloqueante (`_sync`) con `ureq` para servicios CLI o batch.
- **Catálogos Oficiales SUNAT**: Implementación exhaustiva de los catálogos 01, 02, 06, 07, 08, 09, 10, 17, 52 y 59 con variantes abiertas `Desconocido(String)` para compatibilidad hacia adelante.

---

## Instalación

Agregue `sunat_cpe` a su archivo `Cargo.toml`:

```toml
[dependencies]
sunat_cpe = "0.2"
rust_decimal = "1.36"
rust_decimal_macros = "1.36"
```

Por defecto, se incluye el cliente asíncrono (`async`). Si prefiere utilizar el cliente síncrono bloqueante sin runtime de Tokio:

```toml
[dependencies]
sunat_cpe = { version = "0.2", default-features = false, features = ["blocking"] }
```

---

## Guía Rápida de Uso

### 1. Construcción y Validación de una Factura Electrónica

```rust,no_run
use rust_decimal_macros::dec;
use sunat_cpe::cpe_catalogos::*;
use sunat_cpe::cpe_modelos::*;
use sunat_cpe::CpeValidador;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let factura = CpeFactura {
        serie: "F001".to_string(),
        correlativo: 1,
        fecha_emision: "2026-09-22".to_string(),
        hora_emision: Some("15:30:00".to_string()),
        fecha_vencimiento: None,
        tipo_operacion: CpeTipoOperacion::VentaInterna,
        moneda: CpeTipoMoneda::PEN,
        emisor: CpeEmisor {
            ruc: "20131312955".to_string(),
            razon_social: "EMPRESA EJEMPLO S.A.C.".to_string(),
            nombre_comercial: Some("MI EMPRESA".to_string()),
            direccion: None,
        },
        receptor: CpeReceptor {
            tipo_documento: CpeTipoDocumentoIdentidad::RUC,
            numero_documento: "20000000001".to_string(),
            razon_social: "CLIENTE VIP S.A.".to_string(),
            direccion: None,
            correo_electronico: None,
        },
        forma_pago: CpeFormaPago::Contado,
        informacion_pago: None,
        items: vec![CpeItem {
            numero_orden: 1,
            codigo_producto: Some("PROD01".to_string()),
            descripcion: "Desarrollo de Software en Rust".to_string(),
            unidad_medida: "NIU".to_string(),
            cantidad: dec!(1.0),
            valor_unitario: dec!(100.00),
            precio_unitario: dec!(118.00),
            subtotal_linea: dec!(100.00),
            total_linea: dec!(118.00),
            tipo_afectacion_igv: CpeTipoAfectacionIgv::GravadoOperacionOnerosa,
            monto_igv: dec!(18.00),
            porcentaje_igv: dec!(18.00),
            monto_isc: None,
            monto_icbper: None,
            total_impuestos_item: dec!(18.00),
        }],
        totales: CpeTotales {
            total_operaciones_gravadas: dec!(100.00),
            total_igv: dec!(18.00),
            importe_total_pagar: dec!(118.00),
            ..Default::default()
        },
        leyendas: vec![CpeLeyenda {
            codigo: CpeCodigoLeyenda::MontoEnLetras,
            descripcion: "CIENTO DIECIOCHO CON 00/100 SOLES".to_string(),
        }],
        documentos_referencia: vec![],
    };

    // Validación pre-vuelo (RUC Módulo 11, series, cuadre aritmético exacto)
    CpeValidador::validar_factura(&factura)?;
    println!("Comprobante validado correctamente.");

    Ok(())
}
```

### 2. Firma Digital y Extracción de Código QR

```rust,no_run
use sunat_cpe::{
    cpe_extraer_hash_resumen, CpeCertificadoDigital, CpeFirmador,
    CpeRepresentacionImpresa, CpeUblSerializador,
};

fn firmar_y_generar_qr(
    factura: &sunat_cpe::CpeFactura,
    bytes_p12: &[u8],
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Serializar a OASIS UBL 2.1
    let xml_ubl = factura.a_xml_ubl()?;

    // Cargar certificado PKCS#12 (.p12 / .pfx) puramente en Rust
    let cert = CpeCertificadoDigital::desde_pkcs12(bytes_p12, password)?;

    // Firmar XML canónico con XMLDSig (RSA-SHA256)
    let xml_firmado = cert.cpe_firmar_xml(&xml_ubl)?;

    // Extraer DigestValue para representación impresa
    let hash_resumen = cpe_extraer_hash_resumen(&xml_firmado)?;
    println!("Hash DigestValue: {}", hash_resumen);

    // Generar cadena oficial para Código QR (R.S. 097-2012/SUNAT)
    let cadena_qr = factura.cpe_generar_cadena_qr(&hash_resumen);
    println!("Cadena QR Oficial: {}", cadena_qr);

    Ok(())
}
```

### 3. Empaquetado y Envío Asíncrono a SUNAT

```rust,no_run
use sunat_cpe::{
    cpe_empaquetado, cpe_ws, CpeAmbiente, CpeCredencialesSol, CpeEstadoCdr,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nombre_xml = "20131312955-01-F001-00000001.xml";
    let xml_firmado = b"<Invoice>...</Invoice>";

    // Comprimir en ZIP según nomenclatura obligatoria
    let zip_bytes = cpe_empaquetado::cpe_comprimir_xml_a_zip(nombre_xml, xml_firmado)?;

    // Credenciales SOL
    let credenciales = CpeCredencialesSol {
        ruc: "20131312955".to_string(),
        usuario_sol: "MODDATOS".to_string(),
        clave_sol: "moddatos".to_string(),
    };

    // Envío SOAP sendBill a SUNAT Beta
    let cdr = cpe_ws::cpe_enviar_documento_async(
        CpeAmbiente::Beta,
        &credenciales,
        "20131312955-01-F001-00000001.zip",
        &zip_bytes,
    ).await?;

    match cdr.estado {
        CpeEstadoCdr::Aceptado => println!("¡Comprobante aceptado por SUNAT!"),
        CpeEstadoCdr::AceptadoConObservaciones(obs) => {
            println!("Aceptado con advertencias: {:?}", obs);
        }
        CpeEstadoCdr::Rechazado(err) => println!("Rechazado por SUNAT: {}", err),
    }

    Ok(())
}
```

---

## Módulos de la Librería

| Módulo | Descripción |
| :--- | :--- |
| [`cpe_catalogos`](src/cpe_catalogos/) | Catálogos normativos oficiales de SUNAT (01, 02, 06, 07, 08, 09, 10, 17, 52, 59). |
| [`cpe_modelos`](src/cpe_modelos/) | Modelos fuertemente tipados con precisión `rust_decimal` y trait `CpeRepresentacionImpresa`. |
| [`cpe_validador`](src/cpe_validador/) | Validador pre-vuelo con algoritmo de Módulo 11 para RUC, DNI, series y balance aritmético. |
| [`cpe_ubl`](src/cpe_ubl/) | Generador de XML OASIS UBL 2.1 para Facturas, Boletas, Notas de Crédito y Débito. |
| [`cpe_firma`](src/cpe_firma/) | Criptografía nativa en Rust: XMLDSig RSA-SHA256, certificados PKCS#12 y extracción de hash. |
| [`cpe_empaquetado`](src/cpe_empaquetado/) | Compresión ZIP y codificación Base64 en memoria con nomenclatura oficial de SUNAT. |
| [`cpe_ws`](src/cpe_ws/) | Clientes SOAP (`sendBill`, `sendSummary`, `getStatus`) para entornos Beta y Producción. |
| [`cpe_cdr`](src/cpe_cdr/) | Descompresión y lectura semántica basada en eventos del XML de Constancia de Recepción. |
| [`cpe_error`](src/cpe_error/) | Tipado exhaustivo de errores mediante `CpeError`. |

---

## Licencia

Este proyecto está licenciado bajo cualquiera de las siguientes licencias, a su elección:

- [Licencia Apache, Versión 2.0](LICENSE-APACHE) ([http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))
- [Licencia MIT](LICENSE-MIT) ([http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))

### Copyright

Copyright &copy; 2026 **César A Vergara Buenaventura** &lt;cesarvergarab@gmail.com&gt;.
