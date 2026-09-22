# Graph Report - sunat_cpe  (2026-09-22)

## Corpus Check
- 42 files · ~104,808 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 3 file(s) not represented in the graph (top: (none) 3)

## Summary
- 354 nodes · 572 edges · 21 communities (19 shown, 2 thin omitted)
- Extraction: 99% EXTRACTED · 1% INFERRED · 0% AMBIGUOUS · INFERRED: 7 edges (avg confidence: 0.88)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Modelos Comprobantes Factura y Boleta
- Arquitectura y Evidencia Visual
- Catálogo 06 Tipos de Documento Identidad
- Catálogo 01 Tipos de Comprobante
- Firma Digital XMLDSig y PKCS#12
- Parser y Validación de Constancias CDR
- Cliente Web Service SOAP y Seguridad
- Estructuras de Modelos CPE y Re-exportaciones
- Serialización XML OASIS UBL 2.1
- Catálogo 07 Afectación al IGV
- Representación Impresa y Códigos QR
- Catálogo 02 Tipos de Moneda
- Catálogo 08 Sistema de Cálculo ISC
- Catálogo 09 Tipos de Nota de Crédito
- Catálogo 10 Tipos de Nota de Débito
- Catálogo 17 Tipos de Operación SUNAT
- Guías de Publicación Crates.io
- Directrices de Gobernanza y Arquitectura
- Historial de Solicitudes y Buenas Prácticas
- Regla Antifloating y Precisión Decimal
- Crate Raíz sunat_cpe

## God Nodes (most connected - your core abstractions)
1. `CpeBoleta` - 18 edges
2. `CpeFactura` - 18 edges
3. `CpeNotaCredito` - 16 edges
4. `CpeNotaDebito` - 16 edges
5. `CpeTipoDocumento` - 15 edges
6. `CpeTipoMoneda` - 13 edges
7. `CpeInvoiceParams` - 13 edges
8. `CpeCdr` - 12 edges
9. `CpeEmisor` - 12 edges
10. `CpeCredencialesSol` - 12 edges

## Surprising Connections (you probably didn't know these)
- `Captura Visual Check 1440x900 Modo Oscuro` --references--> `Diagrama Interactivo de Arquitectura sunat_cpe`  [EXTRACTED]
  docs/arquitectura_sunat_cpe.visual-check.1440x900.dark.png → docs/arquitectura_sunat_cpe.html
- `Captura Visual Check 1440x900 Modo Claro` --references--> `Diagrama Interactivo de Arquitectura sunat_cpe`  [EXTRACTED]
  docs/arquitectura_sunat_cpe.visual-check.1440x900.light.png → docs/arquitectura_sunat_cpe.html
- `Captura Visual Check 2048x1320 Modo Oscuro` --references--> `Diagrama Interactivo de Arquitectura sunat_cpe`  [EXTRACTED]
  docs/arquitectura_sunat_cpe.visual-check.2048x1320.dark.png → docs/arquitectura_sunat_cpe.html
- `Captura Visual Check 2048x1320 Modo Claro` --references--> `Diagrama Interactivo de Arquitectura sunat_cpe`  [EXTRACTED]
  docs/arquitectura_sunat_cpe.visual-check.2048x1320.light.png → docs/arquitectura_sunat_cpe.html
- `cpe_construir_envelope_send_bill()` --calls--> `cpe_codificar_base64()`  [INFERRED]
  src/cpe_ws/mod.rs → src/cpe_empaquetado/mod.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Cumplimiento de Precisión Numérica y Aritmética Decimal en Base 10** — docs_reglas_proyecto_prohibicion_punto_flotante, docs_manual_tecnico_cpe_modelos [INFERRED 0.95]
- **Pipeline Integral de Emisión y Envío de Comprobantes Electrónicos** — docs_manual_tecnico_cpe_modelos, docs_manual_tecnico_cpe_ubl, docs_manual_tecnico_cpe_firma, docs_manual_tecnico_cpe_empaquetado, docs_manual_tecnico_cpe_ws, docs_manual_tecnico_cpe_cdr [EXTRACTED 1.00]
- **Validación Pre-Vuelo y Cuadre de Integridad Fiscal sin Coma Flotante** — docs_manual_tecnico_cpe_validador, docs_manual_tecnico_modulo11_ruc, docs_manual_tecnico_aritmetica_antifloating, docs_plan_implementacion_refactorizaciones_y_mejoras_paso3_utilidades_validador [EXTRACTED 1.00]
- **Plan Estratégico de Tres Pasos para Refactorización y Extensión** — docs_plan_implementacion_refactorizaciones_y_mejoras_paso1_ubl_cdr, docs_plan_implementacion_refactorizaciones_y_mejoras_paso2_notas, docs_plan_implementacion_refactorizaciones_y_mejoras_paso3_utilidades_validador [EXTRACTED 1.00]

## Communities (21 total, 2 thin omitted)

### Community 0 - "Modelos Comprobantes Factura y Boleta"
Cohesion: 0.09
Nodes (34): Default, CpeBoleta, Option, String, Vec, CpeCuotaPago, CpeDireccion, CpeDocumentoReferencia (+26 more)

### Community 1 - "Arquitectura y Evidencia Visual"
Cohesion: 0.06
Nodes (40): Diagrama Interactivo de Arquitectura sunat_cpe, Vista Arquitectónica Flujo de Emisión y Envío, Vista Arquitectónica Transporte y Web Service, Vista Arquitectónica Validación y Criptografía, Captura Visual Check 1440x900 Modo Oscuro, Captura Visual Check 1440x900 Modo Claro, Captura Visual Check 2048x1320 Modo Oscuro, Captura Visual Check 2048x1320 Modo Claro (+32 more)

### Community 2 - "Catálogo 06 Tipos de Documento Identidad"
Cohesion: 0.06
Nodes (18): CpeTipoDocumentoIdentidad, Display, Formatter, Result, Self, String, CpeCodigoLeyenda, Display (+10 more)

### Community 3 - "Catálogo 01 Tipos de Comprobante"
Cohesion: 0.10
Nodes (13): CpeTipoDocumento, Display, Formatter, Result, Self, String, CpeComunicacionBaja, CpeItemComunicacionBaja (+5 more)

### Community 4 - "Firma Digital XMLDSig y PKCS#12"
Cohesion: 0.12
Nodes (8): RsaPrivateKey, cpe_extraer_hash_resumen(), CpeCertificadoDigital, CpeFirmador, CpeResult, Self, String, test_extraccion_hash_resumen()

### Community 5 - "Parser y Validación de Constancias CDR"
Cohesion: 0.13
Nodes (17): Error, CpeCdr, CpeEstadoCdr, CpeResult, Option, Self, String, Vec (+9 more)

### Community 6 - "Cliente Web Service SOAP y Seguridad"
Cohesion: 0.33
Nodes (14): cpe_construir_envelope_get_status(), cpe_construir_envelope_send_bill(), cpe_construir_envelope_send_summary(), cpe_construir_soap_header_security(), cpe_enviar_documento_async(), cpe_enviar_documento_sync(), cpe_enviar_resumen_async(), cpe_enviar_resumen_sync() (+6 more)

### Community 7 - "Estructuras de Modelos CPE y Re-exportaciones"
Cohesion: 0.35
Nodes (6): CpeBoleta, CpeFactura, CpeNotaCredito, CpeNotaDebito, CpeValidador, CpeResult

### Community 8 - "Serialización XML OASIS UBL 2.1"
Cohesion: 0.27
Nodes (9): cpe_construir_invoice_ubl_interno(), cpe_xml_escape(), CpeBoleta, CpeFactura, CpeNotaCredito, CpeNotaDebito, CpeUblSerializador, CpeResult (+1 more)

### Community 9 - "Catálogo 07 Afectación al IGV"
Cohesion: 0.17
Nodes (6): CpeTipoAfectacionIgv, Display, Formatter, Result, Self, String

### Community 10 - "Representación Impresa y Códigos QR"
Cohesion: 0.23
Nodes (6): CpeBoleta, CpeFactura, CpeNotaCredito, CpeNotaDebito, CpeRepresentacionImpresa, String

### Community 11 - "Catálogo 02 Tipos de Moneda"
Cohesion: 0.18
Nodes (6): CpeTipoMoneda, Display, Formatter, Result, Self, String

### Community 12 - "Catálogo 08 Sistema de Cálculo ISC"
Cohesion: 0.18
Nodes (6): CpeTipoSistemaIsc, Display, Formatter, Result, Self, String

### Community 13 - "Catálogo 09 Tipos de Nota de Crédito"
Cohesion: 0.18
Nodes (6): CpeTipoNotaCredito, Display, Formatter, Result, Self, String

### Community 14 - "Catálogo 10 Tipos de Nota de Débito"
Cohesion: 0.18
Nodes (6): CpeTipoNotaDebito, Display, Formatter, Result, Self, String

### Community 15 - "Catálogo 17 Tipos de Operación SUNAT"
Cohesion: 0.18
Nodes (6): CpeTipoOperacion, Display, Formatter, Result, Self, String

### Community 16 - "Guías de Publicación Crates.io"
Cohesion: 0.29
Nodes (7): Reglas de Inmutabilidad de Crates.io, Guía de Publicación en Crates.io, Protocolo de Verificación Pre-Publicación, Tipos de Comprobantes SUNAT, Firma Digital XMLDSig, Crate sunat_cpe, Soporte UBL 2.0 y 2.1

### Community 17 - "Directrices de Gobernanza y Arquitectura"
Cohesion: 0.29
Nodes (7): Alcance Exclusivo CPE y Exclusión de SIRE, Decisiones Arquitectónicas GRILL-ME, Convenciones Git y Commits, Principios Karpathy Guidelines, Regla de Idioma Español y Prefijo CPE, Reglas del Proyecto, Estándares de Rust Best Practices

### Community 18 - "Historial de Solicitudes y Buenas Prácticas"
Cohesion: 0.67
Nodes (3): Historial de Solicitudes y Respuestas sunat_cpe, Directrices de Desarrollo Karpathy Guidelines, Buenas Prácticas de Rust y Rendimiento de Grado Librería

## Knowledge Gaps
- **25 isolated node(s):** `sunat_cpe`, `Soporte UBL 2.0 y 2.1`, `Tipos de Comprobantes SUNAT`, `Firma Digital XMLDSig`, `Protocolo de Verificación Pre-Publicación` (+20 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 147 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **2 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `CpeValidador` connect `Estructuras de Modelos CPE y Re-exportaciones` to `Parser y Validación de Constancias CDR`?**
  _High betweenness centrality (0.224) - this node is a cross-community bridge._
- **Why does `CpeTotales` connect `Modelos Comprobantes Factura y Boleta` to `Estructuras de Modelos CPE y Re-exportaciones`?**
  _High betweenness centrality (0.198) - this node is a cross-community bridge._
- **Why does `CpeInvoiceParams` connect `Modelos Comprobantes Factura y Boleta` to `Serialización XML OASIS UBL 2.1`, `Catálogo 02 Tipos de Moneda`, `Catálogo 17 Tipos de Operación SUNAT`?**
  _High betweenness centrality (0.101) - this node is a cross-community bridge._
- **What connects `sunat_cpe`, `Soporte UBL 2.0 y 2.1`, `Tipos de Comprobantes SUNAT` to the rest of the system?**
  _25 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Modelos Comprobantes Factura y Boleta` be split into smaller, more focused modules?**
  _Cohesion score 0.08563134978229318 - nodes in this community are weakly interconnected._
- **Should `Arquitectura y Evidencia Visual` be split into smaller, more focused modules?**
  _Cohesion score 0.05897435897435897 - nodes in this community are weakly interconnected._
- **Should `Catálogo 06 Tipos de Documento Identidad` be split into smaller, more focused modules?**
  _Cohesion score 0.058823529411764705 - nodes in this community are weakly interconnected._