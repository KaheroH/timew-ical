use clap::Parser;
use chrono::{DateTime, FixedOffset, Utc};
use std::process::Command;
use serde::{Deserialize, Serialize};
use icalendar::{Calendar, CalendarDateTime, Component, Event, EventLike};

#[derive(Parser)]
pub struct Arguments {
    pub start: String,
    pub end: String,
}

#[derive(Debug)]
pub enum TimewOutBytes {
    Output(Vec<u8>),
    Error(Vec<u8>),
}
impl TimewOutBytes {
    pub fn new(start: DateTime<FixedOffset>, end: DateTime<FixedOffset>) -> Self {
        let command_output = Command::new("timew")
            .args([
                "export",
                &start.format("%Y-%m-%d").to_string(),
                "-",
                &end.format("%Y-%m-%d").to_string(),
            ])
            .output()
            .expect("Failed to execute timew process with args");

        let output = command_output.stdout;
        let error = command_output.stderr;
    
        if !error.is_empty() { TimewOutBytes::Error(error) }
        else { TimewOutBytes::Output(output) }
    }
}
impl From<TimewOutBytes> for TimewOutString {
    fn from(bytes: TimewOutBytes) -> Self {
        match bytes {
            TimewOutBytes::Output(out) => match String::from_utf8(out) {
                    Ok(s) => TimewOutString::Output(s.trim_end().to_string()),
                    Err(e) => panic!("The Standard Output stream from timew contained invalid utf8 characters: {e}"),
                },
            TimewOutBytes::Error(err) => match String::from_utf8(err) {
                    Ok(s) => TimewOutString::Error(s.trim_end().to_string()),
                    Err(e) => panic!("The Error stream from timew contained invalid utf8 characters: {e}"),
                },
        }
    }
}

#[derive(Debug)]
pub enum TimewOutString {
    Output(String),
    Error(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeTrackingEntry {
    id: i64,
    #[serde(with = "date_format")]
    start: DateTime<Utc>,
    #[serde(with = "date_format")]
    end: DateTime<Utc>,
    tags: Vec<String>,
}
mod date_format {
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &str = "%Y%m%dT%H%M%SZ";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> 
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeData {
   pub entries: Vec<TimeTrackingEntry>,
}
impl From<TimewOutString> for TimeData {
    fn from(timew_string: TimewOutString) -> Self {
        match timew_string {
            TimewOutString::Output(out) => TimeData { entries: serde_json::from_str(&out).unwrap() },
            TimewOutString::Error(err) => panic!("timew Error: {err}"),
        }
    }
}
impl TimeData {
    pub fn create_calendar(&self) -> Calendar {
        self.entries.iter()
        .map(|entry|
            Event::new()
                .summary(&entry.tags[0])
                .description(&entry.tags[1..].join(","))
                .starts(CalendarDateTime::from(entry.start))
                .ends(CalendarDateTime::from(entry.end))
                .done()
        )
        .collect::<Calendar>()
        .name("Time Logging")
        .done()
    } 
}
