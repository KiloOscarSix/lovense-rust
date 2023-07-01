use crate::action::Action;
use crate::strength::Strength;
use reqwest::{Client, Error, Response};
use serde_json::Number;
use serde_json::{json, Value};
use std::cmp;

pub struct LovenseClient {
    local_ip: String,
    http_port: Option<u16>,
    https_port: Option<u16>,
}

impl LovenseClient {
    pub fn new(local_ip: &str, https_port: u16) -> LovenseClient {
        let local_ip = String::from(local_ip);
        LovenseClient {
            local_ip,
            https_port: Some(https_port),
            http_port: None,
        }
    }

    pub fn new_http(local_ip: &str, http_port: u16) -> LovenseClient {
        LovenseClient {
            local_ip: String::from(local_ip),
            http_port: Some(http_port),
            https_port: None,
        }
    }

    async fn send_request(&self, endpoint: &str, data: Value) -> Result<Response, Error> {
        let client = Client::new();

        let url = match (self.http_port, self.https_port) {
            (None, None) => panic!("No port specified"),
            (Some(http_port), None) => {
                format!("http://{}:{}/{}", self.local_ip, http_port, endpoint)
            }
            (None, Some(https_port)) => {
                format!("https://{}:{}/{}", self.local_ip, https_port, endpoint)
            }
            (Some(_), Some(https_port)) => {
                format!("https://{}:{}/{}", self.local_ip, https_port, endpoint)
            }
        };

        client.post(&url).json(&data).send().await
    }

    fn create_common_data(
        &self,
        action: String,
        time_sec: f32,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Value {
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
    pub async fn single_function(
        &self,
        action: Action,
        strength: Strength,
        time_sec: f32,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        let strength = match action {
            Action::Pump => cmp::min(strength as i32, 3),
            _ => strength as i32,
        };

        let action = format!("{}:{}", action.to_string(), strength);

        let data = self.create_common_data(action, time_sec, toy_id, stop_previous);

        self.send_request("command", data).await
    }

    pub async fn looping_function(
        &self,
        action: Action,
        strength: Strength,
        time_sec: f32,
        loop_running_sec: f64,
        loop_pause_sec: f64,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        let strength = match action {
            Action::Pump => cmp::min(strength as i32, 3),
            _ => strength as i32,
        };

        let action = format!("{}:{}", action.to_string(), strength);

        let mut data = self.create_common_data(action, time_sec, toy_id, stop_previous);
        data["loopRunningSec"] = Value::Number(Number::from_f64(loop_running_sec).unwrap());
        data["loopPauseSec"] = Value::Number(Number::from_f64(loop_pause_sec).unwrap());

        self.send_request("command", data).await
    }

    pub async fn single_vibrate(
        &self,
        strength: Strength,
        time_sec: f32,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.single_function(Action::Vibrate, strength, time_sec, toy_id, stop_previous)
            .await
    }

    pub async fn looping_vibrate(
        &self,
        strength: Strength,
        time_sec: f32,
        loop_running_sec: f64,
        loop_pause_sec: f64,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.looping_function(
            Action::Vibrate,
            strength,
            time_sec,
            loop_running_sec,
            loop_pause_sec,
            toy_id,
            stop_previous,
        )
        .await
    }

    pub async fn single_rotate(
        &self,
        strength: Strength,
        time_sec: f32,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.single_function(Action::Rotate, strength, time_sec, toy_id, stop_previous)
            .await
    }

    pub async fn looping_rotate(
        &self,
        strength: Strength,
        time_sec: f32,
        loop_running_sec: f64,
        loop_pause_sec: f64,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.looping_function(
            Action::Rotate,
            strength,
            time_sec,
            loop_running_sec,
            loop_pause_sec,
            toy_id,
            stop_previous,
        )
        .await
    }

    pub async fn single_pump(
        &self,
        strength: Strength,
        time_sec: f32,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.single_function(Action::Pump, strength, time_sec, toy_id, stop_previous)
            .await
    }

    pub async fn looping_pump(
        &self,
        strength: Strength,
        time_sec: f32,
        loop_running_sec: f64,
        loop_pause_sec: f64,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.looping_function(
            Action::Pump,
            strength,
            time_sec,
            loop_running_sec,
            loop_pause_sec,
            toy_id,
            stop_previous,
        )
        .await
    }

    pub async fn single_thrusting(
        &self,
        strength: Strength,
        time_sec: f32,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.single_function(Action::Thrusting, strength, time_sec, toy_id, stop_previous)
            .await
    }

    pub async fn looping_thrusting(
        &self,
        strength: Strength,
        time_sec: f32,
        loop_running_sec: f64,
        loop_pause_sec: f64,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.looping_function(
            Action::Thrusting,
            strength,
            time_sec,
            loop_running_sec,
            loop_pause_sec,
            toy_id,
            stop_previous,
        )
        .await
    }

    pub async fn single_fingering(
        &self,
        strength: Strength,
        time_sec: f32,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.single_function(Action::Fingering, strength, time_sec, toy_id, stop_previous)
            .await
    }

    pub async fn looping_fingering(
        &self,
        strength: Strength,
        time_sec: f32,
        loop_running_sec: f64,
        loop_pause_sec: f64,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.looping_function(
            Action::Fingering,
            strength,
            time_sec,
            loop_running_sec,
            loop_pause_sec,
            toy_id,
            stop_previous,
        )
        .await
    }

    pub async fn single_suction(
        &self,
        strength: Strength,
        time_sec: f32,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.single_function(Action::Suction, strength, time_sec, toy_id, stop_previous)
            .await
    }

    pub async fn looping_suction(
        &self,
        strength: Strength,
        time_sec: f32,
        loop_running_sec: f64,
        loop_pause_sec: f64,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.looping_function(
            Action::Suction,
            strength,
            time_sec,
            loop_running_sec,
            loop_pause_sec,
            toy_id,
            stop_previous,
        )
        .await
    }

    pub async fn single_all(
        &self,
        strength: Strength,
        time_sec: f32,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.single_function(Action::All, strength, time_sec, toy_id, stop_previous)
            .await
    }

    pub async fn looping_all(
        &self,
        strength: Strength,
        time_sec: f32,
        loop_running_sec: f64,
        loop_pause_sec: f64,
        toy_id: Option<String>,
        stop_previous: bool,
    ) -> Result<Response, Error> {
        self.looping_function(
            Action::All,
            strength,
            time_sec,
            loop_running_sec,
            loop_pause_sec,
            toy_id,
            stop_previous,
        )
        .await
    }
}
