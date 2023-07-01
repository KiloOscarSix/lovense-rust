use std::fmt::Formatter;

pub enum LovenseAction {
    Vibrate,
    Rotate,
    Pump,
    Thrusting,
    Fingering,
    Suction,
    All
}

impl std::fmt::Display for LovenseAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}