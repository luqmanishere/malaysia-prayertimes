use chrono::prelude::*;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PrayerData {
    pub prayer_time: Vec<PrayerTime>,
    status: String,
    server_time: String,
    period_type: String,
    lang: String,
    zone: String,
}

impl PrayerData {
    pub async fn from_options(
        zone: Option<&str>,
        time: Option<&str>,
    ) -> Result<PrayerData, Box<dyn std::error::Error>> {
        let file = reqwest::get(format!(
            "https://www.e-solat.gov.my/index.php?r=esolatApi/TakwimSolat&period={}&zone={}",
            time.unwrap_or("today"),
            zone.unwrap_or("SGR01")
        ))
        .await?
        .text()
        .await?;
        let deserialized: PrayerData = serde_json::from_str(&file)?;
        Ok(deserialized)
    }
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
}

#[derive(Debug, Deserialize)]
pub struct PrayerTime {
    // TODO Implement DateTime for hijri (assuming 28 days probably)
    pub hijri: String,
    #[serde(deserialize_with = "deserialize_to_date")]
    pub date: NaiveDate,
    #[serde(deserialize_with = "deserialize_to_time")]
    pub imsak: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    pub fajr: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    pub syuruk: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    pub dhuhr: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    pub asr: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    pub maghrib: NaiveDateTime,
    #[serde(deserialize_with = "deserialize_to_time")]
    pub isha: NaiveDateTime,
}

/// Deserializes given value to time
fn deserialize_to_time<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let now = Local::now().date();
    let naivedate = NaiveDateTime::new(
        now.naive_local(),
        NaiveTime::parse_from_str(s, "%H:%M:%S").unwrap(),
    );
    Ok(naivedate)
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

fn convert_my_to_en_date(s: &str) -> String {
    let my = [
        "Jan", "Feb", "Mac", "Apr", "Mei", "Jun", "Jul", "Ogos", "Sep", "Okt", "Nov", "Dis",
    ];
    let en = [
        "Jan", "Feb", "Mac", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];

    if s.contains(my[4]) {
        s.replace(my[4], en[4])
    } else if s.contains(my[7]) {
        s.replace(my[7], en[7])
    } else if s.contains(my[9]) {
        s.replace(my[9], en[9])
    } else if s.contains(my[11]) {
        s.replace(my[11], en[11])
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {

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
