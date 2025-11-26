use crate::error::{QtiError, Result};
use std::path::Path;
use xmltree::{Element, XMLNode};

/// XSD-based schema definitions for QTI 1.2
pub struct QtiSchema {
    /// Root element name
    pub root: String,
    /// Valid elements and their attributes
    pub elements: Vec<ElementDef>,
}

#[derive(Debug, Clone)]
pub struct ElementDef {
    pub name: String,
    pub attributes: Vec<AttributeDef>,
    pub required: bool,
    pub children: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AttributeDef {
    pub name: String,
    pub required: bool,
    pub values: Option<Vec<String>>, // Enumerated values if applicable
}

impl QtiSchema {
    pub fn from_xsd_file(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_xsd_string(&content)
    }

    /// Currently returns hardcoded schema; full XSD parsing not yet implemented
    pub fn from_xsd_string(_xsd: &str) -> Result<Self> {
        Ok(Self::qti_1_2_schema())
    }

    /// QTI 1.2 schema definitions based on ims_qtiasiv1p2p1.xsd specification
    pub fn qti_1_2_schema() -> Self {
        Self {
            root: "questestinterop".to_string(),
            elements: vec![
                ElementDef {
                    name: "questestinterop".to_string(),
                    attributes: vec![
                        AttributeDef {
                            name: "xmlns".to_string(),
                            required: true,
                            values: Some(vec![
                                "http://www.imsglobal.org/xsd/ims_qtiasiv1p2".to_string()
                            ]),
                        },
                        AttributeDef {
                            name: "xmlns:xsi".to_string(),
                            required: true,
                            values: Some(vec![
                                "http://www.w3.org/2001/XMLSchema-instance".to_string()
                            ]),
                        },
                    ],
                    required: true,
                    children: vec!["assessment".to_string()],
                },
                ElementDef {
                    name: "qtimetadata".to_string(),
                    attributes: vec![],
                    required: false,
                    children: vec!["qtimetadatafield".to_string()],
                },
                ElementDef {
                    name: "itemmetadata".to_string(),
                    attributes: vec![],
                    required: false,
                    children: vec!["qtimetadatafield".to_string()],
                },
                ElementDef {
                    name: "qtimetadatafield".to_string(),
                    attributes: vec![],
                    required: false,
                    children: vec!["fieldlabel".to_string(), "fieldentry".to_string()],
                },
                ElementDef {
                    name: "fieldlabel".to_string(),
                    attributes: vec![],
                    required: false,
                    children: vec![],
                },
                ElementDef {
                    name: "fieldentry".to_string(),
                    attributes: vec![],
                    required: false,
                    children: vec![],
                },
                ElementDef {
                    name: "assessment".to_string(),
                    attributes: vec![
                        AttributeDef {
                            name: "ident".to_string(),
                            required: true,
                            values: None,
                        },
                        AttributeDef {
                            name: "title".to_string(),
                            required: true,
                            values: None,
                        },
                    ],
                    required: true,
                    children: vec!["qtimetadata".to_string(), "section".to_string()],
                },
                ElementDef {
                    name: "section".to_string(),
                    attributes: vec![
                        AttributeDef {
                            name: "ident".to_string(),
                            required: true,
                            values: None,
                        },
                        AttributeDef {
                            name: "title".to_string(),
                            required: false,
                            values: None,
                        },
                    ],
                    required: true,
                    children: vec!["item".to_string()],
                },
                ElementDef {
                    name: "item".to_string(),
                    attributes: vec![
                        AttributeDef {
                            name: "ident".to_string(),
                            required: true,
                            values: None,
                        },
                        AttributeDef {
                            name: "title".to_string(),
                            required: false,
                            values: None,
                        },
                        AttributeDef {
                            name: "maxattempts".to_string(),
                            required: false,
                            values: None,
                        },
                    ],
                    required: true,
                    children: vec![
                        "itemmetadata".to_string(),
                        "presentation".to_string(),
                        "resprocessing".to_string(),
                        "itemfeedback".to_string(),
                    ],
                },
                ElementDef {
                    name: "presentation".to_string(),
                    attributes: vec![],
                    required: true,
                    children: vec![
                        "material".to_string(),
                        "response_lid".to_string(),
                        "response_str".to_string(),
                        "response_num".to_string(),
                        "response_grp".to_string(),
                    ],
                },
                ElementDef {
                    name: "response_lid".to_string(),
                    attributes: vec![
                        AttributeDef {
                            name: "ident".to_string(),
                            required: true,
                            values: None,
                        },
                        AttributeDef {
                            name: "rcardinality".to_string(),
                            required: false,
                            values: Some(vec!["Single".to_string(), "Multiple".to_string()]),
                        },
                        AttributeDef {
                            name: "rtiming".to_string(),
                            required: false,
                            values: Some(vec!["No".to_string(), "Yes".to_string()]),
                        },
                    ],
                    required: false,
                    children: vec!["render_choice".to_string()],
                },
                ElementDef {
                    name: "render_choice".to_string(),
                    attributes: vec![AttributeDef {
                        name: "shuffle".to_string(),
                        required: false,
                        values: Some(vec!["Yes".to_string(), "No".to_string()]),
                    }],
                    required: false,
                    children: vec!["response_label".to_string()],
                },
                ElementDef {
                    name: "response_label".to_string(),
                    attributes: vec![AttributeDef {
                        name: "ident".to_string(),
                        required: true,
                        values: None,
                    }],
                    required: false,
                    children: vec!["material".to_string()],
                },
                ElementDef {
                    name: "material".to_string(),
                    attributes: vec![],
                    required: false,
                    children: vec!["mattext".to_string(), "matimage".to_string()],
                },
                ElementDef {
                    name: "mattext".to_string(),
                    attributes: vec![AttributeDef {
                        name: "texttype".to_string(),
                        required: false,
                        values: Some(vec!["text/plain".to_string(), "text/html".to_string()]),
                    }],
                    required: false,
                    children: vec![],
                },
                ElementDef {
                    name: "resprocessing".to_string(),
                    attributes: vec![],
                    required: true,
                    children: vec!["outcomes".to_string(), "respcondition".to_string()],
                },
                ElementDef {
                    name: "outcomes".to_string(),
                    attributes: vec![],
                    required: false,
                    children: vec!["decvar".to_string()],
                },
                ElementDef {
                    name: "decvar".to_string(),
                    attributes: vec![
                        AttributeDef {
                            name: "varname".to_string(),
                            required: true,
                            values: None,
                        },
                        AttributeDef {
                            name: "vartype".to_string(),
                            required: false,
                            values: None,
                        },
                        AttributeDef {
                            name: "minvalue".to_string(),
                            required: false,
                            values: None,
                        },
                        AttributeDef {
                            name: "maxvalue".to_string(),
                            required: false,
                            values: None,
                        },
                    ],
                    required: false,
                    children: vec![],
                },
                ElementDef {
                    name: "respcondition".to_string(),
                    attributes: vec![AttributeDef {
                        name: "continue".to_string(),
                        required: false,
                        values: Some(vec!["Yes".to_string(), "No".to_string()]),
                    }],
                    required: false,
                    children: vec![
                        "conditionvar".to_string(),
                        "setvar".to_string(),
                        "displayfeedback".to_string(),
                    ],
                },
                ElementDef {
                    name: "conditionvar".to_string(),
                    attributes: vec![],
                    required: false,
                    children: vec![
                        "varequal".to_string(),
                        "and".to_string(),
                        "or".to_string(),
                        "not".to_string(),
                    ],
                },
                ElementDef {
                    name: "varequal".to_string(),
                    attributes: vec![
                        AttributeDef {
                            name: "respident".to_string(),
                            required: true,
                            values: None,
                        },
                        AttributeDef {
                            name: "case".to_string(),
                            required: false,
                            values: Some(vec!["Yes".to_string(), "No".to_string()]),
                        },
                    ],
                    required: false,
                    children: vec![],
                },
                ElementDef {
                    name: "setvar".to_string(),
                    attributes: vec![
                        AttributeDef {
                            name: "varname".to_string(),
                            required: true,
                            values: None,
                        },
                        AttributeDef {
                            name: "action".to_string(),
                            required: false,
                            values: Some(vec![
                                "Set".to_string(),
                                "Add".to_string(),
                                "Subtract".to_string(),
                            ]),
                        },
                    ],
                    required: false,
                    children: vec![],
                },
                ElementDef {
                    name: "displayfeedback".to_string(),
                    attributes: vec![AttributeDef {
                        name: "linkrefid".to_string(),
                        required: true,
                        values: None,
                    }],
                    required: false,
                    children: vec![],
                },
                ElementDef {
                    name: "itemfeedback".to_string(),
                    attributes: vec![AttributeDef {
                        name: "ident".to_string(),
                        required: true,
                        values: None,
                    }],
                    required: false,
                    children: vec!["material".to_string()],
                },
                ElementDef {
                    name: "response_str".to_string(),
                    attributes: vec![
                        AttributeDef {
                            name: "ident".to_string(),
                            required: true,
                            values: None,
                        },
                        AttributeDef {
                            name: "rcardinality".to_string(),
                            required: false,
                            values: Some(vec!["Single".to_string(), "Multiple".to_string()]),
                        },
                    ],
                    required: false,
                    children: vec!["render_fib".to_string()],
                },
                ElementDef {
                    name: "response_num".to_string(),
                    attributes: vec![
                        AttributeDef {
                            name: "ident".to_string(),
                            required: true,
                            values: None,
                        },
                        AttributeDef {
                            name: "rcardinality".to_string(),
                            required: false,
                            values: Some(vec!["Single".to_string(), "Multiple".to_string()]),
                        },
                        AttributeDef {
                            name: "numtype".to_string(),
                            required: false,
                            values: None,
                        },
                    ],
                    required: false,
                    children: vec!["render_fib".to_string()],
                },
                ElementDef {
                    name: "render_fib".to_string(),
                    attributes: vec![
                        AttributeDef {
                            name: "rows".to_string(),
                            required: false,
                            values: None,
                        },
                        AttributeDef {
                            name: "columns".to_string(),
                            required: false,
                            values: None,
                        },
                    ],
                    required: false,
                    children: vec![],
                },
            ],
        }
    }

    pub fn validate(&self, element: &Element) -> Result<()> {
        self.validate_element(element, &self.root)?;
        Ok(())
    }

    fn validate_element(&self, element: &Element, expected_name: &str) -> Result<()> {
        if element.name != expected_name {
            return Err(QtiError::ValidationError(format!(
                "Expected element '{}', found '{}'",
                expected_name, element.name
            )));
        }

        let element_def = self
            .elements
            .iter()
            .find(|e| e.name == element.name)
            .ok_or_else(|| {
                QtiError::ValidationError(format!("Unknown element: {}", element.name))
            })?;

        for attr_def in &element_def.attributes {
            // Namespace attributes are handled differently by XML parsers
            if attr_def.name.starts_with("xmlns") {
                continue;
            }

            if attr_def.required && !element.attributes.contains_key(&attr_def.name) {
                return Err(QtiError::ValidationError(format!(
                    "Missing required attribute '{}' on element '{}'",
                    attr_def.name, element.name
                )));
            }

            if let Some(value) = element.attributes.get(&attr_def.name) {
                if let Some(ref valid_values) = attr_def.values {
                    if !valid_values.contains(value) {
                        return Err(QtiError::ValidationError(
                            format!("Invalid value '{}' for attribute '{}' on element '{}'. Valid values: {:?}",
                                value, attr_def.name, element.name, valid_values)
                        ));
                    }
                }
            }
        }

        for child in &element.children {
            if let XMLNode::Element(child_element) = child {
                if !element_def.children.contains(&child_element.name) {
                    return Err(QtiError::ValidationError(format!(
                        "Unexpected child element '{}' in '{}'",
                        child_element.name, element.name
                    )));
                }
                self.validate_element(child_element, &child_element.name)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qti_schema_creation() {
        let schema = QtiSchema::qti_1_2_schema();
        assert_eq!(schema.root, "questestinterop");
        assert!(!schema.elements.is_empty());

        let assessment = schema
            .elements
            .iter()
            .find(|e| e.name == "assessment")
            .expect("assessment element should be defined");

        assert!(assessment.attributes.iter().any(|a| a.name == "ident"));
        assert!(assessment.attributes.iter().any(|a| a.name == "title"));
    }
}
