use axum::Json;
use crate::models::{KeygenResponse, EncryptRequest, EncryptResponse, DecryptRequest, DecryptResponse};
use crate::crypto::{keygen, encrypt, decrypt, utils::Parameters};

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn keygen_handler() -> Json<KeygenResponse> {
    let keys = tokio::task::spawn_blocking(move || {
        let params = Parameters::default();
        keygen::keygen_string(&params, None)
    }).await.unwrap();
    
    Json(KeygenResponse {
        public_key: keys.get("public").unwrap().clone(),
        secret_key: keys.get("secret").unwrap().clone(),
    })
}

pub async fn encrypt_handler(Json(payload): Json<EncryptRequest>) -> Json<EncryptResponse> {
    let ciphertext = tokio::task::spawn_blocking(move || {
        let params = Parameters::default();
        encrypt::encrypt_string(&payload.public_key, &payload.message, &params, None)
    }).await.unwrap();
    
    Json(EncryptResponse {
        ciphertext,
    })
}

pub async fn decrypt_handler(Json(payload): Json<DecryptRequest>) -> Json<DecryptResponse> {
    let message = tokio::task::spawn_blocking(move || {
        let params = Parameters::default();
        decrypt::decrypt_string(&payload.secret_key, &payload.ciphertext, &params)
    }).await.unwrap();
    
    Json(DecryptResponse {
        message,
    })
}
