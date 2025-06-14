#[poem::handler]
pub async fn index() -> poem::Response {
    let html_content = r###"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>ColrSkeww</title>
        <meta name="viewport" content="width=device-width, initial-scale=1.0">

        <link rel="preconnect" href="https://fonts.googleapis.com">
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
        <link href="https://fonts.googleapis.com/css2?family=Lexend:wght@100..900&display=swap" rel="stylesheet">
    </head>
    <body>
        <div class="container">
            <h1>ColrSkeww</h1>
            <p class="subtitle">Color Blindness Simulation Tool</p>

            <h2>Simulation</h2>
            <form action="/upload" method="post" enctype="multipart/form-data">
            <p>Choose a simulation type to see how the image would look for different types of color blindness.</p>
                <div>

                    <select name="deficiency" id="deficiency">
                        <option value="deutan">Deuteranopia (Red-Green)</option>
                        <option value="protan">Protanopia (Red-Green)</option>
                        <option value="tritan">Tritanopia (Blue-Yellow)</option>
                    </select>
                </div>
                <input type="file" name="upload" id="upload" required>
                <button type="submit">Simulate</button>
            </form>

            <div class="api-section">
                <h2>API Documentation</h2>
                <p>ColrSkeww provides a simple HTTP API for programmatic color blindness simulation.</p>

                <h3>Endpoint</h3>
                <div class="endpoint">
                    <strong>POST</strong> <code>/upload</code>
                </div>

                <h3>Request Format</h3>
                <p>Send a <code>multipart/form-data</code> request with the following fields:</p>
                <ul>
                    <li><strong>upload</strong> (required): Image file (PNG, JPEG, BMP, GIF, WebP)</li>
                    <li><strong>deficiency</strong> (optional): Simulation type, defaults to "deutan"</li>
                </ul>

                <h3>Supported Deficiency Types</h3>
                <ul>
                    <li><code>deutan</code> or <code>deuteranopia</code> - Green color blindness (most common)</li>
                    <li><code>protan</code> or <code>protanopia</code> - Red color blindness</li>
                    <li><code>tritan</code> or <code>tritanopia</code> - Blue color blindness (rare)</li>
                </ul>

                <h3>Response</h3>
                <p>Returns a PNG image showing how the uploaded image would appear to someone with the specified color vision deficiency.</p>

                <h3>Example Usage</h3>
                <div class="code-block">
                    <h4>cURL</h4>
                    <pre><code>curl -X POST \
  -F "upload=@image.jpg" \
  -F "deficiency=deutan" \
  http://localhost:3000/upload \
  --output simulated.png</code></pre>
                </div>

                <div class="code-block">
                    <h4>JavaScript (Fetch API)</h4>
                    <pre><code>const formData = new FormData();
formData.append('upload', fileInput.files[0]);
formData.append('deficiency', 'protan');

fetch('/upload', {
  method: 'POST',
  body: formData
})
.then(response => response.blob())
.then(blob => {
  const url = URL.createObjectURL(blob);
  document.getElementById('result').src = url;
});</code></pre>
                </div>

                <h3>Error Responses</h3>
                <ul>
                    <li><strong>400 Bad Request</strong>: Invalid image format or missing upload field</li>
                    <li><strong>400 Bad Request</strong>: Invalid deficiency type</li>
                    <li><strong>500 Internal Server Error</strong>: Image processing error</li>
                </ul>
            </div>

            <footer class="footer">
                <p>&copy; 2025 <a href="https://suryavirkapur.com">svk</a>. All rights reserved.</p>
                <p>ColrSkeww - Scientific color blindness simulation.</p>
                <p>Based on: Viénot, F., Brettel, H., & Mollon, J. D. (1999). Digital Video Colourmaps for Checking the Legibility of Displays by Dichromats. <a href="https://vision.psychol.cam.ac.uk/jdmollon/papers/colourmaps.pdf">(PDF Link)</a></p>
            </footer>
        </div>
        <style>
            body {
                font-family: "Lexend" ,Arial, sans-serif;
                color: #333;
                margin: 0;
                padding: 20px;
            }
            .container {
                max-width: 600px;
                margin: auto;
                padding: 20px;
            }
            h1 {
                text-align: center;
                margin-bottom: 5px;
            }
            .subtitle {
                text-align: center;
                color: #666;
                font-style: italic;
                margin-bottom: 30px;
            }
            form {
                display: flex;
                flex-direction: column;
            }
            label {
                margin-bottom: 5px;
            }
            select, input[type="file"], button {
                margin-bottom: 15px;
                margin-top: 5px;
            }
            button {
                padding: 10px;
                border: none;
                cursor: pointer;
            }
            select {
                width: 100%;
                padding: 8px;
                border-radius: 4px;
                border: 1px solid #ccc;
            }
            input[type="file"] {
                padding: 8px;
                border-radius: 4px;
                border: 1px solid #ccc;
            }
            .api-section {
                margin-top: 40px;
                padding-top: 30px;
            }
            h2 {
                color: #2c3e50;
                border-bottom: 2px solid #3498db;
                padding-bottom: 5px;
            }
            h3 {
                color: #34495e;
                margin-top: 25px;
            }
            .endpoint {
                background: #f8f9fa;
                padding: 10px;
                border-left: 4px solid #3498db;
                margin: 10px 0;
                font-family: monospace;
            }
            .code-block {
                margin: 15px 0;
            }
            .code-block h4 {
                margin-bottom: 5px;
                color: #2c3e50;
            }
            pre {
                background: #2c3e50;
                color: #ecf0f1;
                padding: 15px;
                border-radius: 5px;
                overflow-x: auto;
                font-size: 14px;
                line-height: 1.4;
            }
            code {
                background: #ecf0f1;
                padding: 2px 4px;
                border-radius: 3px;
                font-family: monospace;
                color: #2c3e50;
            }
            pre code {
                background: none;
                padding: 0;
                color: #ecf0f1;
            }
            .footer {
                margin-top: 50px;
                padding-top: 20px;
                border-top: 1px solid #eee;
                text-align: center;
                color: #666;
                font-size: 14px;
            }
            .footer p {
                margin: 5px 0;
            }
            @media (max-width: 600px) {
                .container {
                    padding: 15px;
                }
                button {
                    width: 100%;
                }
                pre {
                    font-size: 12px;
                    padding: 10px;
                }
            }
        </style>
    </body>
    </html>
    "###;
    poem::Response::builder()
        .header("Content-Type", "text/html; charset=utf-8")
        .body(html_content)
}
