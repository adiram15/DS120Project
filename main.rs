mod parser; mod analysis; mod utils; mod report;

use crate::parser::parse_csv;
use crate::analysis::{monthly_average, weekly_average, daily_average};
use crate::utils::smooth_stats;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    // reporting period: monthly/weeky/daily
    #[clap(long, default_value = "monthly")]
    period: String,

    // avg window size 
    #[clap(long)]
    smooth: Option<usize>,

    //Path to airquality csv file
    file: String, 
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let cli = Cli::parse();
   let mut entries = parse_csv(&cli.file)?;
   println!("parsed raw entries: {}", entries.len());
   //entries = filter_outliers(&entries);
   let mut stats = match cli.period.as_str() {
        "weekly" => weekly_average(&entries),
        "daily" => daily_average(&entries), 
        _ => monthly_average(&entries), 
   };
   if let Some(window) = cli.smooth {
        let mut vec: Vec<_> = stats.into_iter().collect();
        vec.sort_by_key(|(k, _)| k.clone());
        stats = crate::utils::smooth_stats(&vec, window).into_iter().collect();

   }
   eprintln!("Loaded {} rows", entries.len());
   report::print_report(&stats);
   Ok(())
}
mod tests {
    use super::*;
    use chrono::{FixedOffset, TimeZone};
    use tempfile::NamedTempFile;
    use std::io::Write;
    use crate::parser::AirQualityEntry;
}

#[test]

fn monthly_average_is_correct() {
    // create single entry at 2004-03-10, 18:00:00
    let dt = FixedOffset::east(0).unwrap()
        .with_ymd_and_hms(2004, 3, 10, 18, 0, 0)
        .unwrap();
    let entry = AirQualityEntry {
        datetime: dt, 
        co: Some(2.0),
        no2: Some(1.0), 
        o3: Some(3.0),
    };
    let stats = monthly_average(&[entry]);
    let s = &stats["2004-03"];
    assert!((s.co_avg -2.0).abs() < f32::EPSILON);
}

#[test]
fn parse_single_row() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = NamedTempFile::new()?;
    writeln!(f, "Date;Time;CO(GT);NO2(GT);O3(GT)")?;
    writeln!(f, "10/03/2004;18.00.00;2.6;113.0;30.1")?;
    let entries = parse_csv(f.path().to_str().unwrap())?;
    assert_eq!(entries.len(), 1);
    let parsed_co = entries[0].co.unwrap();
    assert_eq!((entries[0].co.unwrap() - 2.6).abs() < f32::EPSILON);
    Ok(())
}
