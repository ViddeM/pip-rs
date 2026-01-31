use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long = "remote", action = clap::ArgAction::Append)]
    pub remotes: Vec<String>,
    #[arg(short, long = "zone_id", action = clap::ArgAction::Append)]
    pub zone_ids: Vec<String>,
    #[arg(short, long)]
    pub cloudflare_auth_key: String,
}
