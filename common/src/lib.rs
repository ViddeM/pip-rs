use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    str::FromStr,
};

#[allow(non_upper_case_globals)]
const IPv4: &'static str = "IPv4";
#[allow(non_upper_case_globals)]
const IPv6: &'static str = "IPv6";

pub struct IpResponse {
    addr: IpAddr,
}

impl IpResponse {
    pub fn new(ip: IpAddr) -> Self {
        Self { addr: ip }
    }

    pub fn to_body(self) -> String {
        let (ip_type, ip) = match self.addr {
            IpAddr::V4(ipv4_addr) => (IPv4, ipv4_addr.to_string()),
            IpAddr::V6(ipv6_addr) => (IPv6, ipv6_addr.to_string()),
        };

        format!("{ip_type} {ip}")
    }

    pub fn ip_addr(self) -> IpAddr {
        self.addr
    }

    pub fn parse(text: String) -> Result<Self, String> {
        let Some((ip_type, ip)) = text.split_once(" ") else {
            return Err("Failed to parse: missing required space".to_string());
        };

        #[allow(non_upper_case_globals)]
        let addr = match ip_type {
            IPv4 => {
                let ipv4 = Ipv4Addr::from_str(ip)
                    .map_err(|err| format!("Failed to parse IPv4 addr {err}"))?;
                IpAddr::V4(ipv4)
            }
            IPv6 => {
                let ipv6 = Ipv6Addr::from_str(ip)
                    .map_err(|err| format!("Failed to parse IPv6 addr {err}"))?;
                IpAddr::V6(ipv6)
            }
            unknown => return Err(format!("Unsupported or invalid IP type {unknown}")),
        };

        Ok(Self { addr })
    }
}
