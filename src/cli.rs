use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::{config::Config, prayertime::Zones};

#[derive(Parser)]
#[command(author,version, long_about = None)]
#[command(
    about = "Fetches official prayer times for Malaysian territories using the API exposed by JAKIM, the government-mandated Islamic Authority in Malaysia"
)]
#[command(propagate_version = true)]
pub struct Cli {
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Get todays prayer times
    Today(ZoneInfo),
    List,
}

#[derive(Args)]
pub struct ZoneInfo {
    #[arg(value_enum, default_value_t = get_default_zone())]
    pub zone: crate::prayertime::Zones,
}

#[derive(Args)]
pub struct ZoneTimeInfo {
    #[arg(short, long)]
    pub zone: Option<String>,
    #[arg(short, long)]
    pub time: String,
}

fn get_default_zone() -> Zones {
    if let Ok(config) = Config::new(None) {
        config.get_default_zone()
    } else {
        Zones::SGR01
    }
}
