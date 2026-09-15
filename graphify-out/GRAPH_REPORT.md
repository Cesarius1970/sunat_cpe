# Graph Report - sunat_cpe  (2026-09-15)

## Corpus Check
- 33 files · ~72,951 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 3 file(s) not represented in the graph (top: (none) 3)

## Summary
- 286 nodes · 450 edges · 19 communities (16 shown, 3 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 8 edges (avg confidence: 0.9)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Comprobantes Factura y Boleta
- Arquitectura y Publicación Crates.io
- Catálogo 06 Documentos Identidad
- Procesamiento de Constancias CDR
- Firma XMLDSig y Errores
- Catálogos Notas Crédito y Débito
- Clientes SOAP y Red SUNAT
- Resúmenes Diarios y Bajas
- Catálogo 01 Tipos Comprobante
- Catálogo 07 Afectación IGV
- Catálogo 02 Códigos de Moneda
- Catálogo 08 Sistema de ISC
- Catálogo 17 Tipos de Operación
- Catálogo 52 Códigos de Leyenda
- Catálogo 59 Medios de Pago
- Serializador XML UBL 2.1
- Módulo Raíz Catálogos
- Módulo Raíz Errores
- Crate Principal sunat_cpe

## God Nodes (most connected - your core abstractions)
1. `CpeBoleta` - 18 edges
2. `CpeFactura` - 18 edges
3. `CpeNotaCredito` - 16 edges
4. `CpeNotaDebito` - 16 edges
5. `CpeTipoDocumento` - 14 edges
6. `CpeTipoMoneda` - 12 edges
7. `CpeCdr` - 12 edges
8. `CpeCredencialesSol` - 12 edges
9. `CpeTipoAfectacionIgv` - 11 edges
10. `CpeEmisor` - 11 edges

## Surprising Connections (you probably didn't know these)
- `cpe_construir_envelope_send_bill()` --calls--> `cpe_codificar_base64()`  [INFERRED]
  src/cpe_ws/mod.rs → src/cpe_empaquetado/mod.rs
- `cpe_construir_envelope_send_summary()` --calls--> `cpe_codificar_base64()`  [INFERRED]
  src/cpe_ws/mod.rs → src/cpe_empaquetado/mod.rs
- `cpe_extraer_cdr_de_respuesta_soap()` --calls--> `cpe_decodificar_base64()`  [INFERRED]
  src/cpe_ws/mod.rs → src/cpe_empaquetado/mod.rs
- `Guía de Publicación en Crates.io` --references--> `Crate sunat_cpe`  [EXTRACTED]
  docs/GUIA_PUBLICACION_CRATES_IO.md → README.md
- `Manual Técnico sunat_cpe` --references--> `Crate sunat_cpe`  [EXTRACTED]
  docs/MANUAL_TECNICO.md → README.md

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Pipeline de Emisión y Validación CPE SUNAT** — docs_manual_tecnico_cpe_modelos, docs_manual_tecnico_cpe_ubl, docs_manual_tecnico_cpe_firma, docs_manual_tecnico_cpe_empaquetado, docs_manual_tecnico_cpe_ws, docs_manual_tecnico_cpe_cdr [EXTRACTED 1.00]
- **Cumplimiento de Precisión Numérica y Aritmética Decimal en Base 10** — docs_reglas_proyecto_prohibicion_punto_flotante, docs_manual_tecnico_antifloating_point, docs_manual_tecnico_cpe_modelos [INFERRED 0.95]
- **Sistema de Modelado y Visualización de Arquitectura Técnica** — docs_arquitectura_sunat_cpe_diagram, docs_historico_historico_solicitudes_adopcion_archify, docs_manual_tecnico_flujo_emision [INFERRED 0.85]

## Communities (19 total, 3 thin omitted)

### Community 0 - "Comprobantes Factura y Boleta"
Cohesion: 0.11
Nodes (28): Default, CpeBoleta, Option, String, Vec, CpeCuotaPago, CpeDireccion, CpeDocumentoReferencia (+20 more)

### Community 1 - "Arquitectura y Publicación Crates.io"
Cohesion: 0.07
Nodes (32): Aplicación Cliente (ERP / POS / Facturación), Diagrama Interactivo de Arquitectura de sunat_cpe, Servidores Fiscales (SUNAT / OSE), Reglas de Inmutabilidad de Crates.io, Guía de Publicación en Crates.io, Protocolo de Verificación Pre-Publicación, Adopción de Archify para Arquitectura Técnica, Historial de Solicitudes y Respuestas (+24 more)

### Community 2 - "Catálogo 06 Documentos Identidad"
Cohesion: 0.09
Nodes (12): CpeTipoDocumentoIdentidad, Display, Formatter, Result, Self, String, CpeTipoNotaCredito, Display (+4 more)

### Community 3 - "Procesamiento de Constancias CDR"
Cohesion: 0.18
Nodes (15): CpeCdr, CpeEstadoCdr, extraer_etiqueta(), CpeResult, Option, Self, String, Vec (+7 more)

### Community 4 - "Firma XMLDSig y Errores"
Cohesion: 0.16
Nodes (9): Error, RsaPrivateKey, CpeError, String, CpeCertificadoDigital, CpeFirmador, CpeResult, Self (+1 more)

### Community 5 - "Catálogos Notas Crédito y Débito"
Cohesion: 0.12
Nodes (10): CpeTipoNotaDebito, Display, Formatter, Result, Self, String, CpeNotaDebito, Option (+2 more)

### Community 6 - "Clientes SOAP y Red SUNAT"
Cohesion: 0.33
Nodes (14): cpe_construir_envelope_get_status(), cpe_construir_envelope_send_bill(), cpe_construir_envelope_send_summary(), cpe_construir_soap_header_security(), cpe_enviar_documento_async(), cpe_enviar_documento_sync(), cpe_enviar_resumen_async(), cpe_enviar_resumen_sync() (+6 more)

### Community 7 - "Resúmenes Diarios y Bajas"
Cohesion: 0.23
Nodes (7): CpeComunicacionBaja, CpeItemComunicacionBaja, CpeItemResumenDiario, CpeResumenDiario, Decimal, String, Vec

### Community 8 - "Catálogo 01 Tipos Comprobante"
Cohesion: 0.13
Nodes (6): CpeTipoDocumento, Display, Formatter, Result, Self, String

### Community 9 - "Catálogo 07 Afectación IGV"
Cohesion: 0.17
Nodes (6): CpeTipoAfectacionIgv, Display, Formatter, Result, Self, String

### Community 10 - "Catálogo 02 Códigos de Moneda"
Cohesion: 0.18
Nodes (6): CpeTipoMoneda, Display, Formatter, Result, Self, String

### Community 11 - "Catálogo 08 Sistema de ISC"
Cohesion: 0.18
Nodes (6): CpeTipoSistemaIsc, Display, Formatter, Result, Self, String

### Community 12 - "Catálogo 17 Tipos de Operación"
Cohesion: 0.18
Nodes (6): CpeTipoOperacion, Display, Formatter, Result, Self, String

### Community 13 - "Catálogo 52 Códigos de Leyenda"
Cohesion: 0.18
Nodes (6): CpeCodigoLeyenda, Display, Formatter, Result, Self, String

### Community 14 - "Catálogo 59 Medios de Pago"
Cohesion: 0.18
Nodes (6): CpeMedioPago, Display, Formatter, Result, Self, String

### Community 15 - "Serializador XML UBL 2.1"
Cohesion: 0.36
Nodes (5): CpeBoleta, CpeFactura, CpeUblSerializador, CpeResult, String

## Knowledge Gaps
- **14 isolated node(s):** `sunat_cpe`, `Soporte UBL 2.0 y 2.1`, `Tipos de Comprobantes SUNAT`, `Firma Digital XMLDSig`, `Protocolo de Verificación Pre-Publicación` (+9 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 120 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **3 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `CpeBoleta` connect `Comprobantes Factura y Boleta` to `Catálogo 01 Tipos Comprobante`, `Catálogo 02 Códigos de Moneda`, `Catálogo 17 Tipos de Operación`?**
  _High betweenness centrality (0.066) - this node is a cross-community bridge._
- **Why does `CpeFactura` connect `Comprobantes Factura y Boleta` to `Catálogo 01 Tipos Comprobante`, `Catálogo 02 Códigos de Moneda`, `Catálogo 17 Tipos de Operación`?**
  _High betweenness centrality (0.066) - this node is a cross-community bridge._
- **Why does `CpeNotaCredito` connect `Comprobantes Factura y Boleta` to `Catálogo 01 Tipos Comprobante`, `Catálogo 02 Códigos de Moneda`, `Catálogo 06 Documentos Identidad`?**
  _High betweenness centrality (0.065) - this node is a cross-community bridge._
- **What connects `sunat_cpe`, `Soporte UBL 2.0 y 2.1`, `Tipos de Comprobantes SUNAT` to the rest of the system?**
  _14 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Comprobantes Factura y Boleta` be split into smaller, more focused modules?**
  _Cohesion score 0.10569105691056911 - nodes in this community are weakly interconnected._
- **Should `Arquitectura y Publicación Crates.io` be split into smaller, more focused modules?**
  _Cohesion score 0.06854838709677419 - nodes in this community are weakly interconnected._
- **Should `Catálogo 06 Documentos Identidad` be split into smaller, more focused modules?**
  _Cohesion score 0.08695652173913043 - nodes in this community are weakly interconnected._