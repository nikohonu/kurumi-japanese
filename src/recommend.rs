use crate::anime::get_anime_records;
use chrono::{Duration, NaiveDate};
use std::collections::BTreeMap;

#[derive(clap::Args)]
pub struct RecommendArgs {}

fn seconds_to_hh_mm_ss(seconds: f64) -> String {
    let total_seconds = seconds as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let remaining_seconds = total_seconds % 60;
    format!("{:02}:{:02}:{:02}", hours, minutes, remaining_seconds)
}

fn episodes_from_seconds(seconds: f64) -> f64 {
    seconds / (9.5 / 0.45 * 60.0)
}

impl RecommendArgs {
    pub fn run(&self) {
        let anime_records = get_anime_records();
        let mut date_duration = BTreeMap::new();
        for am in &anime_records {
            println!("{:?}, {}", am, am.duration());
            date_duration
                .entry(am.date)
                .and_modify(|d| *d = *d + am.duration())
                .or_insert(am.duration());
        }
        for dd in &date_duration {
            println!("{:?}", dd);
        }
        let sum: f64 = date_duration.values().sum(); // Convert values to f64 for accurate division
        let count = date_duration.len() as f64; // Get the number of values as f64
        let average = sum / count;
        println!(
            "{} {} {} {}",
            seconds_to_hh_mm_ss(sum),
            seconds_to_hh_mm_ss(average),
            count,
            episodes_from_seconds(average)
        )
    }
}
