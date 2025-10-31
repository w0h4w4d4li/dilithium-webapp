use actix_web::{web, App, HttpServer, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crystals_dilithium::dilithium2::Keypair;
use hex;
use std::env;

#[derive(Deserialize)]
struct SignRequest {
    message: String,
}

#[derive(Serialize)]
struct SignResponse {
    success: bool,
    public_key: String,
    signature: String,
    public_key_size: usize,
    signature_size: usize,
    verified: bool,
    message: String,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
    environment: String,
}

async fn sign_message(req: web::Json<SignRequest>) -> Result<HttpResponse> {
    let msg_bytes = req.message.as_bytes();
    
    let keypair = Keypair::generate(None);
    let signature = keypair.sign(msg_bytes);
    let verified = keypair.public.verify(msg_bytes, &signature);
    
    let public_key_bytes = keypair.public.to_bytes();
    let signature_bytes = signature.as_ref();
    
    let response = SignResponse {
        success: true,
        public_key: hex::encode(public_key_bytes),
        signature: hex::encode(signature_bytes),
        public_key_size: public_key_bytes.len(),
        signature_size: signature_bytes.len(),
        verified,
        message: req.message.clone(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

async fn health_check() -> Result<HttpResponse> {
    let response = HealthResponse {
        status: "healthy".to_string(),
        service: "Dilithium Signature API".to_string(),
        version: "1.0.0".to_string(),
        environment: env::var("RAILWAY_ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

async fn web_interface() -> Result<HttpResponse> {
    let html = r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Dilithium Signature Service</title>
        <style>
            * { box-sizing: border-box; margin: 0; padding: 0; }
            body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; line-height: 1.6; color: #333; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); min-height: 100vh; }
            .container { max-width: 800px; margin: 0 auto; padding: 20px; }
            header { background: rgba(255, 255, 255, 0.1); backdrop-filter: blur(10px); color: white; padding: 2rem 0; text-align: center; margin-bottom: 2rem; border-radius: 15px; }
            .card { background: white; padding: 2rem; border-radius: 15px; box-shadow: 0 10px 30px rgba(0,0,0,0.2); margin-bottom: 2rem; }
            .form-group { margin-bottom: 1.5rem; }
            label { display: block; margin-bottom: 0.5rem; font-weight: bold; color: #2c3e50; }
            input, textarea, button { width: 100%; padding: 1rem; border: 2px solid #e1e8ed; border-radius: 8px; font-size: 1rem; transition: all 0.3s; }
            textarea { min-height: 120px; resize: vertical; font-family: monospace; }
            textarea:focus, input:focus { outline: none; border-color: #3498db; box-shadow: 0 0 0 3px rgba(52, 152, 219, 0.1); }
            button { background: linear-gradient(135deg, #3498db, #2980b9); color: white; border: none; cursor: pointer; font-weight: bold; }
            button:hover { transform: translateY(-2px); box-shadow: 0 5px 15px rgba(0,0,0,0.2); }
            button:disabled { opacity: 0.6; cursor: not-allowed; transform: none; }
            .result { background: #f8f9fa; padding: 1.5rem; border-radius: 8px; margin-top: 1.5rem; font-family: 'Courier New', monospace; font-size: 0.9rem; white-space: pre-wrap; word-break: break-all; border-left: 4px solid #3498db; }
            .success { color: #27ae60; }
            .error { color: #e74c3c; }
            .info { color: #3498db; }
            .feature-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 1rem; margin-top: 1rem; }
            .feature { background: #f8f9fa; padding: 1rem; border-radius: 8px; text-align: center; }
        </style>
    </head>
    <body>
        <div class="container">
            <header>
                <h1>🔐 Dilithium Signature Service</h1>
                <p>Post-Quantum Cryptography Powered by Rust</p>
            </header>

            <div class="card">
                <h2>Sign a Message</h2>
                <div class="form-group">
                    <label for="message">Message to Sign:</label>
                    <textarea id="message" placeholder="Enter your message here...">Hello from Railway! 🚄</textarea>
                </div>
                <button onclick="signMessage()" id="signButton">Generate Quantum-Safe Signature</button>
                <div id="signResult" class="result"></div>
            </div>

            <div class="card">
                <h2>About This Service</h2>
                <p>This service uses <strong>Dilithium</strong>, a post-quantum digital signature scheme selected by NIST for standardization.</p>
                
                <div class="feature-grid">
                    <div class="feature">
                        <h3>🔒 Quantum-Safe</h3>
                        <p>Resistant to attacks from quantum computers</p>
                    </div>
                    <div class="feature">
                        <h3>⚡ Built with Rust</h3>
                        <p>High-performance and memory-safe implementation</p>
                    </div>
                    <div class="feature">
                        <h3>🌐 Deployed on Railway</h3>
                        <p>Serverless deployment with automatic scaling</p>
                    </div>
                </div>
            </div>
        </div>

        <script>
            async function signMessage() {
                const message = document.getElementById('message').value;
                const button = document.getElementById('signButton');
                const resultDiv = document.getElementById('signResult');
                
                if (!message.trim()) {
                    resultDiv.innerHTML = '<span class="error">Please enter a message to sign.</span>';
                    return;
                }

                // Show loading state
                const originalText = button.textContent;
                button.textContent = '🔄 Generating Signature...';
                button.disabled = true;
                resultDiv.innerHTML = '<span class="info">Signing message with Dilithium algorithm...</span>';

                try {
                    const response = await fetch('/sign', {
                        method: 'POST',
                        headers: {
                            'Content-Type': 'application/json',
                        },
                        body: JSON.stringify({ message })
                    });

                    const data = await response.json();

                    if (data.success) {
                        resultDiv.innerHTML = `
<span class="success">✅ Signature generated successfully!</span>

📝 Message: ${data.message}
📏 Message length: ${data.message.length} characters

🔑 Public Key (${data.public_key_size} bytes):
${data.public_key}

📋 Signature (${data.signature_size} bytes):
${data.signature}

✅ Verification: ${data.verified ? 'SUCCESS' : 'FAILED'}

💡 Tip: The signature is quantum-safe and will remain secure even with quantum computers!
                        `;
                    } else {
                        resultDiv.innerHTML = `<span class="error">❌ Error: ${data.error || 'Unknown error'}</span>`;
                    }
                } catch (error) {
                    resultDiv.innerHTML = `<span class="error">❌ Network error: ${error.message}</span>`;
                } finally {
                    button.textContent = originalText;
                    button.disabled = false;
                }
            }

            // Allow submitting with Ctrl+Enter in textarea
            document.getElementById('message').addEventListener('keydown', function(e) {
                if (e.ctrlKey && e.key === 'Enter') {
                    signMessage();
                }
            });
        </script>
    </body>
    </html>
    "#;
    
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = env::var("RAILWAY_STATIC_URL").unwrap_or_else(|_| "0.0.0.0".to_string());
    
    println!("🚄 Starting Dilithium Web Application on Railway");
    println!("🌐 Host: {}", host);
    println!("🔌 Port: {}", port);
    println!("📡 Environment: {}", env::var("RAILWAY_ENVIRONMENT").unwrap_or_else(|_| "production".to_string()));
    println!("🔗 Endpoints:");
    println!("   GET  /          - Web interface");
    println!("   POST /sign      - Sign a message");
    println!("   GET  /health    - Health check");
    
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(web_interface))
            .route("/sign", web::post().to(sign_message))
            .route("/health", web::get().to(health_check))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}