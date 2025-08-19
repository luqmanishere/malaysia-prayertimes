use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use chrono::prelude::*;
use clap::ValueEnum;
use eyre::{Result, eyre};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum::{Display, EnumIter, VariantArray};

use crate::config::CACHE_DIR;

static CACHE: LazyLock<Mutex<Cache>> =
    LazyLock::new(|| Mutex::new(Cache::new(CACHE_DIR.to_path_buf()).unwrap()));

#[derive(Copy, Clone, Default, Debug, ValueEnum, EnumIter, strum::Display, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[rustfmt::skip]
#[value(rename_all="UPPER")]
pub enum Zones {
    JHR01, JHR02, JHR03, JHR04,
    KDH01, KDH02, KDH03, KDH04, KDH05, KDH06, KDH07,
    KTN01, KTN03,
    MLK01,
    NGS01, NGS02,
    PHG01, PHGO2, PHGO3, PHGO4, PHGO5, PHGO6,
    PLS01, PNG01,
    PRK01, PRK02, PRK03, PRK04, PRK05, PRK06, PRK07,
    SBH01, SBH02, SBH03, SBH04, SBH05, SBH06, SBH07, SBH08, SBH09,
    #[default] SGR01, SGR02, SGR03,
    SWK01, SWK02, SWK03, SWK04, SWK05, SWK06, SWK07, SWK08, SWK09,
    TRG01, TRG02, TRG03, TRG04,
    WLY01, WLY02,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct PrayerData {
    pub prayer_time: Vec<PrayerTime>,
    status: String,
    server_time: String,
    period_type: String,
    lang: String,
    zone: String,
}

impl PrayerData {
    pub async fn from_options(zone: Option<Zones>, time: Option<TimePeriod>) -> Result<PrayerData> {
        let data = PrayerQueryBuilder::new()
            .zone(zone.unwrap_or_default())
            .time(time.unwrap_or_default())
            .run()
            .await?;
        Ok(data)
    }

    #[deprecated]
    #[allow(dead_code)]
    pub fn print_waktu_solat_today(&self) {
        let prayer_time = self.prayer_time.first().unwrap();
        println!(
            "Prayer times for {}, {}, zone {}",
            prayer_time.date.format("%A"),
            prayer_time.date,
            self.zone
        );

        println!("Imsak:   {}", prayer_time.imsak.format("%I:%M %p"));
        println!("Subuh:   {}", prayer_time.fajr.format("%I:%M %p"));
        println!("Zohor:   {}", prayer_time.dhuhr.format("%I:%M %p"));
        println!("Asar:    {}", prayer_time.asr.format("%I:%M %p"));
        println!("Maghrib: {}", prayer_time.maghrib.format("%I:%M %p"));
        println!("Isyak:   {}", prayer_time.isha.format("%I:%M %p"));
    }

    pub async fn print_prayer_time_today(zone: Option<Zones>) -> Result<()> {
        let instance = Self::from_options(zone, Some(TimePeriod::Today)).await?;
        let prayer_time = instance
            .prayer_time
            .first()
            .ok_or_else(|| eyre!("No valid prayer time found"))?;

        instance.print_prayer_time(prayer_time);
        Ok(())
    }

    fn print_prayer_time(&self, prayer_time: &PrayerTime) {
        println!(
            "Prayer times for {}, {}, zone {}",
            prayer_time.date.format("%A"),
            prayer_time.date,
            self.zone
        );

        println!("Imsak:   {}", prayer_time.imsak.format("%I:%M %p"));
        println!("Subuh:   {}", prayer_time.fajr.format("%I:%M %p"));
        println!("Syuruk:  {}", prayer_time.syuruk.format("%I:%M %p"));
        println!("Zohor:   {}", prayer_time.dhuhr.format("%I:%M %p"));
        println!("Asar:    {}", prayer_time.asr.format("%I:%M %p"));
        println!("Maghrib: {}", prayer_time.maghrib.format("%I:%M %p"));
        println!("Isyak:   {}", prayer_time.isha.format("%I:%M %p"));
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrayerTime {
    // TODO Implement DateTime for hijri (assuming 28 days probably)
    pub hijri: String,
    #[serde(deserialize_with = "deserialize_to_date")]
    #[serde(serialize_with = "serialize_to_date")]
    pub date: NaiveDate,
    #[serde(deserialize_with = "deserialize_to_time")]
    #[serde(serialize_with = "serialize_to_time")]
    pub imsak: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    #[serde(serialize_with = "serialize_to_time")]
    pub fajr: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    #[serde(serialize_with = "serialize_to_time")]
    pub syuruk: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    #[serde(serialize_with = "serialize_to_time")]
    pub dhuhr: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    #[serde(serialize_with = "serialize_to_time")]
    pub asr: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    #[serde(serialize_with = "serialize_to_time")]
    pub maghrib: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    #[serde(serialize_with = "serialize_to_time")]
    pub isha: NaiveDateTime,
}

/// Deserializes given value to time
fn deserialize_to_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let now = Local::now().date_naive();
    let naivedate = NaiveDateTime::new(now, NaiveTime::parse_from_str(s, "%H:%M:%S").unwrap());
    Ok(naivedate)
}

fn serialize_to_time<S>(naive_datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = naive_datetime.format("%H:%M:%S").to_string();
    serializer.serialize_str(&s)
}

fn deserialize_to_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let s2 = convert_my_to_en_date(s);
    let naivedate = NaiveDate::parse_from_str(s2.as_str(), "%d-%b-%Y").unwrap();
    Ok(naivedate)
}

fn serialize_to_date<S>(naive_date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = naive_date.format("%d-%b-%Y").to_string();
    serializer.serialize_str(&s)
}

fn convert_my_to_en_date(s: &str) -> String {
    let my = ["Mei", "Ogos", "Okt", "Dis"];
    let en = ["May", "Aug", "Oct", "Dec"];

    match my.iter().enumerate().find_map(|(i, my_month)| {
        if s.contains(my_month) {
            Some(s.replace(my_month, en[i]))
        } else {
            None
        }
    }) {
        Some(month) => month,
        None => s.to_string(),
    }
}

/// Available time periods to query
#[derive(Default, Display, VariantArray, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum TimePeriod {
    #[default]
    Today,
    Week,
    Month,
    Year,
}

pub struct PrayerQueryBuilder {
    zone: Zones,
    time: TimePeriod,
}

impl PrayerQueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn zone(mut self, zone: Zones) -> Self {
        self.zone = zone;
        self
    }

    pub fn time(mut self, time: TimePeriod) -> Self {
        self.time = time;
        self
    }

    pub async fn run(self) -> eyre::Result<PrayerData> {
        const API_URL: &str = "https://www.e-solat.gov.my/index.php?r=esolatApi/TakwimSolat";
        let reply = reqwest::get(format!(
            "{API_URL}&period={period}&zone={zone}",
            period = self.time,
            zone = &self.zone,
        ))
        .await?
        .text()
        .await?;

        let json: PrayerData = serde_json::from_str(&reply)?;

        {
            let mut cache = CACHE.lock().unwrap();
            for ptimes in &json.prayer_time {
                cache.insert(self.zone, ptimes.clone());
            }
            cache.save()?;
        }

        Ok(json)
    }
}

impl Default for PrayerQueryBuilder {
    fn default() -> Self {
        Self {
            zone: Zones::SGR01,
            time: TimePeriod::Today,
        }
    }
}

/// Cache request results. Dead simple JSON implementation
struct Cache {
    cache_file: PathBuf,
    map: HashMap<Zones, HashMap<NaiveDate, CacheEntry>>,
}

impl Cache {
    pub fn new(dir: PathBuf) -> eyre::Result<Self> {
        let cache_file = dir.join("cache.json");
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
        }

        // try reading from file
        let map = if cache_file.exists() {
            if let Ok(read_file) = std::fs::read_to_string(&cache_file)
                && let Ok(map) = serde_json::from_str(&read_file)
            {
                map
            } else {
                Default::default()
            }
        } else {
            Default::default()
        };

        Ok(Self {
            cache_file: dir,
            map: map,
        })
    }

    pub fn insert(&mut self, zone: Zones, ptimes: PrayerTime) {
        let cache_entry = CacheEntry {
            last_update: Local::now().naive_local(),
            data: ptimes.clone(),
        };
        self.map
            .entry(zone)
            .or_default()
            .insert(ptimes.date, cache_entry);
    }

    pub fn save(&self) -> eyre::Result<()> {
        let file_path = self.cache_file.join("cache.json");
        std::fs::write(&file_path, serde_json::to_string(&self.map)?)?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
struct CacheEntry {
    last_update: NaiveDateTime,
    data: PrayerTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_all_time_periods() {
        for period in TimePeriod::VARIANTS {
            println!("Running: {period}");
            let data = PrayerQueryBuilder::default()
                .time(*period)
                .run()
                .await
                .unwrap();
            assert!(data.prayer_time.len() != 0)
        }
    }

    // Commented out until I figure out how to dynamically update the dates
    /*
    use super::*;
    #[test]
    fn deserialize_from_data() {
        let output = r#"PrayerData { prayer_time: [PrayerTime { hijri: "1442-08-25", date: 2021-04-08, imsak: 2021-04-09T05:52:00, fajr: 2021-04-09T06:02:00, syuruk: 2021-04-09T07:09:00, dhuhr: 2021-04-09T13:18:00, asr: 2021-04-09T16:25:00, maghrib: 2021-04-09T19:22:00, isha: 2021-04-09T20:32:00 }], status: "OK!", server_time: "2021-04-09 10:04:55", period_type: "today", lang: "ms_my", zone: "SGR01" }"#;
        let deserialized: PrayerData = serde_json::from_str(
            r#"{"prayerTime":[{"hijri":"1442-08-25","date":"08-Apr-2021","day":"Thursday","imsak":"05:52:00","fajr":"06:02:00","syuruk":"07:09:00","dhuhr":"13:18:00","asr":"16:25:00","maghrib":"19:22:00","isha":"20:32:00"}],"status":"OK!","serverTime":"2021-04-09 10:04:55","periodType":"today","lang":"ms_my","zone":"SGR01","bearing":"291&#176; 7&#8242; 23&#8243;"}"#,
        ).unwrap();
        assert_eq!(output, format!("{:?}", deserialized));
    }
    */
}
