use csv::{Writer, WriterBuilder};
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct AnimeRecord {
    pub date: chrono::NaiveDate,
    pub title: String,
    pub episode: f64,
    pub minutes: Option<f64>,
}

fn get_anime_record_path() -> PathBuf {
    dirs::home_dir()
        .expect("I need home dir to work!")
        .join("documents/japanese/anime.csv")
}

impl AnimeRecord {
    pub fn duration(&self) -> f64 {
        match self.minutes {
            Some(m) => m * 60.0,
            _ => 570.0 / 0.45,
        }
    }
}

impl AnimeRecord {
    pub fn all() -> Vec<AnimeRecord> {
        let file = if let Ok(file) = File::open(get_anime_record_path()) {
            file
        } else {
            return Vec::new();
        };
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
    pub fn append(anime_records: Vec<AnimeRecord>) {
        let is_header = !get_anime_record_path().exists();
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(get_anime_record_path())
            .expect("Can't open file");
        let mut wrt = WriterBuilder::new()
            .has_headers(is_header)
            .from_writer(file);
        for ar in &anime_records {
            wrt.serialize(ar).expect("I can't write new record.");
        }
        wrt.flush().expect("I cant't write in file.");
    }
}
