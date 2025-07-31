use clap::Parser;
use timew_ical::*;
use parse_datetime::parse_datetime;

fn main() {
    let args = Arguments::parse();

    let start_dt = parse_datetime(args.start);
    let end_dt = parse_datetime(args.end);

    let timew_bytes = TimewOutBytes::new(start_dt.unwrap(), end_dt.unwrap());
    let timew_string: TimewOutString = timew_bytes.into();
    let time_data: TimeData = timew_string.into();

    let first_element: &TimeTrackingEntry = &time_data.entries[0];

    println!("{first_element:#?}")
}


