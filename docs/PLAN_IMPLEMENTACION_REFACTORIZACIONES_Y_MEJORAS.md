# Plan de Implementación de Refactorizaciones y Nuevas Funcionalidades para `sunat_cpe`

**Documento Técnico de Planificación**  
**Versión:** 1.0.0  
**Fecha:** 2026-09-22  
**Autor:** César A Vergara Buenaventura <cesarvergarab@gmail.com>  
**Estado:** Pendiente de Aprobación por el Usuario  

---

## 1. Introducción y Objetivos

El presente documento detalla la arquitectura técnica, las estructuras de datos, las firmas de funciones, los algoritmos y la estrategia de pruebas para la ejecución de los **tres pasos prioritarios de refactorización y extensión** de la librería `sunat_cpe`:

1. **Paso 1: Robustecimiento del Motor UBL y Parser de Respuestas** (Corrección de fragilidades en Boletas de Venta, escape seguro de entidades XML y parser semántico de CDR con `quick-xml`).
2. **Paso 2: Serialización UBL 2.1 para Notas de Crédito y Notas de Débito** (Implementación de `CpeUblSerializador` para `CpeNotaCredito` y `CpeNotaDebito` conforme a los estándares OASIS y Guías SUNAT).
3. **Paso 3: Utilidades Comerciales, Validador Pre-Vuelo y Representación Impresa** (Generación de cadena para Código QR, extractor de Hash DigestValue, validador de integridad tributaria y carga de certificados PKCS#12/PFX).

> [!IMPORTANT]
> **Regla de Ejecución:** Este documento es exclusivamente de diseño y especificación. **No se modificará ni creará ningún archivo de código de la librería** hasta recibir la aprobación explícita del usuario.

---

## 2. Paso 1: Robustecimiento del Motor UBL y Parser de Respuestas

### 2.1. Problemas a Resolver
1. **Fragilidad en Boletas (`src/cpe_ubl/mod.rs`):** La implementación actual de `CpeBoleta::a_xml_ubl` clona los campos a una `CpeFactura`, genera el XML y realiza un `replacen` de `<cbc:InvoiceTypeCode listID="0101">01</cbc:InvoiceTypeCode>` por el código `03`. Si el comprobante posee un tipo de operación distinto a `0101` (por ejemplo, `0200` Exportación o `0102` Anticipos), el reemplazo falla y se genera un comprobante con código `01` (Factura) pero con serie `B001`, lo que provoca el **rechazo fulminante de SUNAT (Error 1033)**.
2. **Escape Inseguro de XML:** Las cadenas ingresadas por los usuarios (descripciones de ítems, direcciones, nombres comerciales) no pasan por un escape de entidades XML (`&`, `<`, `>`, `"`, `'`), arriesgando la generación de XMLs malformados.
3. **Parser Frágil de CDR y SOAP:** Las funciones `extraer_etiqueta` en `cpe_cdr` y `cpe_ws` utilizan `str::find` con cadenas literales como `<cbc:ResponseCode>`. Espacios adicionales, atributos no esperados o prefijos alternativos (`<ResponseCode>`) provocan lecturas vacías o estados "DESCONOCIDO". Además, no se extraen las múltiples advertencias u observaciones `<cbc:Note>`.

### 2.2. Diseño de la Refactorización

#### 2.2.1. Motor Interno Parametrizado de Facturas y Boletas (`cpe_ubl`)
Se reemplaza la duplicación y el truco de reemplazo por una función interna privada que construye el documento XML recibiendo los parámetros diferenciales:

```rust
// src/cpe_ubl/mod.rs

/// Parámetros de emisión comunes entre Factura (01) y Boleta (03).
struct CpeInvoiceParams<'a> {
    tipo_documento_codigo: &'a str, // "01" o "03"
    serie: &'a str,
    correlativo: u32,
    fecha_emision: &'a str,
    hora_emision: Option<&'a str>,
    fecha_vencimiento: Option<&'a str>,
    tipo_operacion: &'a CpeTipoOperacion,
    moneda: &'a CpeTipoMoneda,
    emisor: &'a CpeEmisor,
    receptor: &'a CpeReceptor,
    forma_pago: &'a CpeFormaPago,
    informacion_pago: Option<&'a CpeInformacionPago>,
    items: &'a [CpeItem],
    totales: &'a CpeTotales,
    leyendas: &'a [CpeLeyenda],
    documentos_referencia: &'a [CpeDocumentoReferencia],
}

fn cpe_construir_invoice_ubl_interno(params: CpeInvoiceParams) -> CpeResult<String> {
    // Construcción directa interpolando params.tipo_documento_codigo en:
    // <cbc:InvoiceTypeCode listID="{}">{}</cbc:InvoiceTypeCode>
}
```

Tanto `CpeFactura::a_xml_ubl` como `CpeBoleta::a_xml_ubl` delegarán directamente en esta función pasando `"01"` y `"03"` respectivamente, eliminando por completo cualquier manipulación posterior de cadenas.

#### 2.2.2. Escapado Seguro de Caracteres XML
Se implementará una función auxiliar de escape rápido:

```rust
// src/cpe_ubl/mod.rs o src/cpe_ubl/cpe_escape.rs
fn cpe_xml_escape(texto: &str) -> String {
    let mut escaped = String::with_capacity(texto.len());
    for c in texto.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(c),
        }
    }
    escaped
}
```
Todos los nodos de texto que no requieran `<![CDATA[...]]>` emplearán `cpe_xml_escape`.

#### 2.2.3. Parser de CDR Basado en Eventos (`quick-xml`)
Se refactorizará `CpeCdr::desde_xml` en `src/cpe_cdr/mod.rs` para utilizar `quick_xml::reader::Reader`:

```rust
// src/cpe_cdr/mod.rs
impl CpeCdr {
    pub fn desde_xml(xml: &str) -> CpeResult<Self> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut buf = Vec::new();
        let mut id = String::new();
        let mut codigo_respuesta = String::new();
        let mut descripcion_respuesta = String::new();
        let mut fecha_respuesta = String::new();
        let mut hora_respuesta = None;
        let mut ruc_emisor = String::new();
        let mut documento_referenciado = String::new();
        let mut hash_comprobante = None;
        let mut observaciones = Vec::new();

        // Bucle de lectura de eventos XML (Start, Text, End)
        // Coincidencia insensible al prefijo del namespace (ej: 'cbc:ResponseCode' o 'ResponseCode')
        // Si la etiqueta local es "Note", se almacena en observaciones.push(texto)
        // ...
    }
}
```

---

## 3. Paso 2: Serialización UBL 2.1 para Notas de Crédito y Débito

### 3.1. Requisitos Normativos de SUNAT
- **Nota de Crédito (Tipo 07):** Estándar OASIS UBL 2.1 `CreditNote-2`. Debe incluir obligatoriamente:
  - `<cbc:CreditNoteTypeCode listID="...">07</cbc:CreditNoteTypeCode>`
  - `<cac:DiscrepancyResponse>`:
    - `<cbc:ReferenceID>`: Serie y correlativo del comprobante que se modifica (ej. `F001-00000001`).
    - `<cbc:ResponseCode>`: Código del motivo de emisión según Catálogo 09 (ej. `01` para Anulación de la operación).
    - `<cbc:Description>`: Sustento o motivo detallado.
  - `<cac:BillingReference>`: Referencia al documento electrónico original (RUC, tipo de comprobante, serie y correlativo).
  - `<cac:CreditNoteLine>`: Líneas de la nota con cantidades, montos, tributos (IGV/ISC) y valores de venta.
- **Nota de Débito (Tipo 08):** Estándar OASIS UBL 2.1 `DebitNote-2`. Debe incluir:
  - `<cbc:RequestedMonetaryTotal>` en lugar de `LegalMonetaryTotal`.
  - `<cac:DiscrepancyResponse>` con códigos del Catálogo 10 (ej. `01` Intereses por mora, `02` Aumento en el valor).
  - `<cac:DebitNoteLine>`.

### 3.2. Implementación Técnica en `src/cpe_ubl/`

Se crearán submódulos especializados o se expandirá `src/cpe_ubl/mod.rs` mediante:
1. `impl CpeUblSerializador for CpeNotaCredito`
2. `impl CpeUblSerializador for CpeNotaDebito`

#### Estructura del XML UBL para Nota de Crédito:
```xml
<?xml version="1.0" encoding="ISO-8859-1" standalone="no"?>
<CreditNote xmlns="urn:oasis:names:specification:ubl:schema:xsd:CreditNote-2"
    xmlns:cac="urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
    xmlns:cbc="urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2"
    xmlns:ds="http://www.w3.org/2000/09/xmldsig#"
    xmlns:ext="urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2">
    <ext:UBLExtensions>
        <ext:UBLExtension>
            <ext:ExtensionContent/>
        </ext:UBLExtension>
    </ext:UBLExtensions>
    <cbc:UBLVersionID>2.1</cbc:UBLVersionID>
    <cbc:CustomizationID>2.0</cbc:CustomizationID>
    <cbc:ID>{SERIE}-{CORRELATIVO:08}</cbc:ID>
    <cbc:IssueDate>{FECHA_EMISION}</cbc:IssueDate>
    <cac:DiscrepancyResponse>
        <cbc:ReferenceID>{DOC_MODIFICADO_SERIE_NUM}</cbc:ReferenceID>
        <cbc:ResponseCode>{TIPO_NOTA_CREDITO_CODIGO}</cbc:ResponseCode>
        <cbc:Description><![CDATA[{SUSTENTO_MOTIVO}]]></cbc:Description>
    </cac:DiscrepancyResponse>
    <cac:BillingReference>
        <cac:InvoiceDocumentReference>
            <cbc:ID>{DOC_MODIFICADO_SERIE_NUM}</cbc:ID>
            <cbc:DocumentTypeCode>{DOC_MODIFICADO_TIPO_CODIGO}</cbc:DocumentTypeCode>
        </cac:InvoiceDocumentReference>
    </cac:BillingReference>
    <!-- AccountingSupplierParty (Emisor) -->
    <!-- AccountingCustomerParty (Receptor) -->
    <!-- TaxTotal -->
    <!-- LegalMonetaryTotal -->
    <!-- CreditNoteLine(s) -->
</CreditNote>
```

#### Estructura del XML UBL para Nota de Débito:
Similar al esquema anterior pero utilizando el elemento raíz `<DebitNote>` y `<cac:RequestedMonetaryTotal>`.

---

## 4. Paso 3: Utilidades Comerciales, Validador Pre-Vuelo y Representación Impresa

### 4.1. Generación de Cadena para Código QR y Extracción de Hash

#### 4.1.1. Estándar de la Cadena QR (R.S. 097-2012/SUNAT)
La cadena requerida para la generación del código QR en los comprobantes físicos (PDF o ticket) consta de 10 campos separados por plecas (`|`):
```text
RUC | Tipo CPE | Serie | Correlativo | Monto IGV | Importe Total | Fecha Emisión | Tipo Doc Receptor | Num Doc Receptor | Código Hash |
```

#### 4.1.2. Trait de Utilidades `CpeRepresentacionImpresa`
Se definirá un nuevo trait para comprobantes:
```rust
// src/cpe_modelos/cpe_qr.rs
pub trait CpeRepresentacionImpresa {
    /// Genera la cadena de texto oficial que debe codificarse dentro del Código QR.
    fn cpe_generar_cadena_qr(&self, codigo_hash: &str) -> String;
}
```

Implementado para `CpeFactura`, `CpeBoleta`, `CpeNotaCredito` y `CpeNotaDebito`.

#### 4.1.3. Extracción de Hash de la Firma Digital
En `src/cpe_firma/mod.rs` se expondrá una función para obtener el DigestValue del comprobante firmado:
```rust
// src/cpe_firma/mod.rs
pub fn cpe_extraer_hash_resumen(xml_firmado: &str) -> CpeResult<String> {
    // Parsea el XML y localiza <ds:DigestValue>...</ds:DigestValue>
    // Retorna el hash SHA-256 en Base64
}
```

### 4.2. Módulo Validador Pre-Vuelo (`cpe_validador`)

Previene llamadas infructuosas y multas por comprobantes mal formados antes del envío a los servidores de SUNAT.

#### 4.2.1. Validaciones Incluidas:
1. **Validación de RUC (Algoritmo Módulo 11):**
   - Verifica 11 dígitos numéricos.
   - Prefijos válidos: `10`, `15`, `17`, `20`.
   - Factores de ponderación: `[5, 4, 3, 2, 7, 6, 5, 4, 3, 2]`.
   - Cálculo del residuo y comprobación del 11º dígito.
2. **Validación de Series:**
   - Facturas: Serie de 4 caracteres, iniciando con letra `F` o `E`.
   - Boletas: Serie de 4 caracteres, iniciando con letra `B` o `E`.
   - Notas de Crédito/Débito: Deben ser coherentes con el documento afectado (`F` para modificar facturas, `B` para modificar boletas).
3. **Cuadre Aritmético Exacto:**
   - Emplea `rust_decimal::Decimal`.
   - Verifica que:
     $$\text{Operaciones Gravadas} + \text{Inafectas} + \text{Exoneradas} + \text{Exportación} + \text{IGV} + \text{ISC} - \text{Descuentos} = \text{Importe Total a Pagar}$$
   - Tolerancia de redondeo: $\pm 0.01$ (un céntimo por dispersión de redondeos de ítems).

```rust
// src/cpe_validador/mod.rs
pub struct CpeValidador;

impl CpeValidador {
    pub fn validar_ruc(ruc: &str) -> CpeResult<()>;
    pub fn validar_factura(factura: &CpeFactura) -> CpeResult<()>;
    pub fn validar_boleta(boleta: &CpeBoleta) -> CpeResult<()>;
    pub fn validar_nota_credito(nota: &CpeNotaCredito) -> CpeResult<()>;
}
```

### 4.3. Carga Directa de Certificados PKCS#12 / PFX

Facilita el uso de certificados digitales emitidos comercialmente en Perú:
```rust
// src/cpe_firma/mod.rs
impl CpeCertificadoDigital {
    /// Carga una clave privada RSA y el certificado público desde un archivo contenedor PKCS#12 (.p12 o .pfx).
    pub fn desde_pkcs12(bytes_p12: &[u8], password: &str) -> CpeResult<Self> {
        // Deserialización y decodificación PKCS#12 con dependencias puras en Rust.
    }
}
```

---

## 5. Matriz de Impacto en Archivos Existentes

| Archivo | Tipo de Cambio | Impacto |
|---|---|---|
| `src/cpe_ubl/mod.rs` | Refactorización y Adición | Generación unificada de Invoice/Boleta sin `replacen`, implementación de UBL para Notas de Crédito y Débito, escape seguro. |
| `src/cpe_cdr/mod.rs` | Refactorización | Parser robusto basado en eventos `quick-xml` y soporte de observaciones. |
| `src/cpe_firma/mod.rs` | Adición | Función `cpe_extraer_hash_resumen` y carga PKCS#12. |
| `src/cpe_modelos/` | Adición | Implementación de `CpeRepresentacionImpresa` (cadena QR). |
| `src/cpe_validador/` (nuevo) | Nuevo Módulo | Validaciones de negocio pre-vuelo (RUC Módulo 11, series, cuadre de totales). |
| `src/lib.rs` | Exportación | Re-exportación de las nuevas estructuras, traits y validadores. |
| `docs/MANUAL_TECNICO.md` | Documentación | Actualización de los capítulos UBL, Notas y Validaciones. |

---

## 6. Plan de Pruebas y Criterios de Aceptación

Para asegurar cero regresiones y máxima fidelidad:

1. **Pruebas de Boleta con distintas operaciones:**
   - Test con `CpeTipoOperacion::VentaInterna` (`0101`) -> Debe contener `<cbc:InvoiceTypeCode listID="0101">03</cbc:InvoiceTypeCode>`.
   - Test con `CpeTipoOperacion::Exportacion` (`0200`) -> Debe contener `<cbc:InvoiceTypeCode listID="0200">03</cbc:InvoiceTypeCode>`.
2. **Pruebas de Nota de Crédito UBL:**
   - Test completo de generación de XML para `CpeNotaCredito` anulando una factura previa, verificando `<cac:DiscrepancyResponse>` y `<cac:BillingReference>`.
3. **Pruebas de Nota de Débito UBL:**
   - Test completo para `CpeNotaDebito` por penalidades o intereses.
4. **Pruebas de Validación Pre-Vuelo:**
   - Test de RUCs válidos e inválidos con el algoritmo Módulo 11.
   - Test de facturas descuadradas aritméticamente -> Debe retornar `CpeError::ErrorValidacion`.
5. **Pruebas de Cadena QR:**
   - Test de coincidencia exacta con el formato de 10 campos de la R.S. 097-2012/SUNAT.
6. **Integridad:**
   - `cargo test` pasando al 100%.
   - `cargo clippy -- -D warnings` pasando sin advertencias.
