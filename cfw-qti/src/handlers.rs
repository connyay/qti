use qti_lib::{Exporter, Parser};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use worker::*;

#[derive(Deserialize)]
pub struct GenerateRequest {
    content: String,
    #[serde(default)]
    canvas: bool,
    #[serde(default)]
    skip_validation: bool,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub async fn serve_html(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::from_html(crate::html::INDEX_HTML)
}

pub async fn health_check(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::from_json(&serde_json::json!({
        "status": "healthy",
        "service": "cfw-qti"
    }))
}

pub async fn generate_qti(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let request_data: GenerateRequest = match req.json().await {
        Ok(data) => data,
        Err(e) => {
            return Response::from_json(&ErrorResponse {
                error: format!("Invalid request format: {}", e),
            })
            .map(|r| r.with_status(400));
        }
    };

    if request_data.content.trim().is_empty() {
        return Response::from_json(&ErrorResponse {
            error: "Quiz content cannot be empty".to_string(),
        })
        .map(|r| r.with_status(400));
    }

    let parser = Parser::new();
    let mut assessment = match parser.parse(&request_data.content) {
        Ok(assessment) => assessment,
        Err(e) => {
            return Response::from_json(&ErrorResponse {
                error: format!("Parse error: {}", e),
            })
            .map(|r| r.with_status(400));
        }
    };

    let filename = if !assessment.title.is_empty() {
        sanitize_filename(&assessment.title)
    } else {
        "quiz".to_string()
    };

    if assessment.identifier.is_empty() {
        assessment.identifier = filename.clone();
    }

    let zip_data = match generate_zip(
        &assessment,
        request_data.canvas,
        request_data.skip_validation,
    ) {
        Ok(data) => data,
        Err(e) => {
            return Response::from_json(&ErrorResponse {
                error: format!("Generation error: {}", e),
            })
            .map(|r| r.with_status(500));
        }
    };

    let headers = Headers::new();
    headers.set("Content-Type", "application/zip")?;
    let clean_filename = filename.trim_end_matches(".zip");
    headers.set(
        "Content-Disposition",
        &format!("attachment; filename=\"{}.zip\"", clean_filename),
    )?;
    headers.set("Content-Length", &zip_data.len().to_string())?;

    Ok(Response::from_bytes(zip_data)?.with_headers(headers))
}

fn generate_zip(
    assessment: &qti_lib::types::Assessment,
    canvas: bool,
    skip_validation: bool,
) -> std::result::Result<Vec<u8>, String> {
    let mut buffer = Cursor::new(Vec::new());

    let mut exporter = if canvas {
        Exporter::new().with_canvas_extensions()
    } else {
        Exporter::new()
    };

    if skip_validation {
        exporter = exporter.skip_validation();
    }

    exporter
        .export_to_writer(assessment, &mut buffer)
        .map_err(|e| format!("Export failed: {}", e))?;

    Ok(buffer.into_inner())
}

fn sanitize_filename(name: &str) -> String {
    let sanitized = name
        .chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => c,
            ' ' => '_',
            _ => '-',
        })
        .collect::<String>()
        .trim_matches(|c| c == '-' || c == '_')
        .to_string();

    if sanitized.is_empty() {
        "quiz".to_string()
    } else {
        sanitized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("My Quiz"), "My_Quiz");
        assert_eq!(sanitize_filename("Quiz #1: Test!"), "Quiz_-1-_Test");
        assert_eq!(sanitize_filename("  test  "), "test");
    }
}
