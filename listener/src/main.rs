#[macro_use]
extern crate rocket;

use std::net::IpAddr;

use rocket::launch;

#[launch]
fn rocket() -> _ {
    env_logger::init();

    rocket::build().mount("/", routes![get_ip])
}

#[get("/ip")]
fn get_ip(ip_addr: IpAddr) -> String {
    info!("Received request from IP {ip_addr}");
    let ip_type = if ip_addr.is_ipv4() { "IPv4" } else { "IPv6" };

    format!("{ip_type} {}", ip_addr.to_string())
}
