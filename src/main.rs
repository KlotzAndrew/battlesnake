#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

use rocket::http::Status;
use rocket_contrib::json::Json;

mod responses;

#[get("/")]
fn index() -> Json<responses::Info> {
    Json(responses::Info {
        apiversion: "1".to_string(),
        author: None,
        color: Some("#b7410e".to_string()),
        head: None,
        tail: None,
        version: Some("0".to_string()),
    })
}

#[post("/start")]
fn start() -> Status {
    Status::Ok
}

#[post("/move")]
fn movement() -> Status {
    Status::Ok
}

#[post("/end")]
fn end() -> Status {
    Status::Ok
}

fn main() {
    rocket::ignite().mount("/", routes![index, start, movement, end]).launch();
}
