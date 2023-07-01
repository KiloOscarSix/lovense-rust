use std::collections::HashMap;
use reqwest::Client;
use serde_json::json;

#[derive(Debug, serde::Deserialize)]
pub struct LovenseQrCodeResponse
{
    pub result: bool,
    pub code: i32,
    pub message: String,
    pub data: HashMap<String, String>
}

pub async fn create_qr_code(token: &str, uid: &str, uname: &str) -> Result<LovenseQrCodeResponse, reqwest::Error> {
    let data = json!({
        "token": token,
        "uid": uid,
        "uname": uname,
        "v": "2"
    });

    let client = Client::new();

    match client.post("https://api.lovense.com/api/lan/getQrCode").json(&data).send().await {
        Ok(res) => Ok(res.json::<LovenseQrCodeResponse>().await?),
        Err(e) => Err(e)
    }
}