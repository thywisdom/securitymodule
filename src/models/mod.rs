use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct KeygenResponse {
    pub public_key: String,
    pub secret_key: String,
}

#[derive(Deserialize)]
pub struct EncryptRequest {
    pub public_key: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct EncryptResponse {
    pub ciphertext: String,
}

#[derive(Deserialize)]
pub struct DecryptRequest {
    pub secret_key: String,
    pub ciphertext: String,
}

#[derive(Serialize)]
pub struct DecryptResponse {
    pub message: String,
}
