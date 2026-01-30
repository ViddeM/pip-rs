use eyre::Context;
use pinger::IpPinger;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    let pinger = IpPinger::builder()
        .with_remote("http://localhost:39393")
        .wrap_err("Invalid remote?")?
        .build()
        .wrap_err("Failed to create IP pinger")?;

    match pinger.ping().await {
        Ok(ip) => println!("Received IP {ip}"),
        Err(errors) => {
            let error_messages = errors
                .into_iter()
                .map(|(remote, error)| format!("\t{remote} :: {error}"))
                .collect::<Vec<_>>();
            eprintln!(
                "Failed to retrieve IP, errors: \n{}",
                error_messages.join("\n")
            )
        }
    };

    Ok(())
}
