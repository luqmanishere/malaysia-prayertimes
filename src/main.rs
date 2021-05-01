mod prayertime;
use clap::{crate_version, App, Arg};
use prayertime::PrayerData;

// The whole reason this is async is because of reqwest. There's probably a lighter library out there.
// I did not know of the existance of a blocking client from reqwest prior to writing this program.
// Anyways, forgive the absurd amount of libraries. I am just too lazy to reinvent the wheel.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // List of zones defined by JAKIM
    let possible_zones = &[
        "JHR01", "JHR02", "JHR03", "JHR04", "KDH01", "KDH02", "KDH03", "KDH04", "KDH05", "KDH06",
        "KDH07", "KTN01", "KTN03", "MLK01", "NGS01", "NGS02", "PHG01", "PHGO2", "PHGO3", "PHGO4",
        "PHGO5", "PHGO6", "PLS01", "PNG01", "PRK01", "PRK02", "PRK03", "PRK04", "PRK05", "PRK06",
        "PRK07", "SBH01", "SBH02", "SBH03", "SBH04", "SBH05", "SBH06", "SBH07", "SBH08", "SBH09",
        "SGR01", "SGR02", "SGR03", "SWK01", "SWK02", "SWK03", "SWK04", "SWK05", "SWK06", "SWK07",
        "SWK08", "SWK09", "TRG01", "TRG02", "TRG03", "TRG04", "WLY01", "WLY02",
    ];
    // Initialize clap
    let matches = App::new("praytime")
        .version(crate_version!())
        .author("Luqmanul Hakim <luqmanulhakim1720@gmail.com>")
        .about("Fetches official prayer times for the area specified from JAKIM")
        .long_about(
            "Fetches official prayer times for the area specified from JAKIM.
If no area is specified, the program defaults to SGR01 (Selangor Zone 01).
Period of times wanted can also be specified
-- Only available for areas governed by JAKIM, ie. Malaysia --",
        )
        // TODO Implement reading settings from configuration files
        // Allow users to specify zone
        .arg(
            Arg::with_name("zone")
                .short("z")
                .long("zone")
                .value_name("ZONE")
                .default_value("SGR01")
                .takes_value(true)
                .possible_values(possible_zones)
                .hide_possible_values(true)
                .help("Specify the zone to lookup times for"),
        )
        // allow user to specify duration of
        .arg(
            Arg::with_name("period")
                .short("p")
                .long("period")
                .value_name("PERIOD")
                .default_value("today")
                .takes_value(true)
                .help("Specify period of times wanted"),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List available zones"),
        )
        .get_matches();

    // Check for arguments
    let zone = matches.value_of("zone");
    let period = matches.value_of("period");
    if matches.is_present("list") {
        // TODO Actually print the values nicely
        println!("{:?}", possible_zones);
        // Terminate the program after printing values
        return Ok(());
    }

    // Initiallize prayer data with options
    let prayer_data = PrayerData::from_options(zone, period).await?;
    // By default print today's prayer times
    prayer_data.print_waktu_solat_today();

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
