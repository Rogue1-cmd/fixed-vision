// import Rocket
#[macro_use] extern crate rocket;

// add routes and services modules
mod routes;
mod services;

// import routes
use routes::date::get_current_date;
use routes::date::date_plus_month;

// start the web server and mount get route at "/api".

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![get_current_date, date_plus_month])
}