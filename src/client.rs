use std::cmp;
use reqwest::{Client, Error, Response};
use serde_json::{json, Value};
use serde_json::Number;
use crate::lovense_action::LovenseAction;


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

        let url = match (self.http_port, self.https_port) {
            (None, None) => panic!("No port specified"),
            (Some(http_port), None) => format!("http://{}:{}/{}", self.local_ip, http_port, endpoint),
            (None, Some(https_port)) => format!("https://{}:{}/{}", self.local_ip, https_port, endpoint),
            (Some(_), Some(https_port)) => format!("https://{}:{}/{}", self.local_ip, https_port, endpoint),
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

    /// Strength: 0-20 except for Pump, which is 0-3
    /// Time (sec): 0 = indefinite length. Otherwise, running time should be greater than 1.
    /// Stop previous: Stop all previous commands and execute current commands
    pub async fn single_function(&self, action: LovenseAction, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        let strength = match action {
            LovenseAction::Pump => cmp::min(strength, 3),
            _ => cmp::min(strength, 20)
        };

        if strength < 0 {
            panic!("Strength must be greater than 0");
        }

        let action = format!("{}:{}", action.to_string(), strength);

        let data = self.create_common_data(action, time_sec, toy_id, stop_previous);

        self.send_request("command", data).await
    }

    pub async fn looping_function(&self, action: LovenseAction, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        let strength = match action {
            LovenseAction::Pump => cmp::min(strength, 3),
            _ => cmp::min(strength, 20)
        };

        if strength < 0 {
            panic!("Strength must be greater than 0");
        }

        let action = format!("{}:{}", action.to_string(), strength);

        let mut data = self.create_common_data(action, time_sec, toy_id, stop_previous);
        data["loopRunningSec"] = Value::Number(Number::from_f64(loop_running_sec).unwrap());
        data["loopPauseSec"] = Value::Number(Number::from_f64(loop_pause_sec).unwrap());

        self.send_request("command", data).await
    }

    pub async fn single_vibrate(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.single_function(LovenseAction::Vibrate, strength, time_sec, toy_id, stop_previous).await
    }

    pub async fn looping_vibrate(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.looping_function(LovenseAction::Vibrate, strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await
    }

    pub async fn single_rotate(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.single_function(LovenseAction::Rotate, strength, time_sec, toy_id, stop_previous).await
    }

    pub async fn looping_rotate(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.looping_function(LovenseAction::Rotate, strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await
    }

    pub async fn single_pump(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.single_function(LovenseAction::Pump, strength, time_sec, toy_id, stop_previous).await
    }

    pub async fn looping_pump(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.looping_function(LovenseAction::Pump, strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await
    }

    pub async fn single_thrusting(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.single_function(LovenseAction::Thrusting, strength, time_sec, toy_id, stop_previous).await
    }

    pub async fn looping_thrusting(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.looping_function(LovenseAction::Thrusting, strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await
    }

    pub async fn single_fingering(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.single_function(LovenseAction::Fingering, strength, time_sec, toy_id, stop_previous).await
    }

    pub async fn looping_fingering(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.looping_function(LovenseAction::Fingering, strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await
    }

    pub async fn single_suction(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.single_function(LovenseAction::Suction, strength, time_sec, toy_id, stop_previous).await
    }

    pub async fn looping_suction(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.looping_function(LovenseAction::Suction, strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await
    }

    pub async fn single_all(&self, strength: i32, time_sec: f32, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.single_function(LovenseAction::All, strength, time_sec, toy_id, stop_previous).await
    }

    pub async fn looping_all(&self, strength: i32, time_sec: f32, loop_running_sec: f64, loop_pause_sec: f64, toy_id: Option<String>, stop_previous: bool) -> Result<Response, Error> {
        self.looping_function(LovenseAction::All, strength, time_sec, loop_running_sec, loop_pause_sec, toy_id, stop_previous).await
    }
}