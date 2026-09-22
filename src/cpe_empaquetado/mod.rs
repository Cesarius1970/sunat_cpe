//! # Módulo de Empaquetado y Compresión ZIP - `cpe_empaquetado`
//!
//! Generación de archivos comprimidos ZIP según la nomenclatura técnica de SUNAT
//! y utilitarios para codificación y decodificación Base64 en memoria.

use crate::cpe_error::{CpeError, CpeResult};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use std::io::{Cursor, Read, Write};
use zip::write::SimpleFileOptions;
use zip::{ZipArchive, ZipWriter};

/// Comprime el contenido XML en un archivo ZIP en memoria con el nombre especificado.
///
/// # Parámetros
/// - `nombre_archivo_xml`: Nombre interno del archivo (ej. `20123456789-01-F001-00000001.xml`).
/// - `contenido_xml`: Contenido XML en bytes (generalmente codificado en ISO-8859-1 o UTF-8).
///
/// # Errors
/// Retorna `CpeError::ErrorEmpaquetado` si la compresión falla.
pub fn cpe_comprimir_xml_a_zip(
    nombre_archivo_xml: &str,
    contenido_xml: &[u8],
) -> CpeResult<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut zip = ZipWriter::new(Cursor::new(&mut buffer));

    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o644);

    zip.start_file(nombre_archivo_xml, options)
        .map_err(|e| CpeError::ErrorEmpaquetado(format!("Error al iniciar archivo en ZIP: {e}")))?;

    zip.write_all(contenido_xml)
        .map_err(|e| CpeError::ErrorEmpaquetado(format!("Error al escribir XML en ZIP: {e}")))?;

    zip.finish()
        .map_err(|e| CpeError::ErrorEmpaquetado(format!("Error al finalizar archivo ZIP: {e}")))?;

    Ok(buffer)
}

/// Descomprime un archivo ZIP en memoria y extrae el contenido del primer archivo encontrado.
///
/// # Errors
/// Retorna `CpeError::ErrorEmpaquetado` si el archivo no es un ZIP válido o está vacío.
pub fn cpe_descomprimir_primer_archivo(zip_bytes: &[u8]) -> CpeResult<(String, Vec<u8>)> {
    let reader = Cursor::new(zip_bytes);
    let mut archive = ZipArchive::new(reader)
        .map_err(|e| CpeError::ErrorEmpaquetado(format!("Archivo ZIP inválido o corrupto: {e}")))?;

    if archive.is_empty() {
        return Err(CpeError::ErrorEmpaquetado(
            "El archivo ZIP está vacío".to_string(),
        ));
    }

    let mut archivo = archive.by_index(0).map_err(|e| {
        CpeError::ErrorEmpaquetado(format!("No se pudo leer el contenido del ZIP: {e}"))
    })?;

    let nombre = archivo.name().to_string();
    let mut contenido = Vec::new();
    archivo
        .read_to_end(&mut contenido)
        .map_err(|e| CpeError::ErrorEmpaquetado(format!("Error al extraer archivo: {e}")))?;

    Ok((nombre, contenido))
}

/// Codifica bytes en una cadena Base64 estándar.
#[must_use]
pub fn cpe_codificar_base64(bytes: &[u8]) -> String {
    BASE64.encode(bytes)
}

/// Decodifica una cadena Base64 a bytes.
///
/// # Errors
/// Retorna `CpeError::ErrorEmpaquetado` si la cadena no es Base64 válida.
pub fn cpe_decodificar_base64(texto_base64: &str) -> CpeResult<Vec<u8>> {
    BASE64
        .decode(texto_base64.trim())
        .map_err(|e| CpeError::ErrorEmpaquetado(format!("Error al decodificar Base64: {e}")))
}
