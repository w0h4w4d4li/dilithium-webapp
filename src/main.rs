use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use crystals_dilithium::dilithium2;
use hex;

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
}

async fn sign_message(req: web::Json<SignRequest>) -> HttpResponse {
    let msg_bytes = req.message.as_bytes();
    
    let keypair = dilithium2::Keypair::generate(None);
    let signature = keypair.sign(msg_bytes);
    
    // CORRECT: Use to_bytes() method
    let public_key_bytes = keypair.public.to_bytes();
    let signature_bytes = signature.as_ref();
    
    let verified = keypair.public.verify(msg_bytes, &signature);
    
    HttpResponse::Ok().json(SignResponse {
        success: true,
        public_key: hex::encode(public_key_bytes),
        signature: hex::encode(signature_bytes),
        public_key_size: public_key_bytes.len(),
        signature_size: signature_bytes.len(),
        verified,
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Dilithium Web Service...");
    
    HttpServer::new(|| {
        App::new()
            .route("/sign", web::post().to(sign_message))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}