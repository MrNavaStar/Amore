mod storage;

#[macro_use] extern crate rocket;

use rocket::{Build, Rocket};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Tank<> {
    name: String
}

#[get("/")]
fn get_tanks() -> &'static str {
    //storage::TANKS.iter().keys();

    "Here is a list of tanks"
}

#[get("/<name>")]
fn get_tank(name: &str) -> String {
     format!("Here is tank {}", name)
}

#[get("/<name>/devices")]
fn get_tank_devices(name: &str) -> String {
    format!("Here is a list of devices attached to tank {}", name)
}

#[put("/", format = "json", data = "<tank>")]
fn put_tank(tank: Json<Tank>) -> String {
    let name = &tank.name;
    name.to_string()
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/tanks", routes![get_tanks, get_tank, get_tank_devices, put_tank])
}