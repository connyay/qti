use crate::builder::QtiBuilder;
use crate::error::{QtiError, Result};
use crate::types::Assessment;
use std::io::Write;
use xmltree::Element;

/// Main generator that converts Assessment to QTI XML
pub struct Generator {
    builder: QtiBuilder,
    pretty_print: bool,
}

impl Generator {
    pub fn new() -> Self {
        Self {
            builder: QtiBuilder::new(),
            pretty_print: true,
        }
    }

    pub fn with_canvas_extensions(mut self) -> Self {
        self.builder = self.builder.with_canvas_extensions();
        self
    }

    pub fn pretty_print(mut self, enabled: bool) -> Self {
        self.pretty_print = enabled;
        self
    }

    /// Generate QTI XML string from Assessment
    pub fn generate(&self, assessment: &Assessment) -> Result<String> {
        let root = self.builder.build_questestinterop(assessment)?;
        self.element_to_xml_string(&root)
    }

    /// Generate QTI XML and write to a writer
    pub fn generate_to_writer<W: Write>(&self, assessment: &Assessment, writer: W) -> Result<()> {
        let root = self.builder.build_questestinterop(assessment)?;
        self.write_element(writer, &root)?;
        Ok(())
    }

    fn element_to_xml_string(&self, element: &Element) -> Result<String> {
        let mut buf = Vec::new();
        self.write_element(&mut buf, element)?;
        String::from_utf8(buf).map_err(|e| QtiError::Utf8Error(e.utf8_error()))
    }

    fn write_element<W: Write>(&self, mut writer: W, element: &Element) -> Result<()> {
        let config = xmltree::EmitterConfig::new()
            .perform_indent(self.pretty_print)
            .indent_string("  ")
            .write_document_declaration(true);

        element.write_with_config(&mut writer, config)?;
        Ok(())
    }

    pub fn generate_package(&self, assessment: &Assessment) -> Result<QtiPackage> {
        let xml = self.generate(assessment)?;
        let manifest = self.generate_manifest(assessment)?;

        Ok(QtiPackage {
            assessment_xml: xml,
            manifest_xml: manifest,
            resources: Vec::new(),
        })
    }

    fn generate_manifest(&self, assessment: &Assessment) -> Result<String> {
        let mut manifest = Element::new("manifest");

        manifest.attributes.insert(
            "identifier".to_string(),
            format!("{}_manifest", assessment.identifier),
        );
        manifest.attributes.insert(
            "xmlns".to_string(),
            "http://www.imsglobal.org/xsd/imsccv1p1/imscp_v1p1".to_string(),
        );
        manifest.attributes.insert(
            "xmlns:lom".to_string(),
            "http://ltsc.ieee.org/xsd/imsccv1p1/LOM/resource".to_string(),
        );
        manifest.attributes.insert(
            "xmlns:lomimscc".to_string(),
            "http://ltsc.ieee.org/xsd/imsccv1p1/LOM/manifest".to_string(),
        );
        manifest.attributes.insert(
            "xmlns:xsi".to_string(),
            "http://www.w3.org/2001/XMLSchema-instance".to_string(),
        );
        manifest.attributes.insert("xsi:schemaLocation".to_string(),
            "http://www.imsglobal.org/xsd/imsccv1p1/imscp_v1p1 http://www.imsglobal.org/profile/cc/ccv1p1/ccv1p1_imscp_v1p2_v1p0.xsd".to_string());

        let mut metadata = Element::new("metadata");
        let mut schema = Element::new("schema");
        schema
            .children
            .push(xmltree::XMLNode::Text("IMS Content".to_string()));
        metadata.children.push(xmltree::XMLNode::Element(schema));
        let mut schemaversion = Element::new("schemaversion");
        schemaversion
            .children
            .push(xmltree::XMLNode::Text("1.1.3".to_string()));
        metadata
            .children
            .push(xmltree::XMLNode::Element(schemaversion));
        manifest.children.push(xmltree::XMLNode::Element(metadata));

        let organizations = Element::new("organizations");
        manifest
            .children
            .push(xmltree::XMLNode::Element(organizations));

        let mut resources = Element::new("resources");

        let mut resource = Element::new("resource");
        resource.attributes.insert(
            "identifier".to_string(),
            format!("{}_resource", assessment.identifier),
        );
        resource
            .attributes
            .insert("type".to_string(), "imsqti_xmlv1p2".to_string());
        resource
            .attributes
            .insert("href".to_string(), format!("{}.xml", assessment.identifier));

        let mut file = Element::new("file");
        file.attributes
            .insert("href".to_string(), format!("{}.xml", assessment.identifier));
        resource.children.push(xmltree::XMLNode::Element(file));

        resources.children.push(xmltree::XMLNode::Element(resource));
        manifest.children.push(xmltree::XMLNode::Element(resources));

        self.element_to_xml_string(&manifest)
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a complete QTI package
pub struct QtiPackage {
    pub assessment_xml: String,
    pub manifest_xml: String,
    pub resources: Vec<QtiResource>,
}

pub struct QtiResource {
    pub filename: String,
    pub content: Vec<u8>,
}
