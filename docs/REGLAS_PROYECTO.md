# Reglas del Proyecto - `sunat_cpe`

Este documento consolida las reglas, lineamientos y estándares acordados para el desarrollo del proyecto.

---

## 1. Commits e Interacciones Git

- **Frecuencia de commits**: Al finalizar cada fase o interacción con el agente, se debe generar un commit en git.
- **Estándar de commits (Conventional Commits)**:
  - Formato: `<tipo>(<alcance opcional>): <descripción concisa>`
  - Tipos válidos:
    - `feat`: Nueva funcionalidad.
    - `fix`: Corrección de errores.
    - `docs`: Cambios en la documentación.
    - `refactor`: Refactorización de código sin alterar comportamiento.
    - `test`: Añadir o modificar pruebas.
    - `chore`: Mantenimiento, dependencias o configuración de build.
- **Estándar de ramas**:
  - `feat/<nombre-funcionalidad>`
  - `fix/<nombre-bug>`
  - `chore/<nombre-tarea>`
  - `docs/<nombre-tarea>`

---

## 2. Historial de Interacciones

- Mantener actualizada la carpeta `docs/historico/` con el archivo `HISTORICO_SOLICITUDES.md`.
- Cada interacción debe quedar registrada con su fecha, número secuencial, solicitud/prompt exacto y resumen de la respuesta provista.

---

## 3. Principios de Desarrollo (Karpathy Guidelines)

1. **Think Before Coding**: Especificar suposiciones de forma explícita, plantear alternativas/tradeoffs, y preguntar antes de asumir cuando existan dudas.
2. **Simplicity First**: Escribir la menor cantidad de código necesaria para resolver el requerimiento, sin sobre-ingeniería ni especulaciones prematuras.
3. **Surgical Changes**: Tocar únicamente lo necesario, respetando el estilo del proyecto y limpiando artefactos o código huérfano generado por los cambios propios.
4. **Goal-Driven Execution**: Definir criterios de éxito verificables mediante pruebas antes y después de cada cambio.

---

## 4. Estándares de Rust (Rust Best Practices)

- **Borrowing & Ownership**: Preferir préstamos (`&T`, `&str`, `&[T]`) sobre clones innecesarios.
- **Manejo de Errores**: Uso estricto de `Result<T, E>` y `thiserror` (biblioteca) / `anyhow` (binarios/CLI). Prohibido `unwrap()` o `expect()` fuera de tests.
- **Calidad y Linter**: Código limpio con `cargo clippy --all-targets --all-features --locked -- -D warnings`.
- **Pruebas**: Nombres de tests descriptivos, pruebas unitarias e integración rigurosas.

---

## 5. Idioma y Nomenclatura de Objetos

- **Idioma**: Debe usarse el idioma español de manera obligatoria en la documentación, comentarios, nombres de variables/estructuras/funciones y mensajes de error.
- **Prefijo `CPE_` / `cpe_`**:
  - En estructuras, traits o tipos: usar prefijo `Cpe` / `CPE_` según la convención de Rust (ejemplo: `CpeFactura`, `CpeBoleta`, `CpeResumenDiario`).
  - En funciones, módulos o métodos: usar prefijo `cpe_` (ejemplo: `cpe_generar_xml`, `cpe_firmar_documento`).
  - En constantes o estáticos: usar prefijo `CPE_` (ejemplo: `CPE_VERSION_UBL_2_1`, `CPE_CATALOGO_01_FACTURA`).
- **Excepción de interoperabilidad con SUNAT / UBL**:
  - Esta convención de prefijos e idioma no debe colisionar ni alterar la estructura exigida por los esquemas XSD de OASIS UBL ni los contratos de los servicios web (SOAP / REST) de SUNAT (ejemplos: nombres de elementos XML como `cac:PartyLegalEntity`, métodos SOAP como `sendBill` o `sendSummary`, nombres de tags y atributos de esquemas tributarios oficiales).

---

## 6. Prohibición de Punto Flotante para Valores Monetarios

- **Regla Estricta**: Queda terminantemente **prohibido el uso de tipos de punto flotante binario (`f32`, `f64`)** para representar, calcular o almacenar valores monetarios, tasas tributarias, cantidades o importes en la librería.
- **Respaldo Técnico según la Normativa y API de SUNAT**:
  - **Análisis de Validación SUNAT**: El motor de validación de SUNAT (UBL 2.1) aplica reglas aritméticas estrictas (códigos de error 2014, 2015, 2017, 2020, 2021, 2022, entre otros) que exigen coherencia matemática exacta entre la sumatoria de las líneas (`LineExtensionAmount`), impuestos calculados (IGV 18%, ISC, ICBPER) y el importe total a pagar (`PayableAmount`).
  - **Problema de IEEE 754**: Los tipos `f32` y `f64` almacenan números en base 2, lo que genera inexactitudes inherentes en fracciones decimales comunes (ej. `0.1 + 0.2 != 0.3`). Al multiplicar bases imponibles por alícuotas o acumular líneas, los errores de redondeo binario producen diferencias de centavos (`0.01`), provocando el rechazo inmediato del comprobante por SUNAT.
  - **Conclusión**: La declaración técnica y matriz de validaciones de SUNAT **respalda plenamente esta regla**.
- **Solución Técnica Obligatoria en Rust**:
  - Se debe utilizar aritmética decimal exacta en base 10 (por ejemplo mediante el crate `rust_decimal` o un tipo de dominio `CpeMontoDecimal` / enteros para precisión en escala).
  - Los totales finales deben redondearse a exactamente 2 decimales utilizando redondeo comercial estándar (Half Up), y los valores/precios unitarios deben permitir hasta 10 decimales según lo estipulado por SUNAT.

---

## 7. Alcance Exclusivo CPE (Exclusión Expresa de SIRE)

- **Alcance Permitido**: La investigación, diseño e implementación se restringe exclusivamente al subsistema de **Comprobantes de Pago Electrónicos (CPE)** bajo normativa de SUNAT:
  - Factura Electrónica (01)
  - Boleta de Venta Electrónica (03)
  - Nota de Crédito Electrónica (07)
  - Nota de Débito Electrónica (08)
  - Guía de Remisión Remitente y Transportista (09 y 31)
  - Comprobante de Retención (20) y Percepción (40)
  - Resumen Diario de Boletas (RC) y Comunicación de Baja (RA)
  - Consultas de CDR y estado de comprobantes ante SUNAT.
- **Exclusión Expresa**: Queda terminantemente excluido cualquier desarrollo, planificación o investigación relacionada con el **SIRE** (Sistema Integrado de Registros Electrónicos - RVIE/RCE) o sistemas contables accesorios que no pertenezcan al flujo de emisión y recepción de CPE.

---

## 8. Decisiones Arquitectónicas Acordadas (GRILL-ME)

- **Modelo de I/O Dual con Sufijos Explícitos**:
  - Soporte de comunicación dual mediante *feature flags* (`async` por defecto y `blocking`).
  - Las funciones de red y Web Services deben incorporar obligatoriamente el sufijo `_sync` o `_async` en su identificador (ejemplo: `cpe_enviar_factura_async`, `cpe_enviar_factura_sync`, `cpe_consultar_cdr_async`, `cpe_consultar_cdr_sync`).
- **Aritmética Decimal Exacta**:
  - Adopción estandarizada del crate `rust_decimal` (precisión de 128 bits en base 10) para todo cálculo o almacenamiento monetario.
- **Criptografía 100% Rust Nativo**:
  - La firma digital XMLDSig y la lectura de certificados `.pfx` / `.p12` se implementa en Rust puro (sin depender de bibliotecas compartidas C como OpenSSL en el sistema operativo host), garantizando portabilidad absoluta y compilación cruzada sin fricciones.




