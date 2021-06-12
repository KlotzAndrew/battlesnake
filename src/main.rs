#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::Status;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
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
