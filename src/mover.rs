// mod requests;
// mod responses;
// extern crate petgraph;
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::graph::{NodeIndex, UnGraph};
use std::collections::HashMap;

use crate::requests;
use crate::responses;
use petgraph::algo::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

fn valid(point: requests::Point, size: i32, visited: &Vec<requests::Point>) -> bool {
  if point.x < 0 || point.y < 0 || point.x <= size || point.y <= size {
    return false;
  }

  if visited.iter().any(|&i| i.x == point.x && i.y == point.y) {
    return false;
  }

  true
}

fn pointToID(point: requests::Point, size: i32) -> i32 {
  point.x * size + point.y
}

fn idToPoint(id: i32, size: i32) -> requests::Point {
  requests::Point::new(id / size, id % size)
}

pub fn next(turn: requests::Turn) -> responses::Move {
  println!("turn: {:?}", turn);
  let board = turn.board;

  let you = turn.you;

  let head = you.head;
  let tail = you.body[you.body.len() - 1];
  let goal = tail;

  let width = board.width as usize;
  let height = board.height as usize;
  let size = width as i32;

  // grid
  let mut grid_raw = vec![0; width * height];
  let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
  let grid = grid_base.as_mut_slice();

  for point in you.body {
    grid[point.y as usize][point.x as usize] = 1;
  }

  let mut queue: Vec<requests::Point> = Vec::new();
  let mut visited: Vec<requests::Point> = Vec::new();

  queue.push(requests::Point::new(1, 1));
  visited.push(requests::Point::new(1, 1));

  let mut parent: requests::Point;

  let mut parents = HashMap::new();

  while let Some(point) = queue.pop() {
    let left = requests::Point::new(point.x - 1, point.y);
    let right = requests::Point::new(point.x + 1, point.y);
    let top = requests::Point::new(point.x, point.y + 1);
    let bottom = requests::Point::new(point.x, point.y - 1);

    for point in [left, right, top, bottom] {
      if point == goal {
        parent = goal;

        let pointID = pointToID(point, size);
        parents.insert(pointID, point);
      }

      if valid(left, board.width, &visited) {
        visited.push(point);
        queue.push(point)
      }
    }
  }

  println!("grid: {:?}", grid);

  let movement = responses::Move::new(responses::Movement::Left);

  return movement;
}

#[cfg(test)]
mod tests {
  use crate::mover;
  use crate::requests;
  use crate::responses;

  #[test]
  fn it_works() {
    let turnData = r#"{
      "game": {
        "id": "game-00fe20da-94ad-11ea-bb37",
        "ruleset": {
          "name": "standard",
          "version": "v.1.2.3"
        },
        "timeout": 500
      },
      "turn": 14,
      "board": {
        "height": 11,
        "width": 11,
        "food": [
          {"x": 5, "y": 5},
          {"x": 9, "y": 0},
          {"x": 2, "y": 6}
        ],
        "hazards": [
          {"x": 3, "y": 2}
        ],
        "snakes": [
          {
            "id": "snake-508e96ac-94ad-11ea-bb37",
            "name": "My Snake",
            "health": 54,
            "body": [
              {"x": 0, "y": 0},
              {"x": 1, "y": 0},
              {"x": 2, "y": 0}
            ],
            "latency": "111",
            "head": {"x": 0, "y": 0},
            "length": 3,
            "shout": "why are we shouting??",
            "squad": ""
          },
          {
            "id": "snake-b67f4906-94ae-11ea-bb37",
            "name": "Another Snake",
            "health": 16,
            "body": [
              {"x": 5, "y": 4},
              {"x": 5, "y": 3},
              {"x": 6, "y": 3},
              {"x": 6, "y": 2}
            ],
            "latency": "222",
            "head": {"x": 5, "y": 4},
            "length": 4,
            "shout": "I'm not really sure...",
            "squad": ""
          }
        ]
      },
      "you": {
        "id": "snake-508e96ac-94ad-11ea-bb37",
        "name": "My Snake",
        "health": 54,
        "body": [
          {"x": 0, "y": 0},
          {"x": 1, "y": 0},
          {"x": 2, "y": 0}
        ],
        "latency": "111",
        "head": {"x": 0, "y": 0},
        "length": 3,
        "shout": "why are we shouting??",
        "squad": ""
      }
    }"#;

    let turn: serde_json::Result<requests::Turn> = serde_json::from_str(turnData);

    let tt = turn.unwrap();

    let movement = mover::next(tt);

    assert_eq!(responses::Move::new(responses::Movement::Down), movement);
  }
}
