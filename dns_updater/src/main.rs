use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use clap::Parser;
use cloudflare::{
    endpoints::{
        self,
        dns::dns::{DnsContent, ListDnsRecords, ListDnsRecordsParams},
    },
    framework::{
        Environment,
        auth::Credentials,
        client::{ClientConfig, blocking_api::HttpApiClient},
    },
};
use eyre::Context;
use pinger::IpPinger;

pub mod cli;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let args = cli::Args::parse();

    let mut pinger = IpPinger::builder();
    for remote in args.remotes.iter() {
        pinger = pinger
            .with_remote(remote)
            .wrap_err_with(|| format!("invalid remote {remote}"))?;
    }

    let pinger = pinger.build().wrap_err("Failed to create IP pinger")?;

    let credentials = Credentials::UserAuthToken {
        token: args.cloudflare_auth_key,
    };
    let cloudflare_client = HttpApiClient::new(
        credentials,
        ClientConfig::default(),
        Environment::Production,
    )
    .wrap_err("Failed to setup cloudflare client")?;

    let ip = match pinger.ping().await {
        Ok(ip) => ip,
        Err(errors) => {
            let error_messages = errors
                .into_iter()
                .map(|(remote, error)| format!("\t{remote} :: {error}"))
                .collect::<Vec<_>>();
            eyre::bail!(
                "Failed to retrieve IP, errors: \n{}",
                error_messages.join("\n")
            );
        }
    };

    for zone_id in args.zone_ids.iter() {
        let mut params = ListDnsRecordsParams::default();

        // TODO: I'm assuming that the addr here doesn't matter since in the API it only requires the type?
        let record_type = match &ip {
            IpAddr::V4(_) => DnsContent::A {
                content: Ipv4Addr::UNSPECIFIED,
            },
            IpAddr::V6(_) => DnsContent::AAAA {
                content: Ipv6Addr::UNSPECIFIED,
            },
        };
        params.record_type = Some(record_type);

        let list_records = ListDnsRecords {
            zone_identifier: zone_id.as_str(),
            params: params,
        };
        let list_response = cloudflare_client
            .request(&list_records)
            .wrap_err("failed to send list request to cloudflare")?;
    }

    Ok(())
}
