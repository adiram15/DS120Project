// formatting and printing of csv output
use crate::analysis::MonthlyStats;
use std::collections::HashMap;

// prints header and rows to standard outp in csv format
// logic: collect keys, sort and print each row with 2 decimal places

pub fn print_report(stats: &HashMap<String, MonthlyStats>) {
    println!("Period, CO_avg, NO2_avg, O3_avg");
    let mut keys: Vec<_> = stats.keys().cloned().collect();
    keys.sort();
    for k in keys {
        let s = &stats[&k];
        println!("{}, {:.2}, {:.2}, {:.2}", k, s.co_avg, s.no2_avg, s.o3_avg);
    }
}
