pub const AI_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AI Quiz Generator</title>
    <style>
        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
            line-height: 1.6;
            color: #333;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            padding: 20px;
        }

        .container {
            max-width: 900px;
            margin: 0 auto;
            background: white;
            border-radius: 12px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            overflow: hidden;
        }

        header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 40px;
            text-align: center;
        }

        header h1 {
            font-size: 2.5em;
            margin-bottom: 10px;
        }

        header p {
            font-size: 1.1em;
            opacity: 0.9;
        }

        .content {
            padding: 40px;
        }

        .form-section {
            margin-bottom: 30px;
        }

        .form-section h3 {
            color: #667eea;
            margin-bottom: 15px;
            font-size: 1.2em;
        }

        label {
            display: block;
            font-weight: 600;
            margin-bottom: 8px;
            color: #555;
        }

        input[type="text"],
        input[type="number"],
        textarea {
            width: 100%;
            padding: 12px;
            border: 2px solid #e0e0e0;
            border-radius: 8px;
            font-size: 14px;
            font-family: inherit;
            transition: border-color 0.3s;
        }

        input[type="text"]:focus,
        input[type="number"]:focus,
        textarea:focus {
            outline: none;
            border-color: #667eea;
        }

        textarea {
            min-height: 150px;
            resize: vertical;
            font-family: 'Monaco', 'Courier New', monospace;
        }

        .checkbox-group,
        .radio-group {
            display: flex;
            flex-direction: column;
            gap: 12px;
        }

        .checkbox-item,
        .radio-item {
            display: flex;
            align-items: center;
            gap: 10px;
        }

        input[type="checkbox"],
        input[type="radio"] {
            width: 20px;
            height: 20px;
            cursor: pointer;
        }

        button {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 15px 40px;
            font-size: 1.1em;
            font-weight: 600;
            border-radius: 8px;
            cursor: pointer;
            transition: transform 0.2s, box-shadow 0.2s;
            width: 100%;
            margin-top: 20px;
        }

        button:hover {
            transform: translateY(-2px);
            box-shadow: 0 8px 20px rgba(102, 126, 234, 0.4);
        }

        button:active {
            transform: translateY(0);
        }

        button:disabled {
            background: #ccc;
            cursor: not-allowed;
            transform: none;
        }

        #status {
            margin-top: 20px;
            padding: 15px;
            border-radius: 8px;
            display: none;
        }

        #status.success {
            background: #d4edda;
            color: #155724;
            border: 1px solid #c3e6cb;
            display: block;
        }

        #status.error {
            background: #f8d7da;
            color: #721c24;
            border: 1px solid #f5c6cb;
            display: block;
        }

        #status.loading {
            background: #d1ecf1;
            color: #0c5460;
            border: 1px solid #bee5eb;
            display: block;
        }

        .info-box {
            background: #e8f4fd;
            border-left: 4px solid #667eea;
            padding: 15px;
            border-radius: 8px;
            margin-top: 10px;
            font-size: 0.9em;
            color: #555;
        }

        .back-link {
            display: inline-block;
            margin: 20px 40px;
            color: white;
            text-decoration: none;
            font-weight: 600;
            opacity: 0.9;
            transition: opacity 0.3s;
        }

        .back-link:hover {
            opacity: 1;
        }

        #generatedQuiz {
            margin-top: 20px;
            padding: 15px;
            background: #f8f9fa;
            border-radius: 8px;
            border: 2px solid #e0e0e0;
            font-family: 'Monaco', 'Courier New', monospace;
            font-size: 13px;
            white-space: pre-wrap;
            display: none;
        }

        .action-buttons {
            display: flex;
            gap: 10px;
            margin-top: 10px;
        }

        .action-buttons button {
            flex: 1;
            margin-top: 0;
        }

        .secondary-btn {
            background: #6c757d !important;
        }
    </style>
</head>
<body>
    <a href="/" class="back-link">← Back to Manual Generator</a>

    <div class="container">
        <header>
            <h1>🤖 AI Quiz Generator</h1>
            <p>Generate educational quizzes powered by AI</p>
        </header>

        <div class="content">
            <form id="aiQuizForm">
                <div class="form-section">
                    <h3>1. Number of Questions</h3>
                    <input type="number" id="questionCount" name="questionCount" value="10" min="1" max="50" required>
                    <div class="info-box">Recommended: 5-15 questions for best results</div>
                </div>

                <div class="form-section">
                    <h3>2. Question Types</h3>
                    <div class="checkbox-group">
                        <div class="checkbox-item">
                            <input type="checkbox" id="multipleChoice" name="questionTypes" value="multiple_choice" checked>
                            <label for="multipleChoice" style="margin-bottom: 0;">Multiple Choice (single answer)</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" id="multipleAnswer" name="questionTypes" value="multiple_answer">
                            <label for="multipleAnswer" style="margin-bottom: 0;">Multiple Choice (multiple answers)</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" id="textMatch" name="questionTypes" value="text_match">
                            <label for="textMatch" style="margin-bottom: 0;">Text Match (short answer)</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" id="numeric" name="questionTypes" value="numeric">
                            <label for="numeric" style="margin-bottom: 0;">Numeric (with tolerance)</label>
                        </div>
                        <div class="checkbox-item">
                            <input type="checkbox" id="essay" name="questionTypes" value="essay">
                            <label for="essay" style="margin-bottom: 0;">Essay/Open-ended</label>
                        </div>
                    </div>
                </div>

                <div class="form-section">
                    <h3>3. Quiz Topic</h3>
                    <input type="text" id="topic" name="topic" placeholder='e.g., "American Revolution", "Photosynthesis", "Python Programming"' required>
                </div>

                <div class="form-section">
                    <h3>4. Grade Level</h3>
                    <div class="radio-group">
                        <div class="radio-item">
                            <input type="radio" id="gradeK2" name="gradeLevel" value="K-2">
                            <label for="gradeK2" style="margin-bottom: 0;">K-2 (Elementary)</label>
                        </div>
                        <div class="radio-item">
                            <input type="radio" id="grade35" name="gradeLevel" value="3-5">
                            <label for="grade35" style="margin-bottom: 0;">3-5 (Elementary)</label>
                        </div>
                        <div class="radio-item">
                            <input type="radio" id="grade68" name="gradeLevel" value="6-8">
                            <label for="grade68" style="margin-bottom: 0;">6-8 (Middle School)</label>
                        </div>
                        <div class="radio-item">
                            <input type="radio" id="grade912" name="gradeLevel" value="9-12" checked>
                            <label for="grade912" style="margin-bottom: 0;">9-12 (High School)</label>
                        </div>
                        <div class="radio-item">
                            <input type="radio" id="gradeCollege" name="gradeLevel" value="college">
                            <label for="gradeCollege" style="margin-bottom: 0;">College/University</label>
                        </div>
                        <div class="radio-item">
                            <input type="radio" id="gradeProfessional" name="gradeLevel" value="professional">
                            <label for="gradeProfessional" style="margin-bottom: 0;">Professional/Advanced</label>
                        </div>
                    </div>
                </div>

                <div class="form-section">
                    <h3>5. Include Feedback</h3>
                    <div class="radio-group">
                        <div class="radio-item">
                            <input type="radio" id="feedbackNever" name="feedback" value="never">
                            <label for="feedbackNever" style="margin-bottom: 0;">Never</label>
                        </div>
                        <div class="radio-item">
                            <input type="radio" id="feedbackValuable" name="feedback" value="valuable" checked>
                            <label for="feedbackValuable" style="margin-bottom: 0;">Only when valuable (recommended)</label>
                        </div>
                        <div class="radio-item">
                            <input type="radio" id="feedbackAlways" name="feedback" value="always">
                            <label for="feedbackAlways" style="margin-bottom: 0;">Always</label>
                        </div>
                    </div>
                </div>

                <div class="form-section">
                    <h3>6. Question Distribution (Optional)</h3>
                    <input type="text" id="distribution" name="distribution" placeholder='e.g., "4 multiple choice, 3 numeric, 3 essay"'>
                    <div class="info-box">Leave blank for even distribution across selected types</div>
                </div>

                <div class="form-section">
                    <h3>7. Source Material (Optional)</h3>
                    <textarea id="sourceMaterial" name="sourceMaterial" placeholder="Paste text, notes, or key points the quiz should be based on..."></textarea>
                    <div class="info-box">Provide context, learning objectives, or specific content to focus on</div>
                </div>

                <button type="submit" id="generateBtn">Generate Quiz with AI</button>
            </form>

            <div id="status"></div>

            <div id="generatedQuiz"></div>

            <div class="action-buttons" id="actionButtons" style="display: none;">
                <button onclick="copyQuiz()" class="secondary-btn">Copy Quiz Text</button>
                <button onclick="downloadQTI()">Download QTI Package</button>
            </div>
        </div>
    </div>

    <script>
        let currentQuizText = '';

        document.getElementById('aiQuizForm').addEventListener('submit', async function(e) {
            e.preventDefault();

            const statusDiv = document.getElementById('status');
            const generateBtn = document.getElementById('generateBtn');
            const generatedQuizDiv = document.getElementById('generatedQuiz');
            const actionButtons = document.getElementById('actionButtons');

            // Get form values
            const questionCount = parseInt(document.getElementById('questionCount').value);
            const topic = document.getElementById('topic').value.trim();
            const gradeLevel = document.querySelector('input[name="gradeLevel"]:checked').value;
            const feedback = document.querySelector('input[name="feedback"]:checked').value;
            const distribution = document.getElementById('distribution').value.trim();
            const sourceMaterial = document.getElementById('sourceMaterial').value.trim();

            // Get selected question types
            const questionTypes = Array.from(document.querySelectorAll('input[name="questionTypes"]:checked'))
                .map(cb => cb.value);

            if (questionTypes.length === 0) {
                statusDiv.className = 'error';
                statusDiv.textContent = 'Please select at least one question type';
                return;
            }

            // Show loading state
            statusDiv.className = 'loading';
            statusDiv.textContent = '🤖 AI is generating your quiz... This may take 30-60 seconds.';
            generateBtn.disabled = true;
            generatedQuizDiv.style.display = 'none';
            actionButtons.style.display = 'none';

            try {
                const response = await fetch('/ai/generate', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        question_count: questionCount,
                        question_types: questionTypes,
                        topic: topic,
                        grade_level: gradeLevel,
                        feedback: feedback,
                        distribution: distribution || null,
                        source_material: sourceMaterial || null
                    })
                });

                if (!response.ok) {
                    const errorData = await response.json();
                    throw new Error(errorData.error || 'Failed to generate quiz');
                }

                const data = await response.json();
                currentQuizText = data.quiz_text;

                generatedQuizDiv.textContent = currentQuizText;
                generatedQuizDiv.style.display = 'block';
                actionButtons.style.display = 'flex';

                statusDiv.className = 'success';
                statusDiv.textContent = '✓ Quiz generated successfully! Review the quiz below and download as QTI package.';
            } catch (error) {
                statusDiv.className = 'error';
                statusDiv.textContent = '✗ Error: ' + error.message;
            } finally {
                generateBtn.disabled = false;
            }
        });

        function copyQuiz() {
            navigator.clipboard.writeText(currentQuizText).then(() => {
                const statusDiv = document.getElementById('status');
                statusDiv.className = 'success';
                statusDiv.textContent = '✓ Quiz text copied to clipboard!';
            });
        }

        async function downloadQTI() {
            const statusDiv = document.getElementById('status');

            statusDiv.className = 'loading';
            statusDiv.textContent = 'Generating QTI package...';

            try {
                const response = await fetch('/generate', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        content: currentQuizText,
                        skip_validation: false
                    })
                });

                if (!response.ok) {
                    const errorData = await response.json();
                    throw new Error(errorData.error || 'Failed to generate QTI package');
                }

                // Get the filename from Content-Disposition header or use default
                const contentDisposition = response.headers.get('Content-Disposition');
                let filename = 'quiz.zip';
                if (contentDisposition) {
                    // Match either quoted or unquoted filename
                    const filenameMatch = contentDisposition.match(/filename="([^"]+)"|filename=([^;\s]+)/);
                    if (filenameMatch) {
                        filename = filenameMatch[1] || filenameMatch[2];
                    }
                }

                // Download the file
                const blob = await response.blob();
                const url = window.URL.createObjectURL(blob);
                const a = document.createElement('a');
                a.href = url;
                a.download = filename;
                document.body.appendChild(a);
                a.click();
                window.URL.revokeObjectURL(url);
                document.body.removeChild(a);

                statusDiv.className = 'success';
                statusDiv.textContent = '✓ QTI package downloaded successfully!';
            } catch (error) {
                statusDiv.className = 'error';
                statusDiv.textContent = '✗ Error: ' + error.message;
            }
        }
    </script>
</body>
</html>
"#;
