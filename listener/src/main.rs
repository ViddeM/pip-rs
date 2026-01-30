#[macro_use]
extern crate rocket;

use std::net::IpAddr;

use common::IpResponse;
use rocket::launch;

#[launch]
fn rocket() -> _ {
    env_logger::init();

    rocket::build().mount("/", routes![get_ip])
}

#[get("/ip")]
fn get_ip(ip_addr: IpAddr) -> String {
    info!("Received request from IP {ip_addr}");
    let ip_response = IpResponse::new(ip_addr);
    ip_response.to_body()
}
