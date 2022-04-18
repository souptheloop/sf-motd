mod repository;
mod render;
mod models;

#[macro_use] extern crate rocket;

use std::fmt::format;
use crate::models::fleet::Fleet;
use crate::repository::fleets_html::ParseError;


#[get("/")]
async fn index() -> String {
    let eventsResult = repository::fleets_html::get_fleets("https://www.spectre-fleet.space".to_string()).await;
    let events = match eventsResult {
        Ok(events) => events,
        Err(e) => return format!("Error: {}", e)
    };
    let motd = render::fleets::render_motd(&events);
    return motd;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

