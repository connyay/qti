use regex::Regex;
use crate::error::{QtiError, Result};
use crate::types::{AcceptableAnswer, Assessment, Choice, Feedback, Question, QuestionType};

pub struct Parser {
    /// Regex patterns matching text2qti format
    question_pattern: Regex,
    mc_correct_pattern: Regex,
    mc_incorrect_pattern: Regex,
    ma_correct_pattern: Regex,
    ma_incorrect_pattern: Regex,
    shortans_pattern: Regex,
    numerical_pattern: Regex,
    essay_pattern: Regex,
    upload_pattern: Regex,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            question_pattern: Regex::new(r"^\d+\.\s+").unwrap(),
            mc_correct_pattern: Regex::new(r"^\*[a-zA-Z]\)\s*").unwrap(),
            mc_incorrect_pattern: Regex::new(r"^[a-zA-Z]\)\s*").unwrap(),
            ma_correct_pattern: Regex::new(r"^\[\*\]\s*").unwrap(),
            ma_incorrect_pattern: Regex::new(r"^\[\s?\]\s*").unwrap(),
            shortans_pattern: Regex::new(r"^\*\s*").unwrap(),
            numerical_pattern: Regex::new(r"^=\s*([-+]?\d*\.?\d+)\s*(?:±\s*([-+]?\d*\.?\d+))?")
                .unwrap(),
            essay_pattern: Regex::new(r"^_{3,}$").unwrap(),
            upload_pattern: Regex::new(r"^\^{3,}$").unwrap(),
        }
    }

    /// Parse text input into an Assessment
    pub fn parse(&self, input: &str) -> Result<Assessment> {
        let mut assessment = Assessment::new("Untitled Assessment");
        let lines: Vec<&str> = input.lines().collect();
        let mut i = 0;

        if let Some(title) = self.extract_title(&lines) {
            assessment.title = title;
        }

        while i < lines.len() {
            if self.question_pattern.is_match(lines[i]) {
                let question = self.parse_question(&lines, &mut i)?;
                assessment.questions.push(question);
            } else {
                i += 1;
            }
        }

        if assessment.questions.is_empty() {
            return Err(QtiError::ParseError(
                "No questions found in input".to_string(),
            ));
        }

        Ok(assessment)
    }

    /// Extract title from metadata or first line
    fn extract_title(&self, lines: &[&str]) -> Option<String> {
        for line in lines.iter().take(5) {
            if line.starts_with("title:") || line.starts_with("Title:") {
                return Some(line[6..].trim().to_string());
            }
            if line.starts_with("#") {
                return Some(line.trim_start_matches('#').trim().to_string());
            }
        }
        None
    }

    /// Parse a single question starting at position i
    fn parse_question(&self, lines: &[&str], i: &mut usize) -> Result<Question> {
        // Extract question text
        let question_line = lines[*i];
        let text = self.question_pattern.replace(question_line, "").to_string();
        *i += 1;

        // Determine question type by looking ahead
        let question_type = self.determine_question_type(lines, *i)?;

        // Parse based on question type
        let question_type = match question_type {
            QuestionTypeHint::MultipleChoice => self.parse_multiple_choice(lines, i)?,
            QuestionTypeHint::MultipleAnswer => self.parse_multiple_answer(lines, i)?,
            QuestionTypeHint::ShortAnswer => self.parse_short_answer(lines, i)?,
            QuestionTypeHint::Numerical => self.parse_numerical(lines, i)?,
            QuestionTypeHint::Essay => {
                *i += 1; // Skip the ___ line
                QuestionType::Essay {
                    expected_length: None,
                    rich_text: true,
                }
            }
            QuestionTypeHint::FileUpload => {
                *i += 1; // Skip the ^^^ line
                QuestionType::FileUpload {
                    allowed_extensions: vec![
                        "pdf".to_string(),
                        "docx".to_string(),
                        "txt".to_string(),
                    ],
                }
            }
        };

        let mut question = Question::new(text, question_type);

        // Check for feedback or solution after the choices
        self.parse_feedback_and_solution(lines, i, &mut question);

        Ok(question)
    }

    /// Determine the question type by looking at the next lines
    fn determine_question_type(&self, lines: &[&str], start: usize) -> Result<QuestionTypeHint> {
        if start >= lines.len() {
            return Err(QtiError::ParseError("Unexpected end of input".to_string()));
        }

        let line = lines[start];

        if self.mc_correct_pattern.is_match(line) || self.mc_incorrect_pattern.is_match(line) {
            Ok(QuestionTypeHint::MultipleChoice)
        } else if self.ma_correct_pattern.is_match(line) || self.ma_incorrect_pattern.is_match(line)
        {
            Ok(QuestionTypeHint::MultipleAnswer)
        } else if self.shortans_pattern.is_match(line) {
            Ok(QuestionTypeHint::ShortAnswer)
        } else if self.numerical_pattern.is_match(line) {
            Ok(QuestionTypeHint::Numerical)
        } else if self.essay_pattern.is_match(line) {
            Ok(QuestionTypeHint::Essay)
        } else if self.upload_pattern.is_match(line) {
            Ok(QuestionTypeHint::FileUpload)
        } else {
            Err(QtiError::InvalidFormat {
                line: start,
                message: format!("Cannot determine question type from: {}", line),
            })
        }
    }

    /// Parse multiple choice questions
    fn parse_multiple_choice(&self, lines: &[&str], i: &mut usize) -> Result<QuestionType> {
        let mut choices = Vec::new();

        while *i < lines.len() {
            let line = lines[*i];

            if self.mc_correct_pattern.is_match(line) {
                let text = self.mc_correct_pattern.replace(line, "").to_string();
                choices.push(Choice::new(text, true));
                *i += 1;
            } else if self.mc_incorrect_pattern.is_match(line) {
                let text = self.mc_incorrect_pattern.replace(line, "").to_string();
                choices.push(Choice::new(text, false));
                *i += 1;
            } else if self.question_pattern.is_match(line) {
                // Next question starts
                break;
            } else if line.trim().is_empty() {
                *i += 1;
            } else {
                break;
            }
        }

        if choices.is_empty() {
            return Err(QtiError::ParseError(
                "No choices found for multiple choice question".to_string(),
            ));
        }

        let correct_count = choices.iter().filter(|c| c.correct).count();
        if correct_count != 1 {
            return Err(QtiError::ParseError(format!(
                "Multiple choice question must have exactly 1 correct answer, found {}",
                correct_count
            )));
        }

        Ok(QuestionType::MultipleChoice {
            choices,
            shuffle: true,
        })
    }

    /// Parse multiple answer questions
    fn parse_multiple_answer(&self, lines: &[&str], i: &mut usize) -> Result<QuestionType> {
        let mut choices = Vec::new();

        while *i < lines.len() {
            let line = lines[*i];

            if self.ma_correct_pattern.is_match(line) {
                let text = self.ma_correct_pattern.replace(line, "").to_string();
                choices.push(Choice::new(text, true));
                *i += 1;
            } else if self.ma_incorrect_pattern.is_match(line) {
                let text = self.ma_incorrect_pattern.replace(line, "").to_string();
                choices.push(Choice::new(text, false));
                *i += 1;
            } else if self.question_pattern.is_match(line) {
                break;
            } else if line.trim().is_empty() {
                *i += 1;
            } else {
                break;
            }
        }

        if choices.is_empty() {
            return Err(QtiError::ParseError(
                "No choices found for multiple answer question".to_string(),
            ));
        }

        Ok(QuestionType::MultipleAnswer {
            choices,
            partial_credit: true,
        })
    }

    /// Parse short answer questions
    fn parse_short_answer(&self, lines: &[&str], i: &mut usize) -> Result<QuestionType> {
        let mut answers = Vec::new();

        while *i < lines.len() {
            let line = lines[*i];

            if self.shortans_pattern.is_match(line) {
                let text = self.shortans_pattern.replace(line, "").to_string();
                answers.push(AcceptableAnswer::new(text));
                *i += 1;
            } else if self.question_pattern.is_match(line) {
                break;
            } else if line.trim().is_empty() {
                *i += 1;
            } else {
                break;
            }
        }

        if answers.is_empty() {
            return Err(QtiError::ParseError(
                "No answers found for short answer question".to_string(),
            ));
        }

        Ok(QuestionType::ShortAnswer {
            answers,
            case_sensitive: false,
        })
    }

    /// Parse numerical questions
    fn parse_numerical(&self, lines: &[&str], i: &mut usize) -> Result<QuestionType> {
        let line = lines[*i];

        if let Some(captures) = self.numerical_pattern.captures(line) {
            let answer = captures
                .get(1)
                .ok_or_else(|| QtiError::ParseError("No numerical answer found".to_string()))?
                .as_str()
                .parse::<f64>()
                .map_err(|_| QtiError::ParseError("Invalid numerical answer".to_string()))?;

            let margin = captures.get(2).and_then(|m| m.as_str().parse::<f64>().ok());

            *i += 1;

            Ok(QuestionType::Numerical {
                answer,
                margin,
                min: None,
                max: None,
            })
        } else {
            Err(QtiError::ParseError(
                "Invalid numerical answer format".to_string(),
            ))
        }
    }

    /// Parse feedback and solution that may follow a question
    fn parse_feedback_and_solution(&self, lines: &[&str], i: &mut usize, question: &mut Question) {
        let mut feedback = Feedback {
            correct: None,
            incorrect: None,
            general: None,
        };

        while *i < lines.len() {
            let line = lines[*i];

            if line.starts_with("Feedback:") || line.starts_with("feedback:") {
                feedback.general = Some(line[9..].trim().to_string());
                *i += 1;
            } else if line.starts_with("Correct:") || line.starts_with("correct:") {
                feedback.correct = Some(line[8..].trim().to_string());
                *i += 1;
            } else if line.starts_with("Incorrect:") || line.starts_with("incorrect:") {
                feedback.incorrect = Some(line[10..].trim().to_string());
                *i += 1;
            } else if line.starts_with("Solution:") || line.starts_with("solution:") {
                question.solution = Some(line[9..].trim().to_string());
                *i += 1;
            } else if self.question_pattern.is_match(line) {
                break;
            } else if line.trim().is_empty() {
                *i += 1;
            } else {
                break;
            }
        }

        if feedback.correct.is_some() || feedback.incorrect.is_some() || feedback.general.is_some()
        {
            question.feedback = Some(feedback);
        }
    }
}

#[derive(Debug)]
enum QuestionTypeHint {
    MultipleChoice,
    MultipleAnswer,
    ShortAnswer,
    Numerical,
    Essay,
    FileUpload,
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_multiple_choice() {
        let input = r#"
title: Sample Quiz

1. What is 2 + 2?
a) 3
*b) 4
c) 5
d) 6

2. What is the capital of France?
*a) Paris
b) London
c) Berlin
d) Madrid
"#;

        let parser = Parser::new();
        let assessment = parser.parse(input).unwrap();

        assert_eq!(assessment.title, "Sample Quiz");
        assert_eq!(assessment.questions.len(), 2);

        // Check first question
        let q1 = &assessment.questions[0];
        assert_eq!(q1.text, "What is 2 + 2?");
        if let QuestionType::MultipleChoice { choices, .. } = &q1.question_type {
            assert_eq!(choices.len(), 4);
            assert!(!choices[0].correct);
            assert!(choices[1].correct);
        } else {
            panic!("Expected MultipleChoice question type");
        }
    }

    #[test]
    fn test_parse_short_answer() {
        let input = r#"
1. What is the largest planet in our solar system?
* Jupiter
* jupiter

2. Name a primary color.
* red
* blue
* yellow
"#;

        let parser = Parser::new();
        let assessment = parser.parse(input).unwrap();

        assert_eq!(assessment.questions.len(), 2);

        // Check first question
        let q1 = &assessment.questions[0];
        if let QuestionType::ShortAnswer { answers, .. } = &q1.question_type {
            assert_eq!(answers.len(), 2);
            assert_eq!(answers[0].text, "Jupiter");
            assert_eq!(answers[1].text, "jupiter");
        } else {
            panic!("Expected ShortAnswer question type");
        }
    }
}
