mod repository;
mod render;
mod models;

#[macro_use] extern crate rocket;


#[get("/")]
async fn index() -> String {
    let events = repository::fleets_html::get_fleets("https://www.spectre-fleet.space".to_string()).await.unwrap();
    let motd = render::fleets::render_motd(&events);
    return motd;
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

