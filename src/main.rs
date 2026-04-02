use clap::Parser;
use icalendar::Calendar;
use parse_datetime::parse_datetime;
use std::fs;
use timew_ical::*;

fn main() -> Result<(), std::io::Error> {
    let args = Arguments::parse();

    let start_dt = parse_datetime(args.start);
    let end_dt = parse_datetime(args.end);

    let timew_bytes = TimewOutBytes::new(start_dt.unwrap(), end_dt.unwrap());
    let timew_string: TimewOutString = timew_bytes.into();
    let time_data: TimeData = timew_string.into();
    let calendar: Calendar = time_data.create_calendar();

    fs::write("/home/kaheroh/time_logging.ics", format!("{calendar}"))?;
    Ok(())
}
