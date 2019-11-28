#[macro_use]
extern crate clap;
use clap::{App,Arg};
use chrono::prelude::*;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("dates")
                .help("input dates to use")
                .multiple(true)
                .required(true),

        )
        .get_matches();

    for matched in matches.values_of("dates").unwrap() {
        match DateTime::parse_from_rfc3339(matched) {
            Err(_) => eprintln!("Failed to parse \"{}\"", matched),
            Ok(datetime) => {
                let duration = Utc::now().signed_duration_since(datetime);
                if duration.num_days() > (365 * 2) || (datetime.day() != 1 && duration.num_days() > 15) {
                    println!("{}", matched);
                }
            }
        }
    }
}