#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::State;
use std::sync::atomic::{AtomicUsize, Ordering};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

struct HitCount(AtomicUsize);

#[get("/count")]
fn count(hit_count: State<HitCount>) -> String {
    hit_count.0.fetch_add(1, Ordering::Relaxed).to_string()
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, count])
        .manage(HitCount(AtomicUsize::new(0)))
        .launch();
}
