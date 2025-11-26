# AI Quiz Generation Features

## Overview

The CFW-QTI worker now includes AI-powered quiz generation using Cloudflare Workers AI. This allows users to automatically generate educational quizzes by specifying parameters like topic, grade level, and question types.

## How It Works

### 1. User Interface (`GET /ai`)

A beautiful, user-friendly form collects the following parameters:

- **Number of Questions**: 1-50 questions (default: 10)
- **Question Types**: Multiple selection of:
  - Multiple Choice (single answer)
  - Multiple Choice (multiple answers)
  - Text Match (short answer)
  - Numeric (with tolerance)
  - Essay/Open-ended
- **Quiz Topic**: Free text (e.g., "Photosynthesis", "American Revolution")
- **Grade Level**: Radio selection
  - K-2 (Elementary)
  - 3-5 (Elementary)
  - 6-8 (Middle School)
  - 9-12 (High School) [default]
  - College/University
  - Professional/Advanced
- **Include Feedback**: How often to add explanatory feedback
  - Never
  - Only when valuable (recommended) [default]
  - Always
- **Question Distribution**: Optional text (e.g., "4 multiple choice, 3 numeric, 3 essay")
- **Source Material**: Optional large textarea for notes, text, or context

### 2. AI Generation (`POST /ai/generate`)

The handler:
1. Validates the request parameters
2. Builds a carefully crafted prompt that includes:
   - Grade-level specific instructions
   - Exact syntax format for the QTI text format
   - Question type requirements
   - Feedback generation rules
   - Source material context (if provided)
3. Calls Cloudflare Workers AI using the `openchat-3.5-0106` model
4. Returns the generated quiz text in the standard format

### 3. Integration with Existing Generator

Once the AI generates quiz text:
- User can review it in a formatted text box
- Click "Copy Quiz Text" to copy to clipboard
- Click "Download QTI Package" to automatically:
  - Send the quiz text to `/generate` endpoint
  - Generate the QTI package
  - Download the ZIP file

## Grade Level Intelligence

The AI is instructed to adjust content based on grade level:

### Elementary (K-2, 3-5)
- Very simple vocabulary
- Short sentences
- Basic concepts
- Concrete examples
- No abstract concepts

### Middle School (6-8)
- Grade-appropriate vocabulary
- Moderate complexity
- Fundamental understanding focus

### High School (9-12)
- Advanced vocabulary
- Complex concepts
- Teenage-appropriate examples

### College/Professional
- Sophisticated vocabulary
- Technical terminology
- Deep understanding expected
- Expert-level knowledge

## Example Prompt Structure

The system generates prompts like:

```
Generate a quiz in the following exact text format...

**Requirements:**
- Create exactly 10 questions
- Topic: Photosynthesis
- Grade Level: high school level (ages 14-18): Use advanced vocabulary and complex concepts
- Question types to use: Multiple Choice (single answer), Text Match (short answer)
- Feedback Rule: ONLY add feedback if valuable

**CRITICAL: Follow this exact syntax format:**

title: [Topic name]

1. Question text here?
*a) Correct answer
b) Wrong answer
...

[Detailed formatting instructions]

Generate the quiz now:
```

## Technical Implementation

### Files Added

1. **`src/ai_html.rs`**: HTML page for AI quiz generation
   - Beautiful, responsive design matching main page
   - Comprehensive form with all parameters
   - Real-time generation status
   - Review and download workflow

2. **`src/ai_handlers.rs`**: Handler functions
   - `serve_ai_html()`: Serves the AI generation page
   - `generate_ai_quiz()`: Processes AI requests
   - `build_quiz_prompt()`: Constructs the AI prompt
   - Grade level and feedback rule mapping
   - Input validation

3. **Router updates in `src/lib.rs`**:
   - `GET /ai` → AI generation page
   - `POST /ai/generate` → AI quiz generation endpoint

4. **Configuration in `wrangler.toml`**:
   ```toml
   [ai]
   binding = "AI"
   ```

### Dependencies

- No new Rust dependencies needed
- Uses existing `worker` crate's AI binding
- Cloudflare Workers AI is available on all paid Cloudflare plans

## AI Model

Uses `@cf/openai/gpt-oss-120b`:
- 120B parameter open-source GPT model
- Excellent instruction following and structured output
- Fast response times (typically 20-40 seconds)
- Good understanding of educational content
- Available on Cloudflare Workers AI

The model can be easily swapped in `src/ai_handlers.rs` by changing the model ID in the `ai.run()` call.

## User Experience Flow

1. User visits `/ai`
2. Fills out quiz parameters (30 seconds)
3. Clicks "Generate Quiz with AI"
4. Loading message appears: "🤖 AI is generating your quiz... This may take 30-60 seconds."
5. Quiz appears in formatted text box (can review/edit)
6. User can:
   - Copy quiz text to use elsewhere
   - Download directly as QTI package
7. Success message confirms download

## Error Handling

The system handles:
- Missing AI binding (clear error message)
- Invalid parameters (validation errors)
- AI generation failures (error display)
- Timeout issues (graceful degradation)
- Empty responses (validation)

## Testing

### Test Locally

```bash
# Start the worker
wrangler dev

# Visit http://localhost:8787/ai

# Or test API directly:
curl -X POST http://localhost:8787/ai/generate \
  -H "Content-Type: application/json" \
  -d @test_ai_quiz.json
```

### Example Request

See `test_ai_quiz.json`:
```json
{
  "question_count": 5,
  "question_types": ["multiple_choice", "text_match"],
  "topic": "The Great Depression",
  "grade_level": "9-12",
  "feedback": "valuable",
  "distribution": null,
  "source_material": null
}
```

## Future Enhancements

Potential improvements:
1. Quiz preview before generating QTI
2. Edit quiz text in-browser
3. Save/load quiz templates
4. Batch generation for multiple topics
5. Integration with learning objectives databases
6. Question bank storage
7. Analytics on quiz difficulty
8. A/B testing different prompts for quality

## Security Considerations

- Input validation on all parameters
- Question count limited to 50 to prevent abuse
- Rate limiting should be configured in Cloudflare dashboard
- No user data stored (stateless generation)
- AI responses are not cached (privacy)

## Cost Considerations

Cloudflare Workers AI pricing:
- Free tier: 10,000 neurons/day
- Each quiz generation uses ~1,000-5,000 neurons
- Paid plans: $0.01 per 1,000 neurons after free tier
- Typical cost: ~$0.01-$0.05 per quiz

Very affordable for educational use!
