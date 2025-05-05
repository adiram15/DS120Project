use std::collections::HashMap;
use crate::parser::AirQualityEntry;
use chrono::Datelike;

//holds running sums of each pollutant and count of entries
// used to compute avgs
struct Accum { sum_co: f32, sum_no2: f32, sum_o3: f32, count: u32}

// stores final avg polutant levels for any given period
#[derive(Clone)]
pub struct MonthlyStats { pub co_avg: f32, pub no2_avg: f32, pub o3_avg: f32}

// list of entries by user provided function, then computing averages for CO, NO2, O3
// the output is a hashmap that mapps each key to its monthlystats
fn aggregate_by<F>(entries: &[AirQualityEntry], key_fn: F) -> HashMap<String, MonthlyStats>
where F: Fn(&AirQualityEntry) -> String {
    let mut map: HashMap<String, Accum> = HashMap::new();
    //loop through each entry and group
    for e in entries {
        let k = key_fn(e); // 2025-W18
        let a = map.entry(k).or_insert(Accum{sum_co: 0.,sum_no2:0.,sum_o3:0.,count:0});
        if let Some(v) = e.co { a.sum_co += v; }
        if let Some(v) = e.no2 { a.sum_no2 += v; }
        if let Some(v) = e.o3 {a.sum_o3 += v; }
        a.count += 1;
    }
    map.into_iter().map(|(k, a)| {
        let c = a.count as f32;
        (k, MonthlyStats{co_avg:a.sum_co/c, no2_avg:a.sum_no2/c, o3_avg:a.sum_o3/c})
    }).collect()
}

// computes monthl averages (YYYY-MM)
pub fn monthly_average(entries: &[AirQualityEntry]) -> HashMap<String, MonthlyStats> {
    aggregate_by(entries, |e| e.datetime.format("%Y-%m").to_string())
}
// weekly avgs (YYYY-www)
pub fn weekly_average(entries: &[AirQualityEntry]) -> HashMap<String, MonthlyStats> { 
    aggregate_by(entries, |e| {
        let iso = e.datetime.iso_week();
        format!("{}-W{:02}", iso.year(), iso.week())
    })
}

// daily avgs (YYYY-mm-dd)
pub fn daily_average(entries: &[AirQualityEntry]) -> HashMap<String, MonthlyStats> {
    aggregate_by(entries, |e| e.datetime.format("%Y-%m-%d").to_string())
}