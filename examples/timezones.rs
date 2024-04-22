use std::io::stdin;

use chrono::Local;
use chrono_tz::Tz;
use human_date_parser::ParseResult::{self, Date, DateTime, Time};

fn main() {
    let time_zones = vec![
        "Antarctica/South_Pole",
        "Europe/Berlin",
        "Europe/London",
        "America/New_York",
        "America/Sao_Paulo",
        "Asia/Seoul",
        "Asia/Tokyo",
        "Asia/Kathmandu",
    ];

    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();

    let times: Vec<(&str, String)> = time_zones
        .iter()
        .map(|tz_str| parse_time(&buffer, tz_str))
        .collect();

    println!();
    println!("{: <25} {: <25}", "Timezone", "Time");
    println!("{:=<25} {:=<25}", "", "");
    for (tz, time) in times {
        println!("{tz: <25} {time: <25}");
    }
}

fn parse_time<'a>(time: &str, tz_str: &'a str) -> (&'a str, String) {
    let time = match human_date_parser::from_human_time(time, &tz_str.parse::<Tz>().unwrap()).unwrap() {
        DateTime(dt) => dt.to_string(),
        Date(d) => d.to_string(),
        Time(t) => t.to_string(),
    };

    (tz_str, time)
}