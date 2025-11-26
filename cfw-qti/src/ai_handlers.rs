use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Deserialize)]
pub struct AiQuizRequest {
    question_count: u32,
    question_types: Vec<String>,
    topic: String,
    grade_level: String,
    feedback: String,
    distribution: Option<String>,
    source_material: Option<String>,
}

#[derive(Serialize)]
pub struct AiQuizResponse {
    quiz_text: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Serialize)]
struct AiTextGenerationRequest {
    input: String,
}

pub async fn serve_ai_html(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::from_html(crate::ai_html::AI_HTML)
}

pub async fn generate_ai_quiz(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let request_data: AiQuizRequest = match req.json().await {
        Ok(data) => data,
        Err(e) => {
            return Response::from_json(&ErrorResponse {
                error: format!("Invalid request format: {}", e),
            })
            .map(|r| r.with_status(400));
        }
    };

    if request_data.question_types.is_empty() {
        return Response::from_json(&ErrorResponse {
            error: "At least one question type must be selected".to_string(),
        })
        .map(|r| r.with_status(400));
    }

    if request_data.topic.trim().is_empty() {
        return Response::from_json(&ErrorResponse {
            error: "Topic cannot be empty".to_string(),
        })
        .map(|r| r.with_status(400));
    }

    let ai = match ctx.env.ai("AI") {
        Ok(ai) => ai,
        Err(_) => {
            return Response::from_json(&ErrorResponse {
                error: "AI binding not configured. Please add AI binding in wrangler.toml"
                    .to_string(),
            })
            .map(|r| r.with_status(500));
        }
    };

    let prompt = build_quiz_prompt(&request_data);

    let ai_request = AiTextGenerationRequest { input: prompt };

    let ai_response_raw: serde_json::Value =
        match ai.run("@cf/openai/gpt-oss-120b", ai_request).await {
            Ok(response) => response,
            Err(e) => {
                return Response::from_json(&ErrorResponse {
                    error: format!("AI generation failed: {}", e),
                })
                .map(|r| r.with_status(500));
            }
        };

    // Response format: { output: [ {reasoning}, {content: [{text: "..."}]} ] }
    let quiz_text =
        if let Some(output_array) = ai_response_raw.get("output").and_then(|v| v.as_array()) {
            let mut text = None;
            for item in output_array {
                if let Some(content_array) = item.get("content").and_then(|v| v.as_array()) {
                    if let Some(first_content) = content_array.first() {
                        if let Some(text_str) = first_content.get("text").and_then(|v| v.as_str()) {
                            if item.get("type").and_then(|v| v.as_str()) == Some("message") {
                                text = Some(text_str.to_string());
                                break;
                            }
                        }
                    }
                }
            }

            if let Some(t) = text {
                t
            } else {
                return Response::from_json(&ErrorResponse {
                    error: "Could not find quiz text in AI response".to_string(),
                })
                .map(|r| r.with_status(500));
            }
        } else {
            return Response::from_json(&ErrorResponse {
                error: format!("Unexpected AI response format: {}", ai_response_raw),
            })
            .map(|r| r.with_status(500));
        };

    Response::from_json(&AiQuizResponse {
        quiz_text: quiz_text.trim().to_string(),
    })
}

fn build_quiz_prompt(request: &AiQuizRequest) -> String {
    let grade_instructions = match request.grade_level.as_str() {
        "K-2" => "kindergarten through 2nd grade level (ages 5-8): Use very simple vocabulary, short sentences, and basic concepts. Focus on concrete examples and avoid abstract concepts.",
        "3-5" => "3rd through 5th grade level (ages 8-11): Use elementary vocabulary and straightforward concepts. Include age-appropriate context and examples.",
        "6-8" => "middle school level (ages 11-14): Use grade-appropriate vocabulary and moderate complexity. Focus on fundamental understanding.",
        "9-12" => "high school level (ages 14-18): Use advanced vocabulary and complex concepts appropriate for teenagers.",
        "college" => "college/university level: Use sophisticated vocabulary and expect deep understanding of concepts.",
        "professional" => "professional/advanced level: Use technical terminology and expect expert knowledge.",
        _ => "high school level",
    };

    let feedback_rules = match request.feedback.as_str() {
        "never" => "Do NOT include feedback for any questions.",
        "valuable" => "ONLY add a `feedback:` line if the answer is counterintuitive, addresses a common misconception, or provides valuable learning context. Most questions should NOT have feedback.",
        "always" => "Include a `feedback:` line for every question with educational explanation.",
        _ => "Only include feedback when valuable.",
    };

    let question_type_descriptions = request
        .question_types
        .iter()
        .map(|qt| match qt.as_str() {
            "multiple_choice" => "Multiple Choice (single answer)",
            "multiple_answer" => "Multiple Choice (multiple correct answers)",
            "text_match" => "Text Match (short answer)",
            "numeric" => "Numeric (with tolerance)",
            "essay" => "Essay/Open-ended",
            _ => qt.as_str(),
        })
        .collect::<Vec<_>>()
        .join(", ");

    let mut prompt = format!(
        r#"You are an expert educational content creator. Your task is to generate a quiz in a specific text format that will be parsed by a computer program.

**CRITICAL:** Output ONLY the quiz text in the format specified below. Do not include any explanations, comments, or additional text outside the quiz format.

**Requirements:**
- Create exactly {} questions
- Topic: {}
- Grade Level: {}
- Question types to use: {}
- Feedback Rule: {}

"#,
        request.question_count,
        request.topic,
        grade_instructions,
        question_type_descriptions,
        feedback_rules
    );

    if let Some(dist) = &request.distribution {
        if !dist.is_empty() {
            prompt.push_str(&format!("- Question distribution: {}\n", dist));
        }
    } else {
        prompt.push_str("- Distribute question types roughly evenly across selected types\n");
    }

    if let Some(source) = &request.source_material {
        if !source.is_empty() {
            prompt.push_str(&format!(
                "\n**Source Material to base questions on:**\n{}\n\n",
                source
            ));
        }
    }

    prompt.push_str(&format!(
        r#"
**CRITICAL: Follow this exact syntax format:**

Start with the title:
title: [Topic name]

Then for each question, use ONE of these formats:

**Multiple Choice (single answer):**
1. Question text here?
a) Wrong answer
*b) Correct answer (asterisk before letter)
c) Wrong answer
d) Wrong answer
feedback: Optional explanation (only if valuable)

**Multiple Choice (multiple answers):**
2. Select all that apply:
[*] Correct answer (asterisk in brackets)
[*] Another correct answer
[ ] Wrong answer (empty brackets)
[ ] Wrong answer

**Short Answer (text match):**
3. Short question?
* acceptable answer
* another acceptable answer
* third acceptable answer

**Numeric (with tolerance):**
4. Calculate the value:
= 3.14 ± 0.01
feedback: Optional explanation

**Essay:**
5. Explain the concept:
___

**Important formatting rules:**
- For multiple choice, use *a), *b), etc. for the correct answer
- For multiple answer, use [*] for correct, [ ] for incorrect
- For short answer, use * before each acceptable answer
- For numeric, use = value ± tolerance
- For essay, use ___ (three underscores)
- Only include "feedback:" when it adds educational value
- Do NOT include any explanations outside the specified format
- Do NOT number feedback lines
- Use appropriate difficulty for {} level
- Ensure questions test understanding, not just memorization

Generate the quiz now:"#,
        grade_instructions
    ));

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_quiz_prompt() {
        let request = AiQuizRequest {
            question_count: 5,
            question_types: vec!["multiple_choice".to_string(), "text_match".to_string()],
            topic: "Photosynthesis".to_string(),
            grade_level: "9-12".to_string(),
            feedback: "valuable".to_string(),
            distribution: None,
            source_material: None,
        };

        let prompt = build_quiz_prompt(&request);

        assert!(prompt.contains("Create exactly 5 questions"));
        assert!(prompt.contains("Photosynthesis"));
        assert!(prompt.contains("high school level"));
    }
}
