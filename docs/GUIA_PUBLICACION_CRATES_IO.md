# Guía Completa de Publicación en Crates.io - `sunat_cpe`

Este documento describe detalladamente los requisitos, procesos obligatorios y el procedimiento paso a paso para publicar la librería `sunat_cpe` en el registro oficial de Rust ([crates.io](https://crates.io)).

---

## 1. Verificación Previa de Disponibilidad

- **Nombre del Crate**: `sunat_cpe`
- **Estado**: **Disponible** (se consultó la API oficial de `crates.io/api/v1/crates/sunat_cpe` confirmando que el nombre no está registrado ni reservado).

---

## 2. Requisitos Previos y Cuenta en Crates.io

1. **Cuenta en GitHub**: crates.io utiliza GitHub para la autenticación de desarrolladores.
2. **Iniciar Sesión en Crates.io**:
   - Ingresar a [https://crates.io](https://crates.io) y hacer clic en **Log in with GitHub**.
3. **Verificación de Correo Electrónico (Obligatorio)**:
   - Ir a [Account Settings](https://crates.io/settings/profile).
   - Ingresar y confirmar su correo electrónico (`cesarvergarab@gmail.com`).
   - *Nota*: crates.io rechaza cualquier intento de publicación si la cuenta no tiene un correo verificado.
4. **Generar API Token de Publicación**:
   - Ir a [API Tokens](https://crates.io/settings/tokens).
   - Hacer clic en **New Token**.
   - Asignar un nombre identificador (ejemplo: `sunat_cpe_publish_token`).
   - Seleccionar los permisos (*scopes*):
     - `publish-new`: Para publicar el crate por primera vez.
     - `publish-update`: Para publicar actualizaciones y nuevas versiones.
   - Copiar el token generado (solo se muestra una vez).

---

## 3. Autenticación Local con Cargo

En la máquina de desarrollo, ejecutar el comando de autenticación:

```bash
cargo login <TU_API_TOKEN>
```

Esto almacena de forma segura las credenciales en `~/.cargo/credentials.toml`.

---

## 4. Requisitos Estándar en `Cargo.toml` (Completados)

crates.io y el equipo de empaquetado de Rust exigen los siguientes campos, los cuales ya están configurados en `Cargo.toml`:

| Campo | Valor Actual | Exigencia |
| :--- | :--- | :--- |
| `name` | `"sunat_cpe"` | Obligatorio y único en crates.io. |
| `version` | `"0.1.0"` | Obligatorio (SemVer). |
| `authors` | `["César A Vergara Buenaventura <cesarvergarab@gmail.com>"]` | Estándar de autoría. |
| `description` | Descripción técnica clara sobre emisión de CPE SUNAT en Rust. | Obligatorio. |
| `license` | `"MIT OR Apache-2.0"` | Expresión SPDX válida obligatoria. |
| `readme` | `"README.md"` | Obligatorio para renderizar portada en crates.io. |
| `documentation` | `"https://docs.rs/sunat_cpe"` | Recomendado oficial para docs.rs. |
| `keywords` | `["sunat", "cpe", "facturacion", "peru", "ubl"]` | Máximo 5 keywords estándar. |
| `categories` | `["financial", "parser-implementations"]` | Categorías oficiales de crates.io. |
| `exclude` | `["docs/historico/*"]` | Excluye historiales internos para aligerar el tarball. |

---

## 5. Protocolo de Verificación Estándar Pre-Publicación

Antes de publicar, es mandatorio ejecutar la suite de verificación local:

### 5.1 Ejecución de Pruebas Unitarias
```bash
cargo test --all-features
```

### 5.2 Linter Estricto de Clippy
```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### 5.3 Simulación de Empaquetado y Publicación (Dry-Run)
Verifica que el paquete se cree, se descomprima y compile en un entorno aislado sin subir nada a la red:
```bash
cargo publish --dry-run
```
*Resultado actual*: **Exitoso (35 archivos empaquetados, 0 advertencias)**.

---

## 6. Proceso de Publicación Oficial

Una vez autenticado con `cargo login` y con el repositorio en un commit limpio de git:

```bash
cargo publish
```

### Reglas Críticas e Inmutabilidad de Crates.io:
> [!IMPORTANT]
> **El código publicado en crates.io es permanente e inmutable**:
> 1. No se puede eliminar una versión publicada ni sobreescribir su contenido.
> 2. Si se detecta un error en `v0.1.0`, la solución consiste en corregir el código, subir la versión a `0.1.1` en `Cargo.toml` y volver a ejecutar `cargo publish`.
> 3. En casos de seguridad graves, se puede marcar una versión como obsoleta mediante `cargo yank --version 0.1.0`, pero no se borra.

---

## 7. Pasos Posteriores a la Publicación

1. **Generación Automática de Documentación en Docs.rs**:
   - En un lapso de 2 a 5 minutos posteriores a la publicación, [docs.rs](https://docs.rs/sunat_cpe) compilará y publicará automáticamente la documentación de la API en base a los comentarios `///` y `//!`.
2. **Etiquetado en Git (Git Tag)**:
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0 - Versión inicial sunat_cpe"
   git push origin v0.1.0
   ```
3. **Consumo por Terceros**:
   Los usuarios podrán incluir la librería en sus proyectos agregando en su `Cargo.toml`:
   ```toml
   [dependencies]
   sunat_cpe = "0.1.0"
   ```
