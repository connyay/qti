use crate::error::{QtiError, Result};
use crate::generator::{Generator, QtiPackage};
use crate::types::Assessment;
use crate::validator::Validator;
use std::io::{Seek, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::{CompressionMethod, ZipWriter};

/// Exports assessments to QTI packages
pub struct Exporter {
    generator: Generator,
    validator: Validator,
    validate_before_export: bool,
}

impl Exporter {
    pub fn new() -> Self {
        Self {
            generator: Generator::new(),
            validator: Validator::new(),
            validate_before_export: true,
        }
    }

    pub fn with_canvas_extensions(mut self) -> Self {
        self.generator = self.generator.with_canvas_extensions();
        self
    }

    pub fn skip_validation(mut self) -> Self {
        self.validate_before_export = false;
        self
    }

    /// Export assessment to a QTI zip file
    pub fn export_to_file(&self, assessment: &Assessment, path: impl AsRef<Path>) -> Result<()> {
        let file = std::fs::File::create(path)?;
        self.export_to_writer(assessment, file)
    }

    /// Export assessment to a writer
    pub fn export_to_writer<W: Write + Seek>(
        &self,
        assessment: &Assessment,
        writer: W,
    ) -> Result<()> {
        // Generate the package
        let package = self.generator.generate_package(assessment)?;

        // Validate if enabled
        if self.validate_before_export {
            self.validator.validate_xml(&package.assessment_xml)?;
        }

        // Create zip archive
        self.create_zip(package, writer)?;

        Ok(())
    }

    fn create_zip<W: Write + Seek>(&self, package: QtiPackage, writer: W) -> Result<()> {
        let mut zip = ZipWriter::new(writer);

        let options = FileOptions::default()
            .compression_method(CompressionMethod::Deflated)
            .unix_permissions(0o644);

        zip.start_file("imsmanifest.xml", options)?;
        zip.write_all(package.manifest_xml.as_bytes())?;

        let ident = self.extract_assessment_ident(&package.assessment_xml)?;
        zip.start_file(format!("{}.xml", ident), options)?;
        zip.write_all(package.assessment_xml.as_bytes())?;

        for resource in package.resources {
            zip.start_file(&resource.filename, options)?;
            zip.write_all(&resource.content)?;
        }

        zip.finish()?;
        Ok(())
    }

    /// Extract assessment identifier from XML by parsing it (somewhat inefficient but simple)
    fn extract_assessment_ident(&self, xml: &str) -> Result<String> {
        let element = xmltree::Element::parse(xml.as_bytes())
            .map_err(|e| QtiError::XmlError(format!("Failed to parse XML: {}", e)))?;

        let assessment = element
            .get_child("assessment")
            .ok_or_else(|| QtiError::XmlError("No assessment element found".to_string()))?;

        assessment
            .attributes
            .get("ident")
            .cloned()
            .ok_or_else(|| QtiError::XmlError("Assessment missing ident attribute".to_string()))
    }

    /// Export assessment to XML string (without packaging)
    pub fn export_to_xml(&self, assessment: &Assessment) -> Result<String> {
        let xml = self.generator.generate(assessment)?;

        if self.validate_before_export {
            self.validator.validate_xml(&xml)?;
        }

        Ok(xml)
    }

    /// Export assessment and return package contents in memory
    pub fn export_to_memory(&self, assessment: &Assessment) -> Result<Vec<u8>> {
        let mut buffer = std::io::Cursor::new(Vec::new());
        self.export_to_writer(assessment, &mut buffer)?;
        Ok(buffer.into_inner())
    }
}

impl Default for Exporter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Choice, Question, QuestionType};
    use tempfile::NamedTempFile;

    #[test]
    fn test_export_to_file() {
        // Create assessment
        let mut assessment = Assessment::new("Test Quiz");
        assessment.identifier = "quiz_123".to_string();

        let question = Question::new(
            "What is the capital of France?",
            QuestionType::MultipleChoice {
                choices: vec![
                    Choice::new("London", false),
                    Choice::new("Paris", true),
                    Choice::new("Berlin", false),
                    Choice::new("Madrid", false),
                ],
                shuffle: true,
            },
        );
        assessment.questions.push(question);

        // Export to temp file
        let temp_file = NamedTempFile::new().expect("Should create temp file");
        let exporter = Exporter::new();
        exporter
            .export_to_file(&assessment, temp_file.path())
            .expect("Should export to file");

        // Verify file exists and is not empty
        let metadata = std::fs::metadata(temp_file.path()).expect("Should get file metadata");
        assert!(metadata.len() > 0, "Exported file should not be empty");

        // Verify it's a valid zip file
        let file = std::fs::File::open(temp_file.path()).expect("Should open file");
        let archive = zip::ZipArchive::new(file).expect("Should be valid zip file");
        assert!(
            archive.len() >= 2,
            "Should contain at least manifest and assessment"
        );
    }

    #[test]
    fn test_export_to_xml() {
        let mut assessment = Assessment::new("XML Test");
        assessment.questions.push(Question::new(
            "Test question",
            QuestionType::ShortAnswer {
                answers: vec![crate::types::AcceptableAnswer::new("answer")],
                case_sensitive: false,
            },
        ));

        let exporter = Exporter::new();
        let xml = exporter
            .export_to_xml(&assessment)
            .expect("Should export to XML");

        assert!(xml.contains("<?xml version"));
        assert!(xml.contains("questestinterop"));
        assert!(xml.contains("assessment"));
        assert!(xml.contains("Test question"));
    }
}
