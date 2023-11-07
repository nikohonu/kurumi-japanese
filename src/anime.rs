use chrono::{Duration, NaiveDate};
use std::fs::File;

#[derive(Debug, serde::Deserialize)]
pub struct AnimeRecord {
    pub date: NaiveDate,
    title: String,
    episode: f64,
    pub minutes: Option<f64>,
}

impl AnimeRecord {
    pub fn duration(&self) -> f64 {
        match self.minutes {
            Some(m) => m * 60.0,
            _ => 570.0 / 0.45,
        }
    }
}

pub fn get_anime_records() -> Vec<AnimeRecord> {
    let file_path = dirs::home_dir()
        .expect("I need home dir to work!")
        .join("documents/japanese/anime.csv");
    let file = File::open(file_path).expect("Can't open file.");
    let mut reader = csv::Reader::from_reader(file);
    let mut anime_records: Vec<AnimeRecord> = vec![];
    for result in reader.deserialize::<AnimeRecord>() {
        match result {
            Ok(am) => anime_records.push(am),
            _ => {}
        }
    }
    anime_records
}
