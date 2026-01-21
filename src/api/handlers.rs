use axum::{Json, http::StatusCode};
use crate::models::{KeygenResponse, EncryptRequest, EncryptResponse, DecryptRequest, DecryptResponse};
use crate::crypto::{keygen, encrypt, decrypt, utils::Parameters};

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn keygen_handler() -> Json<KeygenResponse> {
    let params = Parameters::default();
    let keys = keygen::keygen_string(&params, None);
    
    Json(KeygenResponse {
        public_key: keys.get("public").unwrap().clone(),
        secret_key: keys.get("secret").unwrap().clone(),
    })
}

pub async fn encrypt_handler(Json(payload): Json<EncryptRequest>) -> Json<EncryptResponse> {
    let params = Parameters::default();
    let ciphertext = encrypt::encrypt_string(&payload.public_key, &payload.message, &params, None);
    
    Json(EncryptResponse {
        ciphertext,
    })
}

pub async fn decrypt_handler(Json(payload): Json<DecryptRequest>) -> Json<DecryptResponse> {
    let params = Parameters::default();
    let message = decrypt::decrypt_string(&payload.secret_key, &payload.ciphertext, &params);
    
    Json(DecryptResponse {
        message,
    })
}
