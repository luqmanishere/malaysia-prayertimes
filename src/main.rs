mod cli;
mod config;
mod prayertime;

use clap::Parser;
use cli::{Cli, Commands};
use prayertime::{PrayerData, Zones};
use strum::IntoEnumIterator;

use crate::config::Config;
// The whole reason this is async is because of reqwest. There's probably a lighter library out there.
// I did not know of the existance of a blocking client from reqwest prior to writing this program.
// Anyways, forgive the absurd amount of libraries. I am just too lazy to reinvent the wheel.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let _config = Config::new(cli.config.as_deref())?;

    match cli.command {
        Commands::Today(zone) => {
            let zone = zone.zone.to_string();
            PrayerData::print_prayer_time_today(Some(&zone)).await?;
        }
        Commands::List => {
            println!("Available zones: ");
            for i in Zones::iter() {
                println!("{}", i);
            }
        }
    };

    Ok(())
}

#[allow(dead_code, unused_variables)]
// TODO Write function to dynamically update available zones
async fn get_waktu_zones() -> Result<(), Box<dyn std::error::Error>> {
    let html = reqwest::get("https://www.e-solat.gov.my/index.php?siteId=24&pageId=24")
        .await?
        .text()
        .await?;
    Ok(())
}
