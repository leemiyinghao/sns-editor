#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate diesel;
extern crate dotenv;

use reqwest;
use rocket::{post, routes};
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

pub mod models;
mod orm;
mod tag_parse;

#[derive(Deserialize, Clone)]
struct URLPayload {
    url: String,
}

#[derive(Serialize)]
struct URLPreviewResult {
    title: Option<String>,
    thumbnail: Option<String>,
}

#[post("/", data = "<url_payload>")]
fn preview(url_payload: Json<URLPayload>) -> Json<URLPreviewResult> {
    // check url, download, parse
    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(8))
        .build()
        .unwrap();
    let resp = client
        .get(url_payload.url.clone())
        .header("User-Agent", "facebookexternalhit/1.1")
        .send()
        .unwrap()
        .text()
        .unwrap();

    let tags = tag_parse::parse_from_html(&resp);

    Json(URLPreviewResult {
        title: tags.title,
        thumbnail: tags.thumbnail,
    })
}

fn main() {
    rocket::ignite()
        .mount("/preview", routes![preview])
        .launch();
}
