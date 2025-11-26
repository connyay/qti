use crate::error::{QtiError, Result};
use crate::schema::QtiSchema;
use std::path::Path;
use xmltree::Element;

/// Validates QTI XML against schema
pub struct Validator {
    schema: QtiSchema,
}

impl Validator {
    pub fn new() -> Self {
        Self {
            schema: QtiSchema::qti_1_2_schema(),
        }
    }

    pub fn from_xsd_file(path: impl AsRef<Path>) -> Result<Self> {
        Ok(Self {
            schema: QtiSchema::from_xsd_file(path)?,
        })
    }

    pub fn validate_xml(&self, xml: &str) -> Result<()> {
        let element = self.parse_xml(xml)?;
        self.validate_element(&element)
    }

    pub fn validate_element(&self, element: &Element) -> Result<()> {
        self.schema.validate(element)
    }

    fn parse_xml(&self, xml: &str) -> Result<Element> {
        Element::parse(xml.as_bytes())
            .map_err(|e| QtiError::XmlError(format!("Failed to parse XML: {}", e)))
    }

    pub fn validate_completeness(&self, element: &Element) -> Result<()> {
        if element.name != "questestinterop" {
            return Err(QtiError::ValidationError(format!(
                "Root element must be 'questestinterop', found '{}'",
                element.name
            )));
        }

        let assessment = element
            .get_child("assessment")
            .ok_or_else(|| QtiError::ValidationError("Missing 'assessment' element".to_string()))?;

        let section = assessment
            .get_child("section")
            .ok_or_else(|| QtiError::ValidationError("Missing 'section' element".to_string()))?;

        let items: Vec<&Element> = section
            .children
            .iter()
            .filter_map(|node| {
                if let xmltree::XMLNode::Element(elem) = node {
                    if elem.name == "item" {
                        return Some(elem);
                    }
                }
                None
            })
            .collect();

        if items.is_empty() {
            return Err(QtiError::ValidationError(
                "Assessment must contain at least one item".to_string(),
            ));
        }

        // Validate each item
        for item in items {
            self.validate_item(item)?;
        }

        Ok(())
    }

    fn validate_item(&self, item: &Element) -> Result<()> {
        if !item.attributes.contains_key("ident") {
            return Err(QtiError::ValidationError(
                "Item missing required 'ident' attribute".to_string(),
            ));
        }

        let presentation = item.get_child("presentation").ok_or_else(|| {
            QtiError::ValidationError("Item missing 'presentation' element".to_string())
        })?;

        let _resprocessing = item.get_child("resprocessing").ok_or_else(|| {
            QtiError::ValidationError("Item missing 'resprocessing' element".to_string())
        })?;

        let material = presentation.get_child("material").ok_or_else(|| {
            QtiError::ValidationError("Presentation missing 'material' element".to_string())
        })?;

        let mattext = material.get_child("mattext").ok_or_else(|| {
            QtiError::ValidationError("Material missing 'mattext' element".to_string())
        })?;

        if mattext.get_text().is_none()
            || mattext
                .get_text()
                .map(|s| s.to_string())
                .unwrap_or_default()
                .trim()
                .is_empty()
        {
            return Err(QtiError::ValidationError(
                "Question text cannot be empty".to_string(),
            ));
        }

        let has_response = presentation.get_child("response_lid").is_some()
            || presentation.get_child("response_str").is_some()
            || presentation.get_child("response_num").is_some()
            || presentation.get_child("response_grp").is_some();

        if !has_response {
            return Err(QtiError::ValidationError(
                "Item presentation missing response element".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::Generator;
    use crate::types::{Assessment, Choice, Question, QuestionType};

    #[test]
    fn test_validate_generated_xml() {
        let mut assessment = Assessment::new("Test Assessment");

        let mut question = Question::new(
            "What is 2 + 2?",
            QuestionType::MultipleChoice {
                choices: vec![
                    Choice::new("3", false),
                    Choice::new("4", true),
                    Choice::new("5", false),
                ],
                shuffle: false,
            },
        );
        question.points = 1.0;
        assessment.questions.push(question);

        let generator = Generator::new();
        let xml = generator
            .generate(&assessment)
            .expect("Should generate XML");

        let validator = Validator::new();
        validator
            .validate_xml(&xml)
            .expect("Generated XML should be valid");

        let element = Element::parse(xml.as_bytes()).expect("Should parse XML");
        validator
            .validate_completeness(&element)
            .expect("Should be complete");
    }
}
