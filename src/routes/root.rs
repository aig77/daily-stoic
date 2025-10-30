use axum::response::Html;

pub async fn root() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Daily Stoic API</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
            padding: 20px;
        }
        .container {
            background: white;
            border-radius: 12px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
            max-width: 800px;
            width: 100%;
            padding: 40px;
        }
        h1 {
            color: #333;
            margin-bottom: 10px;
            font-size: 2.5em;
        }
        .subtitle {
            color: #666;
            margin-bottom: 30px;
            font-size: 1.1em;
        }
        .status {
            background: #10b981;
            color: white;
            padding: 8px 16px;
            border-radius: 20px;
            display: inline-block;
            margin-bottom: 30px;
            font-weight: 600;
        }
        .endpoints {
            margin-top: 30px;
        }
        .endpoint {
            background: #f8f9fa;
            border-left: 4px solid #667eea;
            padding: 15px;
            margin-bottom: 15px;
            border-radius: 4px;
        }
        .method {
            display: inline-block;
            padding: 4px 8px;
            border-radius: 4px;
            font-weight: bold;
            font-size: 0.85em;
            margin-right: 10px;
        }
        .get { background: #10b981; color: white; }
        .put { background: #f59e0b; color: white; }
        .path {
            font-family: 'Courier New', monospace;
            color: #333;
            font-weight: 500;
        }
        .description {
            color: #666;
            margin-top: 8px;
            font-size: 0.95em;
        }
        .footer {
            margin-top: 40px;
            padding-top: 20px;
            border-top: 1px solid #e5e7eb;
            color: #666;
            text-align: center;
            font-size: 0.9em;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>📚 Daily Stoic API</h1>
        <p class="subtitle">Quotes from "The Daily Stoic" by Ryan Holiday</p>
        <div class="status">🟢 API Online</div>

        <div class="endpoints">
            <h2 style="color: #333; margin-bottom: 20px;">Available Endpoints</h2>

            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/quote/{id}</span>
                <div class="description">Get a quote by date ID (MM-DD format, e.g., 03-15)</div>
            </div>

            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/quote/daily</span>
                <div class="description">Get today's stoic quote</div>
            </div>

            <div class="endpoint">
                <span class="method get">GET</span>
                <span class="path">/quote/random</span>
                <div class="description">Get a random stoic quote</div>
            </div>

            <div class="endpoint">
                <span class="method put">PUT</span>
                <span class="path">/quote/{id}</span>
                <div class="description">Update a quote by date ID (requires JSON body)</div>
            </div>
        </div>

        <div class="footer">
            Built with Rust 🦀 and Axum
        </div>
    </div>
</body>
</html>"#,
    )
}
