use clap::{App, Arg};
use chrono::{NaiveDate, Utc, Duration};
use std::ops::{Add, Sub};

pub const DATE_FORMAT: &'static str = "%Y-%m-%d";

pub fn resolve_args() -> (String, String, String) {
    let matches = App::new("hnb-exchange")
        .version("0.1.0")
        .arg(Arg::with_name("currency")
            .short("c")
            .long("currency")
            .takes_value(true)
            .help("Currency short name"))
        .arg(Arg::with_name("start date")
            .short("s")
            .long("start-date")
            .takes_value(true)
            .validator(is_date)
            .help("Start date in format yyyy-MM-dd"))
        .arg(Arg::with_name("end date")
            .short("e")
            .long("end-date")
            .takes_value(true)
            .validator(is_date)
            .help("End date in format yyyy-MM-dd"))
        .arg(Arg::with_name("past days")
            .short("p")
            .long("past-days")
            .takes_value(true)
            .conflicts_with_all(&["start date", "end date"])
            .validator(is_num)
            .help("Past days"))
        .get_matches();

    let currency = matches.value_of("currency").unwrap_or("");
    let past_days = matches.value_of("past days").unwrap_or("");

    let (start_date, end_date) =
        if past_days != "" {
            let pd = past_days.parse::<i64>().unwrap(); // we can unwrap we did validation
            let now = Utc::now().date();

            let start = now.sub(Duration::days(pd)).format(DATE_FORMAT).to_string();
            let end = now.add(Duration::days(5)).format(DATE_FORMAT).to_string();

            (start, end)
        } else {
            let now = Utc::now().date();

            let start = matches.value_of("start date")
                .map(|d| d.to_string())
                .unwrap_or(now.format(DATE_FORMAT).to_string());
            let end = matches.value_of("end date")
                .map(|d| d.to_string())
                .unwrap_or(now.add(Duration::days(5)).format(DATE_FORMAT).to_string());

            (start.to_string(), end.to_string())
        };

    (currency.to_string(), start_date, end_date)
}

fn is_date(val: String) -> Result<(), String> {
    NaiveDate::parse_from_str(&val, DATE_FORMAT)
        .map_or_else(|e| Err(e.to_string()), |_| Ok(()))
}

fn is_num(val: String) -> Result<(), String> {
    val.parse::<i64>()
        .map_or_else(|e| Err(e.to_string()), |_| Ok(()))
}
