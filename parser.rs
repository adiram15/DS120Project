use csv;
use chrono::{DateTime, NaiveDateTime, FixedOffset, TimeZone};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RawRecord {
    #[serde(rename = "Date")]
    date: String,

    #[serde(rename = "Time")]
    time: String,

    #[serde(rename = "CO(GT)")]
    co: String,

    #[serde(rename = "NO2(GT)")]
    no2: String,

    #[serde(rename = "PT08.S5(O3)")]
    o3: String, 
}
#[derive(Clone)]
pub struct AirQualityEntry {
    pub datetime: DateTime<FixedOffset>, 
    pub co: Option<f32>,
    pub no2: Option<f32>,
    pub o3: Option<f32>,
}

pub fn parse_csv(path: &str) -> Result<Vec<AirQualityEntry>, Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .flexible(true)
        .has_headers(true)
        .from_path(path)?;
    let mut out = Vec::new();
    for result in rdr.deserialize() {
        let raw: RawRecord = result?;
        if raw.date.trim().is_empty() || raw.time.trim().is_empty() {
            continue;
        }
        let dt_str = format!("{} {}", raw.date, raw.time);
        //convert comma and decimal into f32
        let naive = NaiveDateTime::parse_from_str(&dt_str,"%d/%m/%Y %H.%M.%S")
            .map_err(|e| format!("Bad timestamp '{}':{}", dt_str, e))?;
        let tz = FixedOffset::east_opt(0).unwrap();
        let datetime = tz
            .from_local_datetime(&naive)
            .single()
            .ok_or_else(|| format!("Ambiguous datetime '{}", dt_str))?;
        let parsed: Option<f32> = raw.co.replace(',', ".").parse().ok();
        let co = raw.co.replace(',', ".").parse::<f32>().ok()
            .filter(|&v| v>= 0.0);
        let no2 = raw.no2.replace(',', ".").parse::<f32>().ok()
            .filter(|&v| v>= 0.0);
        let o3 = raw.o3.replace(',', ".").parse::<f32>().ok()
            .filter(|&v| v>= 0.0);
        out.push(AirQualityEntry { datetime, co, no2, o3});

    }
    Ok(out)
}
