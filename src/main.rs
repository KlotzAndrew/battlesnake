#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket::http::Status;
use rocket_contrib::json::Json;

mod responses;
mod requests;

#[get("/")]
fn index() -> Json<responses::Info> {
    Json(responses::Info {
        apiversion: "1".to_string(),
        author: Some("klotz".to_string()),
        color: Some("#B22222".to_string()),
        head: Some("tiger-king".to_string()),
        tail: Some("tiger-tail".to_string()),
        version: Some("0.0.1".to_string()),
    })
}

#[post("/start")]
fn start() -> Status {
    Status::Ok
}

#[post("/move", data = "<req>")]
fn movement(req: Json<requests::Turn>) -> Json<responses::Move> {
    let movement = responses::Move::new(responses::Movement::Left);
    Json(movement)
}

#[post("/end")]
fn end() -> Status {
    Status::Ok
}

fn main() {
    rocket::ignite().mount("/", routes![index, start, movement, end]).launch();
}
