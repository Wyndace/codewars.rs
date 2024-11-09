use std::fmt::Display;
use std::str::FromStr;

enum TrafficLightState {
    Green,
    Yellow,
    Red,
}

impl TrafficLightState {
    pub fn next(&self) -> TrafficLightState {
        match self {
            TrafficLightState::Green => TrafficLightState::Yellow,
            TrafficLightState::Yellow => TrafficLightState::Red,
            TrafficLightState::Red => TrafficLightState::Green,
        }
    }
}

impl FromStr for TrafficLightState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "green" => Ok(TrafficLightState::Green),
            "yellow" => Ok(TrafficLightState::Yellow),
            "red" => Ok(TrafficLightState::Red),
            _ => Err(()),
        }
    }
}

impl Display for TrafficLightState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            TrafficLightState::Green => String::from("green"),
            TrafficLightState::Yellow => String::from("yellow"),
            TrafficLightState::Red => String::from("red"),
        };
        write!(f, "{}", str)
    }
}

pub fn update_light(current: &str) -> String {
    current.parse::<TrafficLightState>().unwrap().next().to_string()
}