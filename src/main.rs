#![allow(unused_variables)]
use std::{error::Error, fmt};

use clap::Parser;
use death_calendar::{
    days_left, days_lived, death_day, lifespan_days, lifespan_weeks, weeks_left, weeks_lived,
    years_left, years_lived,
};
use gregorian::Date;
use svg::{node::element::Rectangle, Document, Node};

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
    svg: bool,
}

fn death_info(bday: Date, years: i16) {
    let today: Date = Date::today_utc();
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
    let w = 1920;
    let h = 1080;

    let mut document = Document::new().set("viewBox", (0, 0, w, h));

    let bg = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", "100%")
        .set("height", "100%")
        .set("fill", "white");

    document.append(bg);

    let today = Date::today_utc();
    let end = death_day(bday, lifespan_years);
    let lived = weeks_lived(today, bday);

    let count = bday.days_since(end);

    let weeks_in_year = 52;

    let mut count = 0;
    let mut curr_date = bday;
    let outer_square_size: f64 = (h / weeks_in_year).min(w / lifespan_years).into();
    let padding = outer_square_size as f64 * 0.2;

    while curr_date < end {
        let fill = if curr_date < today { "black" } else { "white" };
        let x_offset = (f64::from(w) - (outer_square_size * f64::from(lifespan_years))) / 2.0;
        let x = f64::from(count / weeks_in_year).mul_add(outer_square_size, x_offset);
        let y_offset = (f64::from(h) - (outer_square_size * f64::from(weeks_in_year))) / 2.0;
        let y = f64::from(count % weeks_in_year).mul_add(outer_square_size, y_offset);
        let square = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", outer_square_size as f64 - padding)
            .set("height", outer_square_size as f64 - padding)
            .set("fill", fill)
            .set("stroke", "black");
        document.append(square);
        /* All this below is just to make sure there are always 52 weeks. To do this, we change the
        number of days in a week and skip the 29th of February whenever it comes. */
        let week_length = if count % 52 == 0 { 8 } else { 7 };
        for _ in 0..week_length {
            curr_date = curr_date.next();
            if curr_date.month() == 2 && curr_date.day() == 29 {
                curr_date = curr_date.next();
            }
        }
        count += 1;
    }

    println!("{}", document.to_string());
}

fn main() {
    // Use these blocks of code for testing purposes.
    // let args = Args::parse_from(["death-calendar", "--svg", "1997-9-2", "-l", "100"]);
    // let args = Args::parse_from(["death-calendar", "2000-01-01", "-l", "100"]);
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
