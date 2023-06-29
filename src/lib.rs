use reqwest::Client;
use serde_json::json;

pub async fn vibrate(strength: i32, time: f32, stop_previous: bool) {
    let lovense_local_ip = "localhost";
    let lovense_http_port = 8000;

    let data = json!({
        "command": "Function",
        "action": format!("Vibrate:{}", strength),
        "timeSec": time,
        "stopPrevious": stop_previous as i32,
        "apiVer": 1
    });

    let client = Client::new();
    let url = format!("http://{}:{}/command", lovense_local_ip, lovense_http_port);

    match client.post(&url).json(&data).send().await {
        Ok(_) => println!("Vibrate command sent"),
        Err(e) => println!("Error sending vibrate command: {}", e)
    }
}