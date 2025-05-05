//Helpeer function for data cleaning/smooting

use crate::analysis::MonthlyStats;

// removes entries with poolutant values outside realistic human exposure ranges
// #inputs -> slice of AirQualityEntry
// output -> vec<airqualityentry> containing only valid readings
// True purpose: Ensures averages aren't skewed by sensor errors

pub fn filter_outliers(entries: &[crate::parser::AirQualityEntry]) -> Vec<crate::parser::AirQualityEntry> {
    entries
        .iter()
        .filter(|e| {
            //check if pollutant present
            e.co.map_or(true, |v| (0.0..=15.0).contains(&v)) &&
            e.no2.map_or(true, |v| (0.0..=200.0). contains(&v)) &&
            e.o3.map_or(true, |v| (0.0..=200.0).contains(&v))
        })
        .cloned()
        .collect()
}

pub fn smooth_stats(stats: &[(String, MonthlyStats)], window: usize) -> Vec<(String, MonthlyStats)> {
    if window <= 1 { return stats.to_vec(); }
    let mut out = Vec::new();
    for i in 0..stats.len() {
        let start = i.saturating_sub(window - 1);
        let slice = &stats[start..=i];
        let n = slice.len() as f32;
        let (sum_co, sum_no2, sum_o3) = slice.iter().fold((0.0,0.0,0.0), |(c, n, o), (_k, s)| {
            (c+ s.co_avg, n+s.no2_avg, o + s.o3_avg)
        });
        let avg = MonthlyStats { co_avg: sum_co/n, no2_avg: sum_no2/n, o3_avg: sum_o3/n};
        out.push((stats[i].0.clone(), avg));
    }
    out
}