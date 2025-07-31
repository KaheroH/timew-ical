use clap::Parser;
use chrono::{DateTime, FixedOffset};
use std::process::Command;

#[derive(Parser)]
pub struct Arguments {
    pub start: String,
    pub end: String,
}

#[derive(Debug)]
pub struct TimewOutBytes {
    pub output: Vec<u8>,
    pub error: Vec<u8>,
}
impl TimewOutBytes {
    pub fn new(start: DateTime<FixedOffset>, end: DateTime<FixedOffset>) -> Self {
        let command_output = Command::new("timew")
            .args([
                "export",
                &start.date_naive().format("%Y-%m-%d").to_string(),
                &end.date_naive().format("%Y-%m-%d").to_string(),
            ])
            .output()
            .expect("Failed to execute timew process with args");

        let output = command_output.stdout;
        let error = command_output.stderr;
        
        TimewOutBytes { output, error }
    }
}
impl From<TimewOutBytes> for TimewOutString {
    fn from(bytes: TimewOutBytes) -> Self {
        let output = match String::from_utf8(bytes.output) {
            Ok(s) => s.trim_end().to_string(),
            Err(e) => panic!("timew output contains invalid utf8 characters. timew-ical cannot yet handle these: {e}"),
        };
        let error = match String::from_utf8(bytes.error) {
            Ok(s) => s.trim_end().to_string(),
            Err(e) => panic!("The Error stream from timew contained invalid utf8 characters: {e}"),
        };

        TimewOutString { output, error }
    }
}

#[derive(Debug)]
pub struct TimewOutString {
    pub output: String,
    pub error: String,
}

#[derive(Debug)]
pub struct TimeTrackingEntry {}
#[derive(Debug)]
pub struct TimeData {
   pub entries: Vec<TimeTrackingEntry>, 
}

impl From<TimewOutString> for TimeData {
    fn from(timew_string: TimewOutString) -> Self {
    }
}
