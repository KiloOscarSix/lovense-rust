use std::collections::HashMap;
use reqwest::Client;
use serde_json::json;

pub struct LovenseServer {
    token: String,
}

impl LovenseServer {
    pub fn new(token: String) -> LovenseServer {
        LovenseServer {token}
    }

    pub async fn create_qr_code(&self, user_id: &str, username: &str) -> Result<LovenseQrCodeResponse, reqwest::Error> {
        let data = json!({
            "token": &self.token,
            "uid": user_id,
            "uname": username,
            "v": "2"
        });

        let client = Client::new();

        match client.post("https://api.lovense.com/api/lan/getQrCode").json(&data).send().await {
            Ok(res) => Ok(res.json::<LovenseQrCodeResponse>().await?),
            Err(e) => Err(e)
        }

    }
}


#[derive(Debug, serde::Deserialize)]
pub struct LovenseQrCodeResponse
{
    pub result: bool,
    pub code: i32,
    pub message: String,
    pub data: HashMap<String, String>
}