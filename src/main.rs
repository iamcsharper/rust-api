#![feature(decl_macro, proc_macro_hygiene)]

use rocket::serde::json::Json;
/**
 * TODO:
 * 1) Logging
 * 2) dot env config
 * 3) github actions build / deploy
 * 4) Forms
 * 5) Files
 * 6) Common code
 * 7) Validations
 * 8) Auth
 */
use rust_api::{establish_connection, models::post::NewPost};

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rocket;

// use rocket::{fairing::AdHoc, State};

use diesel::prelude::*;
use rust_api::{models::post::Post, services::*};

lazy_static! {
    // pub static ref RB: Rbatis = Rbatis::new();
}

#[get("/")]
async fn api_list_posts() -> String {
    use rust_api::schema::posts::dsl::*;

    let connection = establish_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    return serde_json::json!(results).to_string();
}

#[post("/<id>/publish")]
fn api_publish_post(id: i32) -> String {
    let connection = establish_connection();

    let result = publish_post(&connection, id);

    return serde_json::json!(result).to_string();
}

#[post("/", format = "json", data = "<post>")]
fn api_create_post(post: Json<NewPost>) -> String {
    let connection = establish_connection();

    println!("body={:?}, title={:?}", post.body, post.title);

    let post = create_post(
        &connection,
        &String::from("My nice title"),
        &String::from("My nice body goes here"),
    );

    return serde_json::json!(post).to_string();
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/posts", routes![api_list_posts, api_create_post, api_publish_post])
        // .attach(AdHoc::on_ignite("Rbatis Database", |rocket| async move {
        //     rocket.manage(RB)
        // }))
        .launch()
        .await?;

    Ok(())
}
