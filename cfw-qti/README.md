# CFW-QTI - Cloudflare Worker for QTI Generation

A Cloudflare Worker that provides a web interface for generating QTI (Question and Test Interoperability) packages from text-based quiz input.

## Features

- **Web Interface**: Simple HTML form for entering quiz text
- **AI Quiz Generation**: Generate quizzes automatically using Cloudflare Workers AI
  - Configurable question count, types, and difficulty
  - Grade-level appropriate content (K-2 through Professional)
  - Optional source material integration
  - Smart feedback generation
- **Real-time Generation**: Generate QTI packages directly in the browser
- **Format Guide**: Built-in documentation showing quiz format syntax
- **Skip Validation**: Option to skip schema validation for faster generation

## API Endpoints

### `GET /`
Serves the HTML interface for manual quiz generation.

### `GET /ai`
Serves the AI-powered quiz generation interface.

### `POST /generate`
Generates a QTI package from quiz text.

**Request Body:**
```json
{
  "content": "title: My Quiz\n\n1. What is 2+2?\n*a) 4\nb) 5",
  "skip_validation": false
}
```

**Response:**
- Content-Type: `application/zip`
- Content-Disposition: `attachment; filename="quiz.zip"`
- Body: ZIP file containing QTI package

**Error Response:**
```json
{
  "error": "Error message here"
}
```

### `POST /ai/generate`
Generates quiz text using AI based on parameters.

**Request Body:**
```json
{
  "question_count": 10,
  "question_types": ["multiple_choice", "text_match"],
  "topic": "Photosynthesis",
  "grade_level": "9-12",
  "feedback": "valuable",
  "distribution": "7 multiple choice, 3 short answer",
  "source_material": "Optional text or notes to base quiz on"
}
```

**Response:**
```json
{
  "quiz_text": "title: Photosynthesis Quiz\n\n1. What is..."
}
```

**Question Types:**
- `multiple_choice` - Single correct answer
- `multiple_answer` - Multiple correct answers
- `text_match` - Short answer
- `numeric` - Numeric with tolerance
- `essay` - Essay/open-ended

**Grade Levels:**
- `K-2`, `3-5`, `6-8`, `9-12`, `college`, `professional`

**Feedback Options:**
- `never` - No feedback
- `valuable` - Only when helpful (recommended)
- `always` - Feedback on every question

### `GET /health`
Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "service": "cfw-qti"
}
```

## Development

### Prerequisites
- Rust (latest stable)
- Node.js and npm
- Wrangler CLI: `npm install -g wrangler`

### Local Development

1. Build the worker:
```bash
cd cfw-qti
cargo build --target wasm32-unknown-unknown --release
```

2. Run locally with Wrangler:
```bash
wrangler dev
```

3. Open your browser to `http://localhost:8787`

### Deployment

1. Configure your Cloudflare account in `wrangler.toml`

2. Deploy:
```bash
wrangler deploy
```

## Quiz Format

The worker accepts the same text format as the CLI tool:

```
title: Sample Quiz

1. What is the capital of France?
*a) Paris
b) London
c) Berlin
d) Madrid
feedback: Paris is the capital and largest city of France.

2. Select all prime numbers:
[*] 2
[*] 3
[ ] 4
[*] 5
```

### Supported Question Types

- **Multiple Choice**: `*a)` for correct, `a)` for incorrect
- **Multiple Answer**: `[*]` for correct, `[ ]` for incorrect
- **Short Answer**: `* answer` (multiple acceptable answers allowed)
- **Numerical**: `= value ± margin`
- **Essay**: `___` (3+ underscores)
- **File Upload**: `^^^` (3+ carets)

## Architecture

The worker is built with:
- **worker-rs**: Cloudflare Workers SDK for Rust
- **qti-lib**: Core QTI generation library (from this monorepo)
- **serde**: JSON serialization/deserialization

The worker compiles to WebAssembly and runs on Cloudflare's edge network.

## Size Optimization

The workspace is configured with aggressive size optimizations for the release build:
- `opt-level = "z"`: Optimize for binary size
- `lto = true`: Enable link-time optimization
- `codegen-units = 1`: Single codegen unit for better optimization

This keeps the worker bundle small enough to meet Cloudflare's size limits.

## License

MIT
