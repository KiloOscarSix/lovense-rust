use std::cmp;
use reqwest::{Client, Error, Response};
use serde_json::{json, Value};
use serde_json::Number;


pub struct LovenseClient {
    local_ip: String,
    http_port: Option<u16>,
    https_port: Option<u16>
}

impl LovenseClient {
    pub fn new(local_ip: &str, https_port: u16) -> LovenseClient {
        let local_ip = String::from(local_ip);
        LovenseClient { local_ip, https_port: Some(https_port), http_port: None}
    }

    pub fn new_http(local_ip: &str, http_port: u16) -> LovenseClient {
        LovenseClient { local_ip: String::from(local_ip), http_port: Some(http_port), https_port: None }
    }

    async fn send_request(&self, endpoint: &str, data: Value) -> Result<Response, Error> {
        let client = Client::new();

        let url = match (self.http_port.is_some(), self.https_port.is_some()) {
            (false, false) => panic!("No port specified"),
            (true, true) => panic!("Both http and https ports specified"),
            (false, true) => format!("https://{}:{}/{}", self.local_ip, self.https_port.unwrap(), endpoint),
            (true, false) => format!("http://{}:{}/{}", self.local_ip, self.http_port.unwrap(), endpoint),
        };


        client.post(&url).json(&data).send().await
    }

    fn create_common_data(&self, action: String, time_sec: f32, toy_id: Option<String>, stop_previous: bool) -> Value {
        let mut data = json!({
            "command": "Function",
            "action": action,
            "timeSec": time_sec,
            "stopPrevious": stop_previous as i32,
            "apiVer": 1
        });

        if let Some(toy_id) = toy_id {
            data["toy_id"] = Value::String(toy_id);
        }

        data
    }

    /// Actions: Vibrate, Rotate, Pump, Thrusting, Fingering, Suction, All
    /// Strength: 0-20 except for Pump, which is 0-3
    /// Time (sec): 0 = indefinite length. Otherwise, running time should be greater than 1.
    /// Stop previous: Stop all previous commands and execute current commands
    pub async fn single_function(&self, action: &str, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) {
        let strength = match action {
            "Pump" => cmp::min(strength, 3),
            _ => cmp::min(strength, 20)
        };

        let action = format!("{}:{}", action, strength);

        let data = self.create_common_data(action, time_sec, toy_id, stop_previous);

        match self.send_request("command", data).await {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e)
        }
    }

    pub async fn looping_function(&self, action: &str, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) {
        let strength = match action {
            "Pump" => cmp::min(strength, 3),
            _ => cmp::min(strength, 20)
        };

        let action = format!("{}:{}", action, strength);

        let mut data = self.create_common_data(action, time_sec, toy_id, stop_previous);
        data["loopRunningSec"] = Value::Number(Number::from_f64(loop_running_sec).unwrap());
        data["loopPauseSec"] = Value::Number(Number::from_f64(loop_pause_sec).unwrap());

        match self.send_request("command", data).await {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e)
        }
    }

    pub async fn single_vibrate(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) {
        self.single_function("Vibrate", strength, time_sec, toy_id, stop_previous).await;
    }

    pub async fn looping_vibrate(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) {
        self.looping_function("Vibrate", strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await;
    }

    pub async fn single_rotate(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) {
        self.single_function("Rotate", strength, time_sec, toy_id, stop_previous).await;
    }

    pub async fn looping_rotate(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) {
        self.looping_function("Rotate", strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await;
    }

    pub async fn single_pump(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) {
        self.single_function("Pump", strength, time_sec, toy_id, stop_previous).await;
    }

    pub async fn looping_pump(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) {
        self.looping_function("Pump", strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await;
    }

    pub async fn single_thrusting(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) {
        self.single_function("Thrusting", strength, time_sec, toy_id, stop_previous).await;
    }

    pub async fn looping_thrusting(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) {
        self.looping_function("Thrusting", strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await;
    }

    pub async fn single_fingering(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) {
        self.single_function("Fingering", strength, time_sec, toy_id, stop_previous).await;
    }

    pub async fn looping_fingering(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) {
        self.looping_function("Fingering", strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await;
    }

    pub async fn single_suction(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) {
        self.single_function("Suction", strength, time_sec, toy_id, stop_previous).await;
    }

    pub async fn looping_suction(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) {
        self.looping_function("Suction", strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await;
    }

    pub async fn single_all(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) {
        self.single_function("All", strength, time_sec, toy_id, stop_previous).await;
    }

    pub async fn looping_all(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) {
        self.looping_function("All", strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await;
    }
}