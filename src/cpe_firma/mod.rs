//! # Módulo de Firma Digital XMLDSig - `cpe_firma`
//!
//! Implementación 100% Rust nativa para la firma digital de comprobantes XML
//! bajo el estándar W3C XML Signature (XMLDSig Enveloped) requerido por SUNAT.

use crate::cpe_error::{CpeError, CpeResult};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use rsa::RsaPrivateKey;
use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::pkcs1v15::SigningKey;
use rsa::pkcs8::DecodePrivateKey;
use rsa::signature::{SignatureEncoding, SignerMut};
use sha2::{Digest, Sha256};
use std::fmt::Write;

/// Trait para firmar digitalmente documentos XML de comprobantes de pago.
pub trait CpeFirmador {
    /// Toma un XML sin firmar y retorna el XML con el bloque `<ds:Signature>` incrustado.
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorFirmaDigital` si la firma criptográfica o la inyección fallan.
    fn cpe_firmar_xml(&self, xml_sin_firmar: &str) -> CpeResult<String>;
}

/// Representa una clave privada RSA y el certificado X.509 público asociado.
#[derive(Clone)]
pub struct CpeCertificadoDigital {
    /// Clave privada RSA para firmar los hashes del comprobante.
    pub clave_privada: RsaPrivateKey,
    /// Certificado X.509 público codificado en Base64.
    pub certificado_x509_base64: String,
}

impl CpeCertificadoDigital {
    /// Crea una nueva instancia a partir de una clave privada RSA y el certificado en Base64.
    #[must_use]
    pub fn nuevo(clave_privada: RsaPrivateKey, certificado_x509_base64: String) -> Self {
        Self {
            clave_privada,
            certificado_x509_base64: certificado_x509_base64.replace(['\r', '\n', ' '], ""),
        }
    }

    /// Carga una clave privada RSA y el certificado público desde un archivo contenedor PKCS#12 (`.p12` o `.pfx`).
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorFirmaDigital` si el archivo no puede parsearse, la contraseña es incorrecta
    /// o no se encuentra la clave privada o el certificado.
    pub fn desde_pkcs12(bytes_p12: &[u8], password: &str) -> CpeResult<Self> {
        let pfx = p12::PFX::parse(bytes_p12).map_err(|e| {
            CpeError::ErrorFirmaDigital(format!(
                "Error al parsear archivo PKCS#12 (.p12/.pfx): {e:?}"
            ))
        })?;

        let key_bags = pfx.key_bags(password).map_err(|e| {
            CpeError::ErrorFirmaDigital(format!(
                "Error al desencriptar clave privada PKCS#12: {e:?}"
            ))
        })?;

        let key_der = key_bags.into_iter().next().ok_or_else(|| {
            CpeError::ErrorFirmaDigital(
                "No se encontró clave privada en el archivo PKCS#12".to_string(),
            )
        })?;

        let clave_privada = RsaPrivateKey::from_pkcs8_der(&key_der)
            .or_else(|_| RsaPrivateKey::from_pkcs1_der(&key_der))
            .map_err(|e| {
                CpeError::ErrorFirmaDigital(format!(
                    "Error al decodificar clave privada RSA desde PKCS#12: {e}"
                ))
            })?;

        let cert_bags = pfx.cert_x509_bags(password).map_err(|e| {
            CpeError::ErrorFirmaDigital(format!(
                "Error al desencriptar certificado X.509 PKCS#12: {e:?}"
            ))
        })?;

        let cert_der = cert_bags.into_iter().next().ok_or_else(|| {
            CpeError::ErrorFirmaDigital(
                "No se encontró certificado X.509 en el archivo PKCS#12".to_string(),
            )
        })?;

        let certificado_x509_base64 = BASE64.encode(&cert_der);

        Ok(Self::nuevo(clave_privada, certificado_x509_base64))
    }
}

/// Extrae el DigestValue (hash SHA-256 en Base64) incrustado en la firma digital de un XML firmado.
///
/// Este hash es requerido oficialmente por SUNAT para imprimir en la representación física (PDF/Ticket)
/// y en el código de barras o código QR.
///
/// # Errors
/// Retorna `CpeError::ErrorFirmaDigital` si el comprobante no contiene el elemento `<ds:DigestValue>`.
pub fn cpe_extraer_hash_resumen(xml_firmado: &str) -> CpeResult<String> {
    if let Some(pos_inicio) = xml_firmado.find("DigestValue>") {
        let inicio = pos_inicio + "DigestValue>".len();
        if let Some(pos_fin) = xml_firmado[inicio..].find("</") {
            let hash = xml_firmado[inicio..inicio + pos_fin].trim();
            if !hash.is_empty() {
                return Ok(hash.to_string());
            }
        }
    }
    Err(CpeError::ErrorFirmaDigital(
        "No se encontró el elemento <ds:DigestValue> en el XML firmado".to_string(),
    ))
}

impl CpeFirmador for CpeCertificadoDigital {
    fn cpe_firmar_xml(&self, xml_sin_firmar: &str) -> CpeResult<String> {
        // 1. Calcular el Digest (SHA-256) del XML canónico
        let mut hasher = Sha256::new();
        hasher.update(xml_sin_firmar.as_bytes());
        let digest_bytes = hasher.finalize();
        let digest_base64 = BASE64.encode(digest_bytes);

        // 2. Construir el bloque SignedInfo canónico
        let mut signed_info = String::with_capacity(1024);
        signed_info.push_str("<SignedInfo xmlns=\"http://www.w3.org/2000/09/xmldsig#\">\n");
        signed_info.push_str("    <CanonicalizationMethod Algorithm=\"http://www.w3.org/TR/2001/REC-xml-c14n-20010315\"/>\n");
        signed_info.push_str("    <SignatureMethod Algorithm=\"http://www.w3.org/2001/04/xmldsig-more#rsa-sha256\"/>\n");
        signed_info.push_str("    <Reference URI=\"\">\n");
        signed_info.push_str("        <Transforms>\n");
        signed_info.push_str("            <Transform Algorithm=\"http://www.w3.org/2000/09/xmldsig#enveloped-signature\"/>\n");
        signed_info.push_str("        </Transforms>\n");
        signed_info.push_str(
            "        <DigestMethod Algorithm=\"http://www.w3.org/2001/04/xmlenc#sha256\"/>\n",
        );
        let _ = writeln!(
            &mut signed_info,
            "        <DigestValue>{}</DigestValue>",
            digest_base64
        );
        signed_info.push_str("    </Reference>\n");
        signed_info.push_str("</SignedInfo>");

        // 3. Firmar el SignedInfo con la clave privada RSA usando RSA-SHA256 (PKCS#1 v1.5)
        let mut signing_key = SigningKey::<Sha256>::new(self.clave_privada.clone());
        let firma = signing_key.sign(signed_info.as_bytes());
        let firma_base64 = BASE64.encode(firma.to_vec());

        // 4. Construir el elemento ds:Signature completo
        let mut signature_xml = String::with_capacity(2048);
        signature_xml.push_str(
            "<ds:Signature xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\" Id=\"SignSUNAT\">\n",
        );
        let _ = writeln!(&mut signature_xml, "    {}", signed_info);
        let _ = writeln!(
            &mut signature_xml,
            "    <ds:SignatureValue>{}</ds:SignatureValue>",
            firma_base64
        );
        signature_xml.push_str("    <ds:KeyInfo>\n");
        signature_xml.push_str("        <ds:X509Data>\n");
        let _ = writeln!(
            &mut signature_xml,
            "            <ds:X509Certificate>{}</ds:X509Certificate>",
            self.certificado_x509_base64
        );
        signature_xml.push_str("        </ds:X509Data>\n");
        signature_xml.push_str("    </ds:KeyInfo>\n");
        signature_xml.push_str("</ds:Signature>");

        // 5. Inyectar dentro de <ext:ExtensionContent/>
        if !xml_sin_firmar.contains("<ext:ExtensionContent/>") {
            return Err(CpeError::ErrorFirmaDigital(
                "No se encontró el elemento <ext:ExtensionContent/> en el XML UBL para inyectar la firma".to_string(),
            ));
        }

        let xml_firmado = xml_sin_firmar.replacen(
            "<ext:ExtensionContent/>",
            &format!(
                "<ext:ExtensionContent>\n{}\n        </ext:ExtensionContent>",
                signature_xml
            ),
            1,
        );

        Ok(xml_firmado)
    }
}
