#[macro_use]
extern crate rocket;

use std::{convert::Infallible, net::IpAddr};

use rocket::{Request, http::Status, launch, outcome::Outcome, request::FromRequest};

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

#[get("/debug/ip")]
fn debug_ip(ips: DebugIps) -> String {
    ips.data
}

struct DebugIps {
    data: String,
}

#[crate::async_trait]
impl<'r> FromRequest<'r> for DebugIps {
    type Error = Infallible;

    async fn from_request(
        request: &'r Request<'_>,
    ) -> Outcome<Self, (Status, Self::Error), Status> {
        let real_ip = request.headers().get("X-Real-IP").collect::<Vec<_>>();
        let forwarded_for = request.headers().get("X-Forwarded-For").collect::<Vec<_>>();

        let data = String::new();
        Outcome::Success(DebugIps { data })
    }
}
