use thiserror::Error;

#[derive(Error, Debug)]
pub enum QtiError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid question format at line {line}: {message}")]
    InvalidFormat { line: usize, message: String },

    #[error("XML generation error: {0}")]
    XmlError(String),

    #[error("Schema validation error: {0}")]
    ValidationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("XML parsing error: {0}")]
    XmlParseError(#[from] quick_xml::Error),

    #[error("XML tree error: {0}")]
    XmlTreeError(#[from] xmltree::Error),

    #[error("Zip error: {0}")]
    ZipError(#[from] zip::result::ZipError),

    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, QtiError>;
