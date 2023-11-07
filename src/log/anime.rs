use crate::data::AnimeRecord;
use clap::{Parser, Subcommand};
use skim::prelude::{Skim, SkimItemReader, SkimOptionsBuilder};
use std::collections::HashSet;
use std::io::{stdin, stdout, Cursor, Write};
use std::str::FromStr;
#[derive(clap::Args)]
pub struct AnimeArgs {
    /// Name of the anime you want to log
    #[arg(short, long)]
    title: Option<String>,
    /// Date of a logged record
    #[arg(short, long)]
    date: Option<chrono::NaiveDate>,
    /// First episode of current logging
    #[arg(short, long)]
    start: Option<f64>,
    /// Last episode of current logging
    #[arg(short, long)]
    end: Option<f64>,
    /// Episode count
    #[arg(short, long)]
    count: Option<i64>,
    /// Duration of each episode
    #[arg(short, long)]
    minutes: Option<f64>,
}

fn get_titles(anime_records: &Vec<AnimeRecord>) -> Vec<String> {
    let mut titles: Vec<String> = anime_records
        .iter()
        .map(|item| item.title.clone())
        .collect();
    titles.reverse();
    let mut unique_titles: Vec<String> = Vec::new();
    for title in titles {
        if !unique_titles.contains(&title) {
            unique_titles.push(title);
        }
    }
    unique_titles
}

fn prompt_string(message: String) -> String {
    print!("{}: ", message);
    stdout().flush().unwrap();
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Can't read input");
    user_input.trim().to_string()
}

fn prompt_numeric<T>(message: &str, default: Option<T>) -> T
where
    T: FromStr + std::fmt::Display + Copy,
{
    if let Some(d) = default {
        print!("{} [{}]: ", message, d);
    } else {
        print!("{}: ", message);
    }
    stdout().flush().unwrap();
    let mut user_input = String::new();
    stdin()
        .read_line(&mut user_input)
        .expect("Can't read input");
    let user_input = user_input.trim();
    if user_input.is_empty() {
        if let Some(d) = default {
            d
        } else {
            prompt_numeric(message, default)
        }
    } else {
        if let Ok(s) = user_input.parse::<T>() {
            s
        } else {
            prompt_numeric(message, default)
        }
    }
}

fn menu(message: &str, values: Vec<String>, new: bool) -> Option<String> {
    let options = SkimOptionsBuilder::default().build().unwrap();
    let input = if new {
        format!("{}\n{}", "New ", values.join("\n"))
    } else {
        values.join("\n")
    };
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));
    let selected_item = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());
    let selected_item = selected_item.get(0);
    let selected_item = if let Some(si) = selected_item {
        si.output()
    } else {
        return None;
    };
    println!("\x1B[0m");
    if selected_item == "New " {
        return None;
    } else {
        println!("{message}: {selected_item}");
        Some(selected_item.to_string())
    }
}

impl AnimeArgs {
    pub fn run(&self) {
        let anime_records = AnimeRecord::all();
        let titles = get_titles(&anime_records);
        let today = chrono::Utc::now().date_naive();
        // Start "Title"
        let title = if let Some(t) = &self.title {
            t.clone()
        } else {
            let selected_item = menu("Title", titles, true);
            if let Some(s) = selected_item {
                s
            } else {
                prompt_string(String::from("Title"))
            }
        };
        // End "Title"
        let date = if let Some(d) = self.date { d } else { today };
        // Start "Start"
        let filtred_anime_records: Vec<AnimeRecord> = anime_records
            .iter()
            .filter(|item| item.title == title)
            .cloned()
            .collect();
        let default_start = if filtred_anime_records.is_empty() {
            1.0
        } else {
            if let Some(anime_record) = filtred_anime_records.last() {
                anime_record.episode + 1.0
            } else {
                1.0
            }
        };
        let start = if let Some(s) = self.start {
            s
        } else {
            prompt_numeric("Start episode", Some(default_start))
        };
        // End "Start"
        // Start "End"
        let end = if let Some(e) = self.end {
            e
        } else {
            prompt_numeric("End episode", Some(default_start))
        };
        // End "End"
        // Start "Count"
        let default_count = (end - start) as i64 + 1;
        let count = if let Some(s) = self.count {
            s
        } else {
            prompt_numeric("Episode count", Some(default_count))
        };
        // End "Count"
        let default_minutes = 9.5 / 0.45;
        // Start "Minutes"
        let minutes = if let Some(s) = self.minutes {
            Some(s)
        } else {
            let m = prompt_numeric("Minutes", Some(default_minutes));
            if default_minutes == m {
                None
            } else {
                Some(m)
            }
        };
        // End "Minutes"
        let mut anime_records: Vec<AnimeRecord> = Vec::new();
        if default_count == count {
            let mut current = start;
            for _ in 0..count {
                anime_records.push(AnimeRecord {
                    title: title.clone(),
                    date: date,
                    episode: current,
                    minutes: minutes,
                });
                current += 1.0;
            }
        } else {
            println!("Enter episode number for every record.");
            for _ in 0..count {
                let episode = prompt_numeric::<f64>("Episode number", None);
                anime_records.push(AnimeRecord {
                    title: title.clone(),
                    date: date,
                    episode: episode,
                    minutes: minutes,
                });
            }
        }
        AnimeRecord::append(anime_records)
    }
}
