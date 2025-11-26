use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a complete assessment/quiz
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assessment {
    pub title: String,
    pub identifier: String,
    pub description: Option<String>,
    pub time_limit: Option<u32>, // in minutes
    pub questions: Vec<Question>,
    pub metadata: AssessmentMetadata,
}

impl Assessment {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            identifier: format!("assessment_{}", Uuid::new_v4()),
            description: None,
            time_limit: None,
            questions: Vec::new(),
            metadata: AssessmentMetadata::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AssessmentMetadata {
    pub author: Option<String>,
    pub course: Option<String>,
    pub shuffle_questions: bool,
    pub shuffle_answers: bool,
    pub show_feedback: bool,
    pub allow_review: bool,
}

/// Represents an individual question/item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub title: String,
    pub text: String,
    pub question_type: QuestionType,
    pub points: f32,
    pub feedback: Option<Feedback>,
    pub solution: Option<String>,
}

impl Question {
    pub fn new(text: impl Into<String>, question_type: QuestionType) -> Self {
        Self {
            id: format!("question_{}", Uuid::new_v4()),
            title: String::new(),
            text: text.into(),
            question_type,
            points: 1.0,
            feedback: None,
            solution: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    MultipleChoice {
        choices: Vec<Choice>,
        shuffle: bool,
    },
    TrueFalse {
        correct_answer: bool,
    },
    MultipleAnswer {
        choices: Vec<Choice>,
        partial_credit: bool,
    },
    ShortAnswer {
        answers: Vec<AcceptableAnswer>,
        case_sensitive: bool,
    },
    Numerical {
        answer: f64,
        margin: Option<f64>,
        min: Option<f64>,
        max: Option<f64>,
    },
    Essay {
        expected_length: Option<usize>,
        rich_text: bool,
    },
    FileUpload {
        allowed_extensions: Vec<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub id: String,
    pub text: String,
    pub correct: bool,
    pub feedback: Option<String>,
    pub weight: Option<f32>, // For partial credit
}

impl Choice {
    pub fn new(text: impl Into<String>, correct: bool) -> Self {
        Self {
            id: format!("choice_{}", Uuid::new_v4()),
            text: text.into(),
            correct,
            feedback: None,
            weight: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptableAnswer {
    pub text: String,
    pub weight: f32, // 1.0 for fully correct, 0.5 for partial credit, etc.
}

impl AcceptableAnswer {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            weight: 1.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feedback {
    pub correct: Option<String>,
    pub incorrect: Option<String>,
    pub general: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnswerType {
    Single(String),        // For single correct answer
    Multiple(Vec<String>), // For multiple correct answers
    Range(f64, f64),       // For numerical ranges
    Pattern(String),       // For regex patterns
}
