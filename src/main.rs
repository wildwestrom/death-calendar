#![allow(unused_variables)]
use std::{error::Error, fmt};

use clap::Parser;
use death_calendar::*;
use gregorian::Date;

#[derive(Debug)]
struct ParseDateError;

impl Error for ParseDateError {}

impl fmt::Display for ParseDateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Could not parse input date")
    }
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// A birthday in `YYYY-MM-DD` format.
    birthday: Date,
    /// Expected lifespan in years.
    #[clap(short, long, default_value_t = 100)]
    lifespan_years: i16,
    /// Render svg calendar.
    #[clap(short, long)]
    svg: bool
}

fn death_info(bday: Date, years: i16) {
    let today = Date::today_utc();
    println!("Your birthday is {}.", bday);
    println!("Your estimated lifespan is {} years.", years);
    println!("You will live for approximately:");
    println!("{} days.", lifespan_days(bday, years));
    println!("{} weeks.", lifespan_weeks(bday, years));
    println!("You will probably die around {}.", death_day(bday, years));
    println!("You have lived for");
    println!("- {} days.", days_lived(today, bday));
    println!("- {} weeks.", weeks_lived(today, bday));
    println!("- {} years.", years_lived(today, bday));
    println!("You have remaining:");
    println!("- {} days", days_left(today, bday, years).abs());
    println!("- {} weeks", weeks_left(today, bday, years).abs());
    println!("- {} years", years_left(today, bday, years).abs());
}

fn render_svg(bday: Date, lifespan_years: i16) {
    todo!()
}

fn main() {
    // Use these blocks of code for testing purposes.
    // let args = Args::parse_from(["death-calendar", "69-4-20", "-l", "69", "-s"]);
    // let args = Args::parse_from(["death-calendar", "--help"]);

    let args = Args::parse();
    let bday = args.birthday;
    let lifespan_years = args.lifespan_years;

    if args.svg {
        render_svg(bday, lifespan_years);
    } else {
        death_info(bday, lifespan_years);
    }
}
