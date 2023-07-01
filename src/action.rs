use std::fmt::Formatter;

pub enum Action {
    Vibrate,
    Rotate,
    Pump,
    Thrusting,
    Fingering,
    Suction,
    All,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
