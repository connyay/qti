pub const INDEX_HTML: &str = r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>QTI Generator</title>
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
            max-width: 1200px;
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
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 30px;
            padding: 40px;
        }

        .panel {
            display: flex;
            flex-direction: column;
        }

        .panel h2 {
            font-size: 1.5em;
            margin-bottom: 20px;
            color: #667eea;
        }

        .form-group {
            margin-bottom: 20px;
        }

        label {
            display: block;
            font-weight: 600;
            margin-bottom: 8px;
            color: #555;
        }

        textarea {
            width: 100%;
            min-height: 400px;
            padding: 15px;
            border: 2px solid #e0e0e0;
            border-radius: 8px;
            font-family: 'Monaco', 'Courier New', monospace;
            font-size: 14px;
            resize: vertical;
            transition: border-color 0.3s;
        }

        textarea:focus {
            outline: none;
            border-color: #667eea;
        }

        .options {
            display: flex;
            gap: 20px;
            margin-bottom: 20px;
        }

        .checkbox-group {
            display: flex;
            align-items: center;
            gap: 8px;
        }

        .checkbox-group input[type="checkbox"] {
            width: 20px;
            height: 20px;
            cursor: pointer;
        }

        .checkbox-group .tooltip {
            position: relative;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            width: 18px;
            height: 18px;
            border-radius: 50%;
            background: #667eea;
            color: white;
            font-size: 12px;
            font-weight: bold;
            cursor: help;
            margin-left: 4px;
        }

        .checkbox-group .tooltip:hover::after {
            content: attr(data-tooltip);
            position: absolute;
            bottom: 125%;
            left: 50%;
            transform: translateX(-50%);
            padding: 8px 12px;
            background: #333;
            color: white;
            font-size: 13px;
            font-weight: normal;
            white-space: nowrap;
            border-radius: 6px;
            z-index: 1000;
            pointer-events: none;
        }

        .checkbox-group .tooltip:hover::before {
            content: '';
            position: absolute;
            bottom: 115%;
            left: 50%;
            transform: translateX(-50%);
            border: 6px solid transparent;
            border-top-color: #333;
            z-index: 1000;
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

        .info-box {
            background: #f8f9fa;
            border-left: 4px solid #667eea;
            padding: 20px;
            border-radius: 8px;
            margin-bottom: 20px;
        }

        .info-box h3 {
            color: #667eea;
            margin-bottom: 10px;
        }

        .info-box code {
            background: #e9ecef;
            padding: 2px 6px;
            border-radius: 4px;
            font-family: 'Monaco', 'Courier New', monospace;
        }

        .info-box ul {
            margin-left: 20px;
            margin-top: 10px;
        }

        .info-box li {
            margin-bottom: 8px;
        }

        .example-box {
            background: #fff9e6;
            border: 1px solid #ffd700;
            padding: 15px;
            border-radius: 8px;
            margin-top: 20px;
        }

        .example-box pre {
            font-family: 'Monaco', 'Courier New', monospace;
            font-size: 13px;
            white-space: pre-wrap;
            margin-top: 10px;
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

        @media (max-width: 968px) {
            .content {
                grid-template-columns: 1fr;
            }
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>QTI Generator</h1>
            <p>Convert text-based quizzes to QTI 1.2 packages for your LMS</p>
            <p style="margin-top: 15px;"><a href="/ai" style="color: white; text-decoration: underline; opacity: 0.9;">🤖 Try AI Quiz Generator</a></p>
        </header>

        <div class="content">
            <div class="panel">
                <h2>Quiz Input</h2>
                <div class="form-group">
                    <label for="quizText">Enter your quiz in text format:</label>
                    <textarea id="quizText" placeholder="title: My Quiz

1. What is 2 + 2?
*a) 4
b) 5
c) 3

2. Name a primary color.
* red
* blue
* yellow"></textarea>
                </div>

                <div class="options">
                    <!-- Canvas extensions temporarily hidden until fully implemented
                    <div class="checkbox-group">
                        <input type="checkbox" id="canvas" name="canvas">
                        <label for="canvas" style="margin-bottom: 0;">Canvas Extensions</label>
                        <span class="tooltip" data-tooltip="Adds Canvas LMS-specific metadata fields for better integration">?</span>
                    </div>
                    -->
                    <div class="checkbox-group">
                        <input type="checkbox" id="skipValidation" name="skipValidation">
                        <label for="skipValidation" style="margin-bottom: 0;">Skip Validation</label>
                        <span class="tooltip" data-tooltip="Skips QTI 1.2 schema validation for faster generation">?</span>
                    </div>
                </div>

                <button id="generateBtn" onclick="generateQTI()">Generate QTI Package</button>

                <div id="status"></div>
            </div>

            <div class="panel">
                <h2>Format Guide</h2>

                <div class="info-box">
                    <h3>Question Types</h3>
                    <ul>
                        <li><code>*a)</code> or <code>*)</code> - Correct choice (multiple choice)</li>
                        <li><code>a)</code> or <code>)</code> - Incorrect choice</li>
                        <li><code>[*]</code> - Correct choice (multiple answer)</li>
                        <li><code>[ ]</code> - Incorrect choice (multiple answer)</li>
                        <li><code>* answer</code> - Acceptable answer (short answer, multiple allowed)</li>
                        <li><code>= num ± margin</code> - Numerical answer with margin</li>
                        <li><code>___</code> - Essay question (3+ underscores)</li>
                        <li><code>^^^</code> - File upload (3+ carets)</li>
                    </ul>
                </div>

                <div class="info-box">
                    <h3>Optional Fields</h3>
                    <ul>
                        <li><code>title: Quiz Title</code> - Assessment title (at the top)</li>
                        <li><code>feedback: text</code> - Feedback after a question</li>
                    </ul>
                </div>

                <div class="example-box">
                    <strong>Example:</strong>
                    <pre>title: Sample Quiz

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
[*] 5</pre>
                </div>
            </div>
        </div>
    </div>

    <script>
        async function generateQTI() {
            const quizText = document.getElementById('quizText').value.trim();
            const canvasCheckbox = document.getElementById('canvas');
            const canvas = canvasCheckbox ? canvasCheckbox.checked : false;
            const skipValidation = document.getElementById('skipValidation').checked;
            const statusDiv = document.getElementById('status');
            const generateBtn = document.getElementById('generateBtn');

            if (!quizText) {
                statusDiv.className = 'error';
                statusDiv.textContent = 'Please enter quiz text';
                return;
            }

            // Show loading state
            statusDiv.className = 'loading';
            statusDiv.textContent = 'Generating QTI package...';
            generateBtn.disabled = true;

            try {
                const response = await fetch('/generate', {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                    },
                    body: JSON.stringify({
                        content: quizText,
                        skip_validation: skipValidation
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
                statusDiv.textContent = '✓ QTI package generated successfully! Download should start automatically.';
            } catch (error) {
                statusDiv.className = 'error';
                statusDiv.textContent = '✗ Error: ' + error.message;
            } finally {
                generateBtn.disabled = false;
            }
        }

        // Allow Ctrl/Cmd+Enter to generate
        document.getElementById('quizText').addEventListener('keydown', function(e) {
            if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
                generateQTI();
            }
        });
    </script>
</body>
</html>
"#;
