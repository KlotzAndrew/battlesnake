use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Debug, Copy, Clone)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

impl Point {
  pub fn new(x: i32, y: i32) -> Point {
    Point { x, y }
  }
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Turn {
  pub game: Game,
  pub turn: u32,
  pub board: Board,
  pub you: Snake,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Game {
  pub id: String,
  pub timeout: i32,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
pub struct Board {
  pub height: i32,
  pub width: i32,
  pub food: Vec<Point>,
  pub snakes: Vec<Snake>,
  pub hazards: Vec<Point>,
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Snake {
  pub id: String,
  pub name: String,
  pub health: i32,
  pub body: Vec<Point>,
  pub head: Point,
  pub length: u32,
  pub shout: String,
  pub squad: Option<String>,
  pub latency: String,
}
