
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct Info {
    pub apiversion: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Move {
    #[serde(rename = "move")]
    movement: Movement,
    #[serde(skip_serializing_if = "Option::is_none")]
    shout: Option<String>,
}

impl Move {
    pub fn new(movement: Movement) -> Move {
        Move { movement , shout: None }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all(serialize = "lowercase", deserialize = "lowercase"))]
pub enum Movement {
    Right,
    Left,
    Up,
    Down,
}
