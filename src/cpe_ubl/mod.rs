//! # Módulo de Generación UBL - `cpe_ubl`
//!
//! Generación de documentos XML conforme a las especificaciones OASIS UBL 2.1 y UBL 2.0
//! adoptadas por SUNAT en las Guías de Validación Oficiales.

use crate::cpe_catalogos::{CpeTipoMoneda, CpeTipoOperacion};
use crate::cpe_error::CpeResult;
use crate::cpe_modelos::{
    CpeBoleta, CpeDocumentoReferencia, CpeEmisor, CpeFactura, CpeFormaPago, CpeInformacionPago,
    CpeItem, CpeLeyenda, CpeNotaCredito, CpeNotaDebito, CpeReceptor, CpeTotales,
};
use std::fmt::Write;

/// Escapa caracteres especiales reservados en XML (&, <, >, ", ').
#[must_use]
pub fn cpe_xml_escape(texto: &str) -> String {
    let mut escapado = String::with_capacity(texto.len());
    for c in texto.chars() {
        match c {
            '&' => escapado.push_str("&amp;"),
            '<' => escapado.push_str("&lt;"),
            '>' => escapado.push_str("&gt;"),
            '"' => escapado.push_str("&quot;"),
            '\'' => escapado.push_str("&apos;"),
            _ => escapado.push(c),
        }
    }
    escapado
}

/// Trait unificado para cualquier comprobante o documento serializable a XML UBL.
///
/// Permite extender fácilmente la librería ante nuevos esquemas o versiones de SUNAT
/// simplemente implementando este trait para nuevos tipos de documentos.
pub trait CpeUblSerializador {
    /// Genera el documento XML UBL completo (sin firma digital incrustada).
    ///
    /// # Errors
    /// Retorna `CpeError::ErrorXml` si los campos requeridos no cumplen con las reglas UBL.
    fn a_xml_ubl(&self) -> CpeResult<String>;
}

/// Estructura interna para parametrizar la generación de facturas y boletas
/// bajo el esquema OASIS UBL 2.1 `Invoice-2`.
struct CpeInvoiceParams<'a> {
    tipo_documento_codigo: &'a str, // "01" para Factura, "03" para Boleta
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
    _informacion_pago: Option<&'a CpeInformacionPago>,
    items: &'a [CpeItem],
    totales: &'a CpeTotales,
    leyendas: &'a [CpeLeyenda],
    documentos_referencia: &'a [CpeDocumentoReferencia],
}

fn cpe_construir_invoice_ubl_interno(params: CpeInvoiceParams<'_>) -> CpeResult<String> {
    let mut xml = String::with_capacity(4096);
    xml.push_str("<?xml version=\"1.0\" encoding=\"ISO-8859-1\" standalone=\"no\"?>\n");
    xml.push_str("<Invoice xmlns=\"urn:oasis:names:specification:ubl:schema:xsd:Invoice-2\"\n");
    xml.push_str("    xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\"\n");
    xml.push_str(
        "    xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\"\n",
    );
    xml.push_str("    xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"\n");
    xml.push_str("    xmlns:ext=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\">\n");

    // Espacio reservado para la firma digital XMLDSig
    xml.push_str("    <ext:UBLExtensions>\n");
    xml.push_str("        <ext:UBLExtension>\n");
    xml.push_str("            <ext:ExtensionContent/>\n");
    xml.push_str("        </ext:UBLExtension>\n");
    xml.push_str("    </ext:UBLExtensions>\n");

    // Versión UBL y Personalización SUNAT
    xml.push_str("    <cbc:UBLVersionID>2.1</cbc:UBLVersionID>\n");
    xml.push_str("    <cbc:CustomizationID>2.0</cbc:CustomizationID>\n");

    // Identificador (Serie-Correlativo)
    let _ = writeln!(
        &mut xml,
        "    <cbc:ID>{}-{:08}</cbc:ID>",
        cpe_xml_escape(params.serie),
        params.correlativo
    );
    let _ = writeln!(
        &mut xml,
        "    <cbc:IssueDate>{}</cbc:IssueDate>",
        cpe_xml_escape(params.fecha_emision)
    );
    if let Some(hora) = params.hora_emision {
        let _ = writeln!(
            &mut xml,
            "    <cbc:IssueTime>{}</cbc:IssueTime>",
            cpe_xml_escape(hora)
        );
    }
    if let Some(vencimiento) = params.fecha_vencimiento {
        let _ = writeln!(
            &mut xml,
            "    <cbc:DueDate>{}</cbc:DueDate>",
            cpe_xml_escape(vencimiento)
        );
    }

    // Tipo de Comprobante (Catálogo 01: "01" Factura, "03" Boleta) y Tipo de Operación (Catálogo 17)
    let _ = writeln!(
        &mut xml,
        "    <cbc:InvoiceTypeCode listID=\"{}\">{}</cbc:InvoiceTypeCode>",
        params.tipo_operacion.codigo(),
        params.tipo_documento_codigo
    );

    // Leyendas (Catálogo 52)
    for leyenda in params.leyendas {
        let _ = writeln!(
            &mut xml,
            "    <cbc:Note languageLocaleID=\"{}\"><![CDATA[{}]]></cbc:Note>",
            leyenda.codigo.codigo(),
            leyenda.texto
        );
    }

    // Moneda (Catálogo 02)
    let _ = writeln!(
        &mut xml,
        "    <cbc:DocumentCurrencyCode>{}</cbc:DocumentCurrencyCode>",
        params.moneda.codigo()
    );

    // Guías de remisión o documentos relacionados
    for doc in params.documentos_referencia {
        xml.push_str("    <cac:DespatchDocumentReference>\n");
        let _ = writeln!(
            &mut xml,
            "        <cbc:ID>{}</cbc:ID>",
            cpe_xml_escape(&doc.serie_numero)
        );
        let _ = writeln!(
            &mut xml,
            "        <cbc:DocumentTypeCode>{}</cbc:DocumentTypeCode>",
            doc.tipo_documento.codigo()
        );
        xml.push_str("    </cac:DespatchDocumentReference>\n");
    }

    // Emisor (AccountingSupplierParty)
    xml.push_str("    <cac:AccountingSupplierParty>\n");
    xml.push_str("        <cac:Party>\n");
    xml.push_str("            <cac:PartyIdentification>\n");
    let _ = writeln!(
        &mut xml,
        "                <cbc:ID schemeID=\"6\">{}</cbc:ID>",
        cpe_xml_escape(&params.emisor.ruc)
    );
    xml.push_str("            </cac:PartyIdentification>\n");
    xml.push_str("            <cac:PartyName>\n");
    let nombre_comercial = params
        .emisor
        .nombre_comercial
        .as_deref()
        .unwrap_or(&params.emisor.razon_social);
    let _ = writeln!(
        &mut xml,
        "                <cbc:Name><![CDATA[{}]]></cbc:Name>",
        nombre_comercial
    );
    xml.push_str("            </cac:PartyName>\n");
    xml.push_str("            <cac:PartyLegalEntity>\n");
    let _ = writeln!(
        &mut xml,
        "                <cbc:RegistrationName><![CDATA[{}]]></cbc:RegistrationName>",
        params.emisor.razon_social
    );
    if let Some(ref dir) = params.emisor.direccion {
        xml.push_str("                <cac:RegistrationAddress>\n");
        if let Some(ref ubigeo) = dir.ubigeo {
            let _ = writeln!(
                &mut xml,
                "                    <cbc:ID>{}</cbc:ID>",
                cpe_xml_escape(ubigeo)
            );
        }
        let _ = writeln!(
            &mut xml,
            "                    <cac:AddressLine><cbc:Line><![CDATA[{}]]></cbc:Line></cac:AddressLine>",
            dir.direccion_detallada
        );
        let _ = writeln!(
            &mut xml,
            "                    <cac:Country><cbc:IdentificationCode>{}</cbc:IdentificationCode></cac:Country>",
            cpe_xml_escape(&dir.codigo_pais)
        );
        xml.push_str("                </cac:RegistrationAddress>\n");
    }
    xml.push_str("            </cac:PartyLegalEntity>\n");
    xml.push_str("        </cac:Party>\n");
    xml.push_str("    </cac:AccountingSupplierParty>\n");

    // Receptor (AccountingCustomerParty)
    xml.push_str("    <cac:AccountingCustomerParty>\n");
    xml.push_str("        <cac:Party>\n");
    xml.push_str("            <cac:PartyIdentification>\n");
    let _ = writeln!(
        &mut xml,
        "                <cbc:ID schemeID=\"{}\">{}</cbc:ID>",
        params.receptor.tipo_documento.codigo(),
        cpe_xml_escape(&params.receptor.numero_documento)
    );
    xml.push_str("            </cac:PartyIdentification>\n");
    xml.push_str("            <cac:PartyLegalEntity>\n");
    let _ = writeln!(
        &mut xml,
        "                <cbc:RegistrationName><![CDATA[{}]]></cbc:RegistrationName>",
        params.receptor.razon_social
    );
    xml.push_str("            </cac:PartyLegalEntity>\n");
    xml.push_str("        </cac:Party>\n");
    xml.push_str("    </cac:AccountingCustomerParty>\n");

    // Forma de Pago (Contado o Crédito con Cuotas)
    match params.forma_pago {
        CpeFormaPago::Contado => {
            xml.push_str("    <cac:PaymentTerms>\n");
            xml.push_str("        <cbc:ID>FormaPago</cbc:ID>\n");
            xml.push_str(
                "        <cbc:PaymentPaymentTermsID>Contado</cbc:PaymentPaymentTermsID>\n",
            );
            xml.push_str("    </cac:PaymentTerms>\n");
        }
        CpeFormaPago::Credito {
            monto_neto_pendiente,
            cuotas,
        } => {
            xml.push_str("    <cac:PaymentTerms>\n");
            xml.push_str("        <cbc:ID>FormaPago</cbc:ID>\n");
            xml.push_str(
                "        <cbc:PaymentPaymentTermsID>Credito</cbc:PaymentPaymentTermsID>\n",
            );
            let _ = writeln!(
                &mut xml,
                "        <cbc:Amount currencyID=\"{}\">{:.2}</cbc:Amount>",
                params.moneda.codigo(),
                monto_neto_pendiente
            );
            xml.push_str("    </cac:PaymentTerms>\n");

            for cuota in cuotas {
                xml.push_str("    <cac:PaymentTerms>\n");
                let _ = writeln!(
                    &mut xml,
                    "        <cbc:ID>FormaPago</cbc:ID>\n        <cbc:PaymentPaymentTermsID>{}</cbc:PaymentPaymentTermsID>",
                    cpe_xml_escape(&cuota.numero_cuota)
                );
                let _ = writeln!(
                    &mut xml,
                    "        <cbc:Amount currencyID=\"{}\">{:.2}</cbc:Amount>",
                    params.moneda.codigo(),
                    cuota.monto
                );
                let _ = writeln!(
                    &mut xml,
                    "        <cbc:PaymentDueDate>{}</cbc:PaymentDueDate>",
                    cpe_xml_escape(&cuota.fecha_vencimiento)
                );
                xml.push_str("    </cac:PaymentTerms>\n");
            }
        }
    }

    // Totales de Impuestos (TaxTotal IGV)
    xml.push_str("    <cac:TaxTotal>\n");
    let _ = writeln!(
        &mut xml,
        "        <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
        params.moneda.codigo(),
        params.totales.total_igv
    );
    xml.push_str("        <cac:TaxSubtotal>\n");
    let _ = writeln!(
        &mut xml,
        "            <cbc:TaxableAmount currencyID=\"{}\">{:.2}</cbc:TaxableAmount>",
        params.moneda.codigo(),
        params.totales.total_operaciones_gravadas
    );
    let _ = writeln!(
        &mut xml,
        "            <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
        params.moneda.codigo(),
        params.totales.total_igv
    );
    xml.push_str("            <cac:TaxCategory>\n");
    xml.push_str("                <cac:TaxScheme>\n");
    xml.push_str("                    <cbc:ID>1000</cbc:ID>\n");
    xml.push_str("                    <cbc:Name>IGV</cbc:Name>\n");
    xml.push_str("                    <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n");
    xml.push_str("                </cac:TaxScheme>\n");
    xml.push_str("            </cac:TaxCategory>\n");
    xml.push_str("        </cac:TaxSubtotal>\n");
    xml.push_str("    </cac:TaxTotal>\n");

    // Totales Monetarios Legales (LegalMonetaryTotal)
    xml.push_str("    <cac:LegalMonetaryTotal>\n");
    let _ = writeln!(
        &mut xml,
        "        <cbc:LineExtensionAmount currencyID=\"{}\">{:.2}</cbc:LineExtensionAmount>",
        params.moneda.codigo(),
        params.totales.total_operaciones_gravadas
    );
    let _ = writeln!(
        &mut xml,
        "        <cbc:TaxInclusiveAmount currencyID=\"{}\">{:.2}</cbc:TaxInclusiveAmount>",
        params.moneda.codigo(),
        params.totales.importe_total_pagar
    );
    let _ = writeln!(
        &mut xml,
        "        <cbc:PayableAmount currencyID=\"{}\">{:.2}</cbc:PayableAmount>",
        params.moneda.codigo(),
        params.totales.importe_total_pagar
    );
    xml.push_str("    </cac:LegalMonetaryTotal>\n");

    // Líneas de la Factura / Boleta (InvoiceLine)
    for item in params.items {
        xml.push_str("    <cac:InvoiceLine>\n");
        let _ = writeln!(&mut xml, "        <cbc:ID>{}</cbc:ID>", item.numero_orden);
        let _ = writeln!(
            &mut xml,
            "        <cbc:InvoicedQuantity unitCode=\"{}\">{:.10}</cbc:InvoicedQuantity>",
            cpe_xml_escape(&item.unidad_medida),
            item.cantidad
        );
        let _ = writeln!(
            &mut xml,
            "        <cbc:LineExtensionAmount currencyID=\"{}\">{:.2}</cbc:LineExtensionAmount>",
            params.moneda.codigo(),
            item.total_valor_venta
        );

        // Precios referenciales y unitarios
        xml.push_str("        <cac:PricingReference>\n");
        xml.push_str("            <cac:AlternativeConditionPrice>\n");
        let _ = writeln!(
            &mut xml,
            "                <cbc:PriceAmount currencyID=\"{}\">{:.10}</cbc:PriceAmount>",
            params.moneda.codigo(),
            item.precio_unitario
        );
        xml.push_str("                <cbc:PriceTypeCode>01</cbc:PriceTypeCode>\n");
        xml.push_str("            </cac:AlternativeConditionPrice>\n");
        xml.push_str("        </cac:PricingReference>\n");

        // Impuesto del ítem
        xml.push_str("        <cac:TaxTotal>\n");
        let _ = writeln!(
            &mut xml,
            "            <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
            params.moneda.codigo(),
            item.monto_igv
        );
        xml.push_str("            <cac:TaxSubtotal>\n");
        let _ = writeln!(
            &mut xml,
            "                <cbc:TaxableAmount currencyID=\"{}\">{:.2}</cbc:TaxableAmount>",
            params.moneda.codigo(),
            item.base_imponible_igv
        );
        let _ = writeln!(
            &mut xml,
            "                <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
            params.moneda.codigo(),
            item.monto_igv
        );
        xml.push_str("                <cac:TaxCategory>\n");
        let _ = writeln!(
            &mut xml,
            "                    <cbc:Percent>{:.2}</cbc:Percent>",
            item.porcentaje_igv
        );
        let _ = writeln!(
            &mut xml,
            "                    <cbc:TaxExemptionReasonCode>{}</cbc:TaxExemptionReasonCode>",
            item.tipo_afectacion_igv.codigo()
        );
        xml.push_str("                    <cac:TaxScheme>\n");
        let _ = writeln!(
            &mut xml,
            "                        <cbc:ID>{}</cbc:ID>",
            item.tipo_afectacion_igv.codigo_tributo_ubl()
        );
        xml.push_str("                        <cbc:Name>IGV</cbc:Name>\n");
        xml.push_str("                        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n");
        xml.push_str("                    </cac:TaxScheme>\n");
        xml.push_str("                </cac:TaxCategory>\n");
        xml.push_str("            </cac:TaxSubtotal>\n");
        xml.push_str("        </cac:TaxTotal>\n");

        // Datos del ítem
        xml.push_str("        <cac:Item>\n");
        let _ = writeln!(
            &mut xml,
            "            <cbc:Description><![CDATA[{}]]></cbc:Description>",
            item.descripcion
        );
        if let Some(ref cod) = item.codigo_producto {
            xml.push_str("            <cac:SellersItemIdentification>\n");
            let _ = writeln!(
                &mut xml,
                "                <cbc:ID>{}</cbc:ID>",
                cpe_xml_escape(cod)
            );
            xml.push_str("            </cac:SellersItemIdentification>\n");
        }
        xml.push_str("        </cac:Item>\n");

        // Valor unitario
        xml.push_str("        <cac:Price>\n");
        let _ = writeln!(
            &mut xml,
            "            <cbc:PriceAmount currencyID=\"{}\">{:.10}</cbc:PriceAmount>",
            params.moneda.codigo(),
            item.valor_unitario
        );
        xml.push_str("        </cac:Price>\n");
        xml.push_str("    </cac:InvoiceLine>\n");
    }

    xml.push_str("</Invoice>");
    Ok(xml)
}

impl CpeUblSerializador for CpeFactura {
    fn a_xml_ubl(&self) -> CpeResult<String> {
        cpe_construir_invoice_ubl_interno(CpeInvoiceParams {
            tipo_documento_codigo: "01",
            serie: &self.serie,
            correlativo: self.correlativo,
            fecha_emision: &self.fecha_emision,
            hora_emision: self.hora_emision.as_deref(),
            fecha_vencimiento: self.fecha_vencimiento.as_deref(),
            tipo_operacion: &self.tipo_operacion,
            moneda: &self.moneda,
            emisor: &self.emisor,
            receptor: &self.receptor,
            forma_pago: &self.forma_pago,
            _informacion_pago: self.informacion_pago.as_ref(),
            items: &self.items,
            totales: &self.totales,
            leyendas: &self.leyendas,
            documentos_referencia: &self.documentos_referencia,
        })
    }
}

impl CpeUblSerializador for CpeBoleta {
    fn a_xml_ubl(&self) -> CpeResult<String> {
        cpe_construir_invoice_ubl_interno(CpeInvoiceParams {
            tipo_documento_codigo: "03",
            serie: &self.serie,
            correlativo: self.correlativo,
            fecha_emision: &self.fecha_emision,
            hora_emision: self.hora_emision.as_deref(),
            fecha_vencimiento: self.fecha_vencimiento.as_deref(),
            tipo_operacion: &self.tipo_operacion,
            moneda: &self.moneda,
            emisor: &self.emisor,
            receptor: &self.receptor,
            forma_pago: &self.forma_pago,
            _informacion_pago: self.informacion_pago.as_ref(),
            items: &self.items,
            totales: &self.totales,
            leyendas: &self.leyendas,
            documentos_referencia: &self.documentos_referencia,
        })
    }
}

impl CpeUblSerializador for CpeNotaCredito {
    fn a_xml_ubl(&self) -> CpeResult<String> {
        let mut xml = String::with_capacity(4096);
        xml.push_str("<?xml version=\"1.0\" encoding=\"ISO-8859-1\" standalone=\"no\"?>\n");
        xml.push_str(
            "<CreditNote xmlns=\"urn:oasis:names:specification:ubl:schema:xsd:CreditNote-2\"\n",
        );
        xml.push_str("    xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\"\n");
        xml.push_str("    xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\"\n");
        xml.push_str("    xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"\n");
        xml.push_str("    xmlns:ext=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\">\n");

        // Espacio reservado para firma digital XMLDSig
        xml.push_str("    <ext:UBLExtensions>\n");
        xml.push_str("        <ext:UBLExtension>\n");
        xml.push_str("            <ext:ExtensionContent/>\n");
        xml.push_str("        </ext:UBLExtension>\n");
        xml.push_str("    </ext:UBLExtensions>\n");

        // Versión UBL y CustomizationID
        xml.push_str("    <cbc:UBLVersionID>2.1</cbc:UBLVersionID>\n");
        xml.push_str("    <cbc:CustomizationID>2.0</cbc:CustomizationID>\n");

        // Identificador (Serie-Correlativo)
        let _ = writeln!(
            &mut xml,
            "    <cbc:ID>{}-{:08}</cbc:ID>",
            cpe_xml_escape(&self.serie),
            self.correlativo
        );
        let _ = writeln!(
            &mut xml,
            "    <cbc:IssueDate>{}</cbc:IssueDate>",
            cpe_xml_escape(&self.fecha_emision)
        );
        if let Some(ref hora) = self.hora_emision {
            let _ = writeln!(
                &mut xml,
                "    <cbc:IssueTime>{}</cbc:IssueTime>",
                cpe_xml_escape(hora)
            );
        }

        // Leyendas (Catálogo 52)
        for leyenda in &self.leyendas {
            let _ = writeln!(
                &mut xml,
                "    <cbc:Note languageLocaleID=\"{}\"><![CDATA[{}]]></cbc:Note>",
                leyenda.codigo.codigo(),
                leyenda.texto
            );
        }

        // Moneda (Catálogo 02)
        let _ = writeln!(
            &mut xml,
            "    <cbc:DocumentCurrencyCode>{}</cbc:DocumentCurrencyCode>",
            self.moneda.codigo()
        );

        // Motivo / Discrepancia (Catálogo 09)
        xml.push_str("    <cac:DiscrepancyResponse>\n");
        let _ = writeln!(
            &mut xml,
            "        <cbc:ReferenceID>{}</cbc:ReferenceID>",
            cpe_xml_escape(&self.documento_modificado.serie_numero)
        );
        let _ = writeln!(
            &mut xml,
            "        <cbc:ResponseCode>{}</cbc:ResponseCode>",
            self.tipo_nota_credito.codigo()
        );
        let _ = writeln!(
            &mut xml,
            "        <cbc:Description><![CDATA[{}]]></cbc:Description>",
            self.sustento_motivo
        );
        xml.push_str("    </cac:DiscrepancyResponse>\n");

        // Referencia al comprobante modificado
        xml.push_str("    <cac:BillingReference>\n");
        xml.push_str("        <cac:InvoiceDocumentReference>\n");
        let _ = writeln!(
            &mut xml,
            "            <cbc:ID>{}</cbc:ID>",
            cpe_xml_escape(&self.documento_modificado.serie_numero)
        );
        let _ = writeln!(
            &mut xml,
            "            <cbc:DocumentTypeCode>{}</cbc:DocumentTypeCode>",
            self.documento_modificado.tipo_documento.codigo()
        );
        xml.push_str("        </cac:InvoiceDocumentReference>\n");
        xml.push_str("    </cac:BillingReference>\n");

        // Emisor (AccountingSupplierParty)
        xml.push_str("    <cac:AccountingSupplierParty>\n");
        xml.push_str("        <cac:Party>\n");
        xml.push_str("            <cac:PartyIdentification>\n");
        let _ = writeln!(
            &mut xml,
            "                <cbc:ID schemeID=\"6\">{}</cbc:ID>",
            cpe_xml_escape(&self.emisor.ruc)
        );
        xml.push_str("            </cac:PartyIdentification>\n");
        xml.push_str("            <cac:PartyName>\n");
        let nombre_comercial = self
            .emisor
            .nombre_comercial
            .as_deref()
            .unwrap_or(&self.emisor.razon_social);
        let _ = writeln!(
            &mut xml,
            "                <cbc:Name><![CDATA[{}]]></cbc:Name>",
            nombre_comercial
        );
        xml.push_str("            </cac:PartyName>\n");
        xml.push_str("            <cac:PartyLegalEntity>\n");
        let _ = writeln!(
            &mut xml,
            "                <cbc:RegistrationName><![CDATA[{}]]></cbc:RegistrationName>",
            self.emisor.razon_social
        );
        if let Some(ref dir) = self.emisor.direccion {
            xml.push_str("                <cac:RegistrationAddress>\n");
            if let Some(ref ubigeo) = dir.ubigeo {
                let _ = writeln!(
                    &mut xml,
                    "                    <cbc:ID>{}</cbc:ID>",
                    cpe_xml_escape(ubigeo)
                );
            }
            let _ = writeln!(
                &mut xml,
                "                    <cac:AddressLine><cbc:Line><![CDATA[{}]]></cbc:Line></cac:AddressLine>",
                dir.direccion_detallada
            );
            let _ = writeln!(
                &mut xml,
                "                    <cac:Country><cbc:IdentificationCode>{}</cbc:IdentificationCode></cac:Country>",
                cpe_xml_escape(&dir.codigo_pais)
            );
            xml.push_str("                </cac:RegistrationAddress>\n");
        }
        xml.push_str("            </cac:PartyLegalEntity>\n");
        xml.push_str("        </cac:Party>\n");
        xml.push_str("    </cac:AccountingSupplierParty>\n");

        // Receptor (AccountingCustomerParty)
        xml.push_str("    <cac:AccountingCustomerParty>\n");
        xml.push_str("        <cac:Party>\n");
        xml.push_str("            <cac:PartyIdentification>\n");
        let _ = writeln!(
            &mut xml,
            "                <cbc:ID schemeID=\"{}\">{}</cbc:ID>",
            self.receptor.tipo_documento.codigo(),
            cpe_xml_escape(&self.receptor.numero_documento)
        );
        xml.push_str("            </cac:PartyIdentification>\n");
        xml.push_str("            <cac:PartyLegalEntity>\n");
        let _ = writeln!(
            &mut xml,
            "                <cbc:RegistrationName><![CDATA[{}]]></cbc:RegistrationName>",
            self.receptor.razon_social
        );
        xml.push_str("            </cac:PartyLegalEntity>\n");
        xml.push_str("        </cac:Party>\n");
        xml.push_str("    </cac:AccountingCustomerParty>\n");

        // Totales de Impuestos (TaxTotal IGV)
        xml.push_str("    <cac:TaxTotal>\n");
        let _ = writeln!(
            &mut xml,
            "        <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
            self.moneda.codigo(),
            self.totales.total_igv
        );
        xml.push_str("        <cac:TaxSubtotal>\n");
        let _ = writeln!(
            &mut xml,
            "            <cbc:TaxableAmount currencyID=\"{}\">{:.2}</cbc:TaxableAmount>",
            self.moneda.codigo(),
            self.totales.total_operaciones_gravadas
        );
        let _ = writeln!(
            &mut xml,
            "            <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
            self.moneda.codigo(),
            self.totales.total_igv
        );
        xml.push_str("            <cac:TaxCategory>\n");
        xml.push_str("                <cac:TaxScheme>\n");
        xml.push_str("                    <cbc:ID>1000</cbc:ID>\n");
        xml.push_str("                    <cbc:Name>IGV</cbc:Name>\n");
        xml.push_str("                    <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n");
        xml.push_str("                </cac:TaxScheme>\n");
        xml.push_str("            </cac:TaxCategory>\n");
        xml.push_str("        </cac:TaxSubtotal>\n");
        xml.push_str("    </cac:TaxTotal>\n");

        // Totales Legales (LegalMonetaryTotal)
        xml.push_str("    <cac:LegalMonetaryTotal>\n");
        let _ = writeln!(
            &mut xml,
            "        <cbc:LineExtensionAmount currencyID=\"{}\">{:.2}</cbc:LineExtensionAmount>",
            self.moneda.codigo(),
            self.totales.total_operaciones_gravadas
        );
        let _ = writeln!(
            &mut xml,
            "        <cbc:TaxInclusiveAmount currencyID=\"{}\">{:.2}</cbc:TaxInclusiveAmount>",
            self.moneda.codigo(),
            self.totales.importe_total_pagar
        );
        let _ = writeln!(
            &mut xml,
            "        <cbc:PayableAmount currencyID=\"{}\">{:.2}</cbc:PayableAmount>",
            self.moneda.codigo(),
            self.totales.importe_total_pagar
        );
        xml.push_str("    </cac:LegalMonetaryTotal>\n");

        // Líneas de la Nota de Crédito (CreditNoteLine)
        for item in &self.items {
            xml.push_str("    <cac:CreditNoteLine>\n");
            let _ = writeln!(&mut xml, "        <cbc:ID>{}</cbc:ID>", item.numero_orden);
            let _ = writeln!(
                &mut xml,
                "        <cbc:CreditedQuantity unitCode=\"{}\">{:.10}</cbc:CreditedQuantity>",
                cpe_xml_escape(&item.unidad_medida),
                item.cantidad
            );
            let _ = writeln!(
                &mut xml,
                "        <cbc:LineExtensionAmount currencyID=\"{}\">{:.2}</cbc:LineExtensionAmount>",
                self.moneda.codigo(),
                item.total_valor_venta
            );

            // Precios
            xml.push_str("        <cac:PricingReference>\n");
            xml.push_str("            <cac:AlternativeConditionPrice>\n");
            let _ = writeln!(
                &mut xml,
                "                <cbc:PriceAmount currencyID=\"{}\">{:.10}</cbc:PriceAmount>",
                self.moneda.codigo(),
                item.precio_unitario
            );
            xml.push_str("                <cbc:PriceTypeCode>01</cbc:PriceTypeCode>\n");
            xml.push_str("            </cac:AlternativeConditionPrice>\n");
            xml.push_str("        </cac:PricingReference>\n");

            // Impuestos del ítem
            xml.push_str("        <cac:TaxTotal>\n");
            let _ = writeln!(
                &mut xml,
                "            <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
                self.moneda.codigo(),
                item.monto_igv
            );
            xml.push_str("            <cac:TaxSubtotal>\n");
            let _ = writeln!(
                &mut xml,
                "                <cbc:TaxableAmount currencyID=\"{}\">{:.2}</cbc:TaxableAmount>",
                self.moneda.codigo(),
                item.base_imponible_igv
            );
            let _ = writeln!(
                &mut xml,
                "                <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
                self.moneda.codigo(),
                item.monto_igv
            );
            xml.push_str("                <cac:TaxCategory>\n");
            let _ = writeln!(
                &mut xml,
                "                    <cbc:Percent>{:.2}</cbc:Percent>",
                item.porcentaje_igv
            );
            let _ = writeln!(
                &mut xml,
                "                    <cbc:TaxExemptionReasonCode>{}</cbc:TaxExemptionReasonCode>",
                item.tipo_afectacion_igv.codigo()
            );
            xml.push_str("                    <cac:TaxScheme>\n");
            let _ = writeln!(
                &mut xml,
                "                        <cbc:ID>{}</cbc:ID>",
                item.tipo_afectacion_igv.codigo_tributo_ubl()
            );
            xml.push_str("                        <cbc:Name>IGV</cbc:Name>\n");
            xml.push_str("                        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n");
            xml.push_str("                    </cac:TaxScheme>\n");
            xml.push_str("                </cac:TaxCategory>\n");
            xml.push_str("            </cac:TaxSubtotal>\n");
            xml.push_str("        </cac:TaxTotal>\n");

            // Datos del ítem
            xml.push_str("        <cac:Item>\n");
            let _ = writeln!(
                &mut xml,
                "            <cbc:Description><![CDATA[{}]]></cbc:Description>",
                item.descripcion
            );
            if let Some(ref cod) = item.codigo_producto {
                xml.push_str("            <cac:SellersItemIdentification>\n");
                let _ = writeln!(
                    &mut xml,
                    "                <cbc:ID>{}</cbc:ID>",
                    cpe_xml_escape(cod)
                );
                xml.push_str("            </cac:SellersItemIdentification>\n");
            }
            xml.push_str("        </cac:Item>\n");

            // Valor unitario
            xml.push_str("        <cac:Price>\n");
            let _ = writeln!(
                &mut xml,
                "            <cbc:PriceAmount currencyID=\"{}\">{:.10}</cbc:PriceAmount>",
                self.moneda.codigo(),
                item.valor_unitario
            );
            xml.push_str("        </cac:Price>\n");
            xml.push_str("    </cac:CreditNoteLine>\n");
        }

        xml.push_str("</CreditNote>");
        Ok(xml)
    }
}

impl CpeUblSerializador for CpeNotaDebito {
    fn a_xml_ubl(&self) -> CpeResult<String> {
        let mut xml = String::with_capacity(4096);
        xml.push_str("<?xml version=\"1.0\" encoding=\"ISO-8859-1\" standalone=\"no\"?>\n");
        xml.push_str(
            "<DebitNote xmlns=\"urn:oasis:names:specification:ubl:schema:xsd:DebitNote-2\"\n",
        );
        xml.push_str("    xmlns:cac=\"urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2\"\n");
        xml.push_str("    xmlns:cbc=\"urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2\"\n");
        xml.push_str("    xmlns:ds=\"http://www.w3.org/2000/09/xmldsig#\"\n");
        xml.push_str("    xmlns:ext=\"urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2\">\n");

        // Espacio reservado para firma digital XMLDSig
        xml.push_str("    <ext:UBLExtensions>\n");
        xml.push_str("        <ext:UBLExtension>\n");
        xml.push_str("            <ext:ExtensionContent/>\n");
        xml.push_str("        </ext:UBLExtension>\n");
        xml.push_str("    </ext:UBLExtensions>\n");

        // Versión UBL y CustomizationID
        xml.push_str("    <cbc:UBLVersionID>2.1</cbc:UBLVersionID>\n");
        xml.push_str("    <cbc:CustomizationID>2.0</cbc:CustomizationID>\n");

        // Identificador (Serie-Correlativo)
        let _ = writeln!(
            &mut xml,
            "    <cbc:ID>{}-{:08}</cbc:ID>",
            cpe_xml_escape(&self.serie),
            self.correlativo
        );
        let _ = writeln!(
            &mut xml,
            "    <cbc:IssueDate>{}</cbc:IssueDate>",
            cpe_xml_escape(&self.fecha_emision)
        );
        if let Some(ref hora) = self.hora_emision {
            let _ = writeln!(
                &mut xml,
                "    <cbc:IssueTime>{}</cbc:IssueTime>",
                cpe_xml_escape(hora)
            );
        }

        // Leyendas (Catálogo 52)
        for leyenda in &self.leyendas {
            let _ = writeln!(
                &mut xml,
                "    <cbc:Note languageLocaleID=\"{}\"><![CDATA[{}]]></cbc:Note>",
                leyenda.codigo.codigo(),
                leyenda.texto
            );
        }

        // Moneda (Catálogo 02)
        let _ = writeln!(
            &mut xml,
            "    <cbc:DocumentCurrencyCode>{}</cbc:DocumentCurrencyCode>",
            self.moneda.codigo()
        );

        // Motivo / Discrepancia (Catálogo 10)
        xml.push_str("    <cac:DiscrepancyResponse>\n");
        let _ = writeln!(
            &mut xml,
            "        <cbc:ReferenceID>{}</cbc:ReferenceID>",
            cpe_xml_escape(&self.documento_modificado.serie_numero)
        );
        let _ = writeln!(
            &mut xml,
            "        <cbc:ResponseCode>{}</cbc:ResponseCode>",
            self.tipo_nota_debito.codigo()
        );
        let _ = writeln!(
            &mut xml,
            "        <cbc:Description><![CDATA[{}]]></cbc:Description>",
            self.sustento_motivo
        );
        xml.push_str("    </cac:DiscrepancyResponse>\n");

        // Referencia al comprobante modificado
        xml.push_str("    <cac:BillingReference>\n");
        xml.push_str("        <cac:InvoiceDocumentReference>\n");
        let _ = writeln!(
            &mut xml,
            "            <cbc:ID>{}</cbc:ID>",
            cpe_xml_escape(&self.documento_modificado.serie_numero)
        );
        let _ = writeln!(
            &mut xml,
            "            <cbc:DocumentTypeCode>{}</cbc:DocumentTypeCode>",
            self.documento_modificado.tipo_documento.codigo()
        );
        xml.push_str("        </cac:InvoiceDocumentReference>\n");
        xml.push_str("    </cac:BillingReference>\n");

        // Emisor (AccountingSupplierParty)
        xml.push_str("    <cac:AccountingSupplierParty>\n");
        xml.push_str("        <cac:Party>\n");
        xml.push_str("            <cac:PartyIdentification>\n");
        let _ = writeln!(
            &mut xml,
            "                <cbc:ID schemeID=\"6\">{}</cbc:ID>",
            cpe_xml_escape(&self.emisor.ruc)
        );
        xml.push_str("            </cac:PartyIdentification>\n");
        xml.push_str("            <cac:PartyName>\n");
        let nombre_comercial = self
            .emisor
            .nombre_comercial
            .as_deref()
            .unwrap_or(&self.emisor.razon_social);
        let _ = writeln!(
            &mut xml,
            "                <cbc:Name><![CDATA[{}]]></cbc:Name>",
            nombre_comercial
        );
        xml.push_str("            </cac:PartyName>\n");
        xml.push_str("            <cac:PartyLegalEntity>\n");
        let _ = writeln!(
            &mut xml,
            "                <cbc:RegistrationName><![CDATA[{}]]></cbc:RegistrationName>",
            self.emisor.razon_social
        );
        if let Some(ref dir) = self.emisor.direccion {
            xml.push_str("                <cac:RegistrationAddress>\n");
            if let Some(ref ubigeo) = dir.ubigeo {
                let _ = writeln!(
                    &mut xml,
                    "                    <cbc:ID>{}</cbc:ID>",
                    cpe_xml_escape(ubigeo)
                );
            }
            let _ = writeln!(
                &mut xml,
                "                    <cac:AddressLine><cbc:Line><![CDATA[{}]]></cbc:Line></cac:AddressLine>",
                dir.direccion_detallada
            );
            let _ = writeln!(
                &mut xml,
                "                    <cac:Country><cbc:IdentificationCode>{}</cbc:IdentificationCode></cac:Country>",
                cpe_xml_escape(&dir.codigo_pais)
            );
            xml.push_str("                </cac:RegistrationAddress>\n");
        }
        xml.push_str("            </cac:PartyLegalEntity>\n");
        xml.push_str("        </cac:Party>\n");
        xml.push_str("    </cac:AccountingSupplierParty>\n");

        // Receptor (AccountingCustomerParty)
        xml.push_str("    <cac:AccountingCustomerParty>\n");
        xml.push_str("        <cac:Party>\n");
        xml.push_str("            <cac:PartyIdentification>\n");
        let _ = writeln!(
            &mut xml,
            "                <cbc:ID schemeID=\"{}\">{}</cbc:ID>",
            self.receptor.tipo_documento.codigo(),
            cpe_xml_escape(&self.receptor.numero_documento)
        );
        xml.push_str("            </cac:PartyIdentification>\n");
        xml.push_str("            <cac:PartyLegalEntity>\n");
        let _ = writeln!(
            &mut xml,
            "                <cbc:RegistrationName><![CDATA[{}]]></cbc:RegistrationName>",
            self.receptor.razon_social
        );
        xml.push_str("            </cac:PartyLegalEntity>\n");
        xml.push_str("        </cac:Party>\n");
        xml.push_str("    </cac:AccountingCustomerParty>\n");

        // Totales de Impuestos (TaxTotal IGV)
        xml.push_str("    <cac:TaxTotal>\n");
        let _ = writeln!(
            &mut xml,
            "        <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
            self.moneda.codigo(),
            self.totales.total_igv
        );
        xml.push_str("        <cac:TaxSubtotal>\n");
        let _ = writeln!(
            &mut xml,
            "            <cbc:TaxableAmount currencyID=\"{}\">{:.2}</cbc:TaxableAmount>",
            self.moneda.codigo(),
            self.totales.total_operaciones_gravadas
        );
        let _ = writeln!(
            &mut xml,
            "            <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
            self.moneda.codigo(),
            self.totales.total_igv
        );
        xml.push_str("            <cac:TaxCategory>\n");
        xml.push_str("                <cac:TaxScheme>\n");
        xml.push_str("                    <cbc:ID>1000</cbc:ID>\n");
        xml.push_str("                    <cbc:Name>IGV</cbc:Name>\n");
        xml.push_str("                    <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n");
        xml.push_str("                </cac:TaxScheme>\n");
        xml.push_str("            </cac:TaxCategory>\n");
        xml.push_str("        </cac:TaxSubtotal>\n");
        xml.push_str("    </cac:TaxTotal>\n");

        // Totales Requeridos (RequestedMonetaryTotal)
        xml.push_str("    <cac:RequestedMonetaryTotal>\n");
        let _ = writeln!(
            &mut xml,
            "        <cbc:LineExtensionAmount currencyID=\"{}\">{:.2}</cbc:LineExtensionAmount>",
            self.moneda.codigo(),
            self.totales.total_operaciones_gravadas
        );
        let _ = writeln!(
            &mut xml,
            "        <cbc:TaxInclusiveAmount currencyID=\"{}\">{:.2}</cbc:TaxInclusiveAmount>",
            self.moneda.codigo(),
            self.totales.importe_total_pagar
        );
        let _ = writeln!(
            &mut xml,
            "        <cbc:PayableAmount currencyID=\"{}\">{:.2}</cbc:PayableAmount>",
            self.moneda.codigo(),
            self.totales.importe_total_pagar
        );
        xml.push_str("    </cac:RequestedMonetaryTotal>\n");

        // Líneas de la Nota de Débito (DebitNoteLine)
        for item in &self.items {
            xml.push_str("    <cac:DebitNoteLine>\n");
            let _ = writeln!(&mut xml, "        <cbc:ID>{}</cbc:ID>", item.numero_orden);
            let _ = writeln!(
                &mut xml,
                "        <cbc:DebitedQuantity unitCode=\"{}\">{:.10}</cbc:DebitedQuantity>",
                cpe_xml_escape(&item.unidad_medida),
                item.cantidad
            );
            let _ = writeln!(
                &mut xml,
                "        <cbc:LineExtensionAmount currencyID=\"{}\">{:.2}</cbc:LineExtensionAmount>",
                self.moneda.codigo(),
                item.total_valor_venta
            );

            // Precios
            xml.push_str("        <cac:PricingReference>\n");
            xml.push_str("            <cac:AlternativeConditionPrice>\n");
            let _ = writeln!(
                &mut xml,
                "                <cbc:PriceAmount currencyID=\"{}\">{:.10}</cbc:PriceAmount>",
                self.moneda.codigo(),
                item.precio_unitario
            );
            xml.push_str("                <cbc:PriceTypeCode>01</cbc:PriceTypeCode>\n");
            xml.push_str("            </cac:AlternativeConditionPrice>\n");
            xml.push_str("        </cac:PricingReference>\n");

            // Impuestos del ítem
            xml.push_str("        <cac:TaxTotal>\n");
            let _ = writeln!(
                &mut xml,
                "            <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
                self.moneda.codigo(),
                item.monto_igv
            );
            xml.push_str("            <cac:TaxSubtotal>\n");
            let _ = writeln!(
                &mut xml,
                "                <cbc:TaxableAmount currencyID=\"{}\">{:.2}</cbc:TaxableAmount>",
                self.moneda.codigo(),
                item.base_imponible_igv
            );
            let _ = writeln!(
                &mut xml,
                "                <cbc:TaxAmount currencyID=\"{}\">{:.2}</cbc:TaxAmount>",
                self.moneda.codigo(),
                item.monto_igv
            );
            xml.push_str("                <cac:TaxCategory>\n");
            let _ = writeln!(
                &mut xml,
                "                    <cbc:Percent>{:.2}</cbc:Percent>",
                item.porcentaje_igv
            );
            let _ = writeln!(
                &mut xml,
                "                    <cbc:TaxExemptionReasonCode>{}</cbc:TaxExemptionReasonCode>",
                item.tipo_afectacion_igv.codigo()
            );
            xml.push_str("                    <cac:TaxScheme>\n");
            let _ = writeln!(
                &mut xml,
                "                        <cbc:ID>{}</cbc:ID>",
                item.tipo_afectacion_igv.codigo_tributo_ubl()
            );
            xml.push_str("                        <cbc:Name>IGV</cbc:Name>\n");
            xml.push_str("                        <cbc:TaxTypeCode>VAT</cbc:TaxTypeCode>\n");
            xml.push_str("                    </cac:TaxScheme>\n");
            xml.push_str("                </cac:TaxCategory>\n");
            xml.push_str("            </cac:TaxSubtotal>\n");
            xml.push_str("        </cac:TaxTotal>\n");

            // Datos del ítem
            xml.push_str("        <cac:Item>\n");
            let _ = writeln!(
                &mut xml,
                "            <cbc:Description><![CDATA[{}]]></cbc:Description>",
                item.descripcion
            );
            if let Some(ref cod) = item.codigo_producto {
                xml.push_str("            <cac:SellersItemIdentification>\n");
                let _ = writeln!(
                    &mut xml,
                    "                <cbc:ID>{}</cbc:ID>",
                    cpe_xml_escape(cod)
                );
                xml.push_str("            </cac:SellersItemIdentification>\n");
            }
            xml.push_str("        </cac:Item>\n");

            // Valor unitario
            xml.push_str("        <cac:Price>\n");
            let _ = writeln!(
                &mut xml,
                "            <cbc:PriceAmount currencyID=\"{}\">{:.10}</cbc:PriceAmount>",
                self.moneda.codigo(),
                item.valor_unitario
            );
            xml.push_str("        </cac:Price>\n");
            xml.push_str("    </cac:DebitNoteLine>\n");
        }

        xml.push_str("</DebitNote>");
        Ok(xml)
    }
}
