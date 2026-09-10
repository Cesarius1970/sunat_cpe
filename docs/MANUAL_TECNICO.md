# Manual Técnico - `sunat_cpe`

Manual técnico y de arquitectura para la librería `sunat_cpe`, desarrollada en Rust para la emisión, firma, validación y envío de Comprobantes de Pago Electrónicos (CPE) bajo los estándares de la Superintendencia Nacional de Aduanas y de Administración Tributaria (SUNAT - Perú).

---

## 1. Arquitectura del Sistema

La librería está estructurada en capas desacopladas con el principio de responsabilidad única, garantizando seguridad en tiempo de compilación y cero costo en abstracciones críticas.

```mermaid
flowchart TD
    A["Modelo de Dominio (CPE Data)"] --> B["Generador UBL (XML UBL 2.1 / 2.0)"]
    B --> C["Motor de Firma Digital (XMLDSig)"]
    C --> D["Empaquetador ZIP (RUC-Tipo-Serie-Num)"]
    D --> E["Cliente Web Service (SOAP / REST)"]
    E --> F["Parser y Validador de CDR"]
```

### Capas Principales:
1. **Modelos de Dominio (`cpe_modelos`)**: Representación tipada de Facturas, Boletas, Notas de Crédito, Notas de Débito, Guías de Remisión, etc. (`CpeFactura`, `CpeBoleta`, etc.), junto con sus catálogos tributarios SUNAT (`CPE_CATALOGO_*`).
2. **Generador UBL (`cpe_ubl`)**: Transformación de modelos fuertemente tipados a documentos XML compatibles con OASIS UBL 2.0 y 2.1 (`cpe_generar_factura_xml`, etc.).
3. **Motor de Firma Digital (`cpe_firma`)**: Implementación del estándar XMLDSig (Enveloped Signature), canonicalización (C14N) y firmado con clave privada RSA (SHA-256 / SHA-1) usando certificados X.509 (`.pfx` / `.p12`).
4. **Empaquetado y Compresión (`cpe_empaquetado`)**: Generación de archivos comprimidos en formato ZIP con la nomenclatura exigida por SUNAT (`{RUC}-{TIPO}-{SERIE}-{NUMERO}.zip`).
5. **Cliente de Servicios Web (`cpe_ws`)**: Conexión segura TLS/HTTPS con los servicios SOAP / REST de SUNAT (OSE / Beta / Homologación / Producción).
6. **Recepción y Validación de CDR (`cpe_cdr`)**: Descompresión y lectura del XML de Constancia de Recepción (CDR) devuelto por SUNAT para determinar el estado de aceptación o rechazo.
7. **Manejo de Errores (`cpe_error`)**: Errores fuertemente tipados con `CpeError`.

---

## 2. Flujo del Proceso de Emisión (Algoritmo General)

```mermaid
sequenceDiagram
    autonumber
    participant App as Aplicación Cliente
    participant Lib as sunat_cpe
    participant Sunat as Servidor SUNAT / OSE

    App->>Lib: Construir Comprobante (Datos validados)
    Lib->>Lib: Serializar a XML UBL 2.1
    Lib->>Lib: Canonicalizar y Firmar Digitalmente (XMLDSig)
    Lib->>Lib: Comprimir XML en ZIP (RUC-TIPO-SERIE-NUM.zip)
    Lib->>Lib: Codificar ZIP en Base64 e inyectar en SOAP Envelope
    Lib->>Sunat: Invocar sendBill(fileName, contentFile)
    Sunat-->>Lib: Retornar applicationResponse (ZIP con CDR en Base64)
    Lib->>Lib: Descomprimir y validar CDR (R-*.xml)
    Lib-->>App: Resultado (Aceptado, Observado, Rechazado)
```

---

## 3. Algoritmos Clave y Especificaciones Técnicas

### 3.1 Algoritmo de Generación XML UBL
- **Objetivo**: Producir un documento XML válido sintáctica y semánticamente según los esquemas XSD de SUNAT basados en OASIS UBL 2.1.
- **Pasos**:
  1. Mapeo de campos del modelo Rust a los nodos UBL correspondientes (`cac:`, `cbc:`).
  2. Inserción del nodo `ext:UBLExtensions` reservado para la firma digital.
  3. Formateo numérico estricto (importes con 2 o más decimales según catálogo, sin separadores de miles).
  4. Inclusión de Códigos de Leyendas (Catálogo No. 52).

### 3.2 Algoritmo de Firma Digital (XMLDSig Enveloped)
- **Normativa**: SUNAT Resolución de Superintendencia Nº 097-2012/SUNAT y modificatorias.
- **Pasos del Algoritmo**:
  1. **Canonicalización del Documento**: Aplicar C14N (Canonical XML sin comentarios: `http://www.w3.org/TR/2001/REC-xml-c14n-20010315`) omitiendo el bloque donde se insertará la firma (`<ext:UBLExtension>`).
  2. **Cálculo del Digest Value**:
     $$\text{DigestValue} = \text{Base64}(\text{SHA256}(\text{C14N}(\text{XML})))$$
  3. **Construcción del nodo `<SignedInfo>`**:
     - Método de canonicalización: `http://www.w3.org/TR/2001/REC-xml-c14n-20010315`.
     - Algoritmo de firma: `http://www.w3.org/2001/04/xmldsig-more#rsa-sha256`.
     - DigestMethod: `http://www.w3.org/2001/04/xmlenc#sha256`.
  4. **Firma del `<SignedInfo>`**:
     $$\text{SignatureValue} = \text{Base64}(\text{RSASign}_{\text{PrivateKey}}(\text{C14N}(\text{SignedInfo})))$$
  5. **Extracción del Certificado X.509**: Incrustar el certificado público en `<KeyInfo><X509Data><X509Certificate>`.
  6. **Incrustación**: Insertar el bloque `<ds:Signature>` completo en el primer `UBLExtension` del comprobante.

### 3.3 Algoritmo de Empaquetado ZIP
- **Nomenclatura Obligatoria**:
  $$\text{NombreArchivo} = \text{RUC}_{11} - \text{TipoCPE}_2 - \text{Serie}_4 - \text{Correlativo}_{1..8}$$
  - Ejemplo: `20123456789-01-F001-00000001.xml`
- **Compresión**: Formato ZIP estándar conteniendo únicamente el archivo XML firmado, nombrado idénticamente con extensión `.zip`.

### 3.4 Algoritmo de Comunicación Web Service (SOAP)
- **Métodos SUNAT**:
  - `sendBill`: Envío sincrónico de Facturas, Boletas y Notas asociadas. Devuelve el CDR de inmediato en la respuesta.
  - `sendSummary`: Envío asincrónico de Resúmenes Diarios de Boletas y Comunicaciones de Baja. Devuelve un número de ticket.
  - `getStatus`: Consulta del estado del ticket generado por `sendSummary`.
  - `sendPack`: Envío de Guías de Remisión Electrónica.
- **Autenticación**: Cabeceras `wsse:Security` con `UsernameToken`:
  - `Username`: `[RUC][USUARIO_SOL]` (ej. `20123456789MODDATOS`)
  - `Password`: `[CLAVE_SOL]` (ej. `moddatos`)

### 3.5 Algoritmo de Procesamiento de CDR (Constancia de Recepción)
- **Estructura del CDR**: Archivo `R-[RUC]-[TIPO]-[SERIE]-[NUMERO].zip` retornado en Base64 dentro del elemento `applicationResponse`.
- **Evaluación del Código de Respuesta (`cbc:ResponseCode`)**:
  - `0`: **Aceptado** (Comprobante válido y aceptado por SUNAT).
  - `0100` a `1999`: **Aceptado con observaciones** (Válido tributariamente, pero requiere subsanar advertencias).
  - `>= 2000`: **Rechazado** (Comprobante inválido, no tiene validez tributaria).

---

## 4. Convenciones de Código y Documentación en Rust

1. **Documentación de Crate y Módulos (`//!`)**: Cada módulo expone su propósito técnico, ejemplos de uso y los catálogos SUNAT que implementa.
2. **Documentación de Elementos Públicos (`///`)**: Todas las funciones, structs, enums y traits públicos cuentan con docstrings explicando:
   - Parámetros y tipos.
   - Errores retornados (`# Errors`).
   - Ejemplos ejecutables vía doc-tests (`# Examples`).
3. **Comentarios de Implementación (`//`)**:
   - Se reservan para explicar el *por qué* de decisiones de diseño, algoritmos matemáticos/criptográficos o particularidades de la normativa técnica SUNAT.
4. **Manejo de Errores con Tipos Dedicados**:
   - `CpeError`: Enum tipado con `thiserror` que desglosa errores de validación local, serialización XML, firma criptográfica, empaquetado, transporte de red y respuestas devueltas por SUNAT.

---

## 5. Mantenimiento del Manual

Este manual debe mantenerse actualizado de forma continua ante:
- Incorporación de nuevos módulos o algoritmos.
- Actualizaciones de esquemas normativos por parte de SUNAT (nuevas resoluciones de superintendencia).
- Cambios en las firmas criptográficas o protocolos de transporte (ej. migración a REST API de Guías de Remisión).
