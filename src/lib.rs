use reqwest::Client;
use serde_json::{json, Value};

pub struct LovenseClient {
    pub local_ip: String,
    pub http_port: u16
}

impl LovenseClient {
    async fn send_request(&self, endpoint: &str, data: Value) {
        let client = Client::new();
        let url = format!("http://{}:{}/{}", self.local_ip, self.http_port, endpoint);

        match client.post(&url).json(&data).send().await {
            Ok(_) => println!("Vibrate command sent"),
            Err(e) => println!("Error sending vibrate command: {}", e)
        }
    }

    /// Actions: Vibrate, Rotate, Pump, Thrusting, Fingering, Suction, All
    /// Strength: 0-20 except for Pump, which is 0-3
    /// Time (sec): 0 = indefinite length. Otherwise, running time should be greater than 1.
    /// Stop previous: Stop all previous commands and execute current commands
    async fn single_function(&self, action: &str, strength: i32, time_sec: f32, toy_id: Option<&str>, stop_previous: bool) {

        let action = format!("{}:{}", action, strength);

        let mut data = json!({
        "command": "Function",
        "action": action,
        "timeSec": time_sec,
        "stopPrevious": stop_previous,
        "apiVer": 1
    });

        if let Some(toy_id) = toy_id {
            data["toy_id"] = Value::String(toy_id.to_string());
        }

        self.send_request( "command", data).await;
    }

    async fn looping_function(&self, action: &str, strength: i32, time_sec: f32, loop_running_sec: f32, loop_pause_sec: f32, toy_id: Option<&str>, stop_previous: bool) {

        let action = format!("{}:{}", action, strength);

        let mut data = json!({
        "command": "Function",
        "action": action,
        "timeSec": time_sec,
        "loopRunningSec": loop_running_sec,
        "loopPauseSec": loop_pause_sec,
        "stopPrevious": stop_previous,
        "apiVer": 1
    });

        if let Some(toy_id) = toy_id {
            data["toy_id"] = Value::String(toy_id.to_string());
        }

        self.send_request("command", data).await;
    }

    pub async fn single_vibrate(&self, strength: i32, time_sec: f32, toy_id: Option<&str>, stop_previous: bool) {
        self.single_function("Vibrate", strength, time_sec, toy_id, stop_previous).await;
    }

    pub async fn looping_vibrate(&self, strength: i32, time_sec: f32, loop_running_sec: f32, loop_pause_sec: f32, toy_id: Option<&str>, stop_previous: bool) {
        self.looping_function("Vibrate", strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await;
    }
}