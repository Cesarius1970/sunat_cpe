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
