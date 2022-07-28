use std::{error::Error, fmt, num::ParseIntError, path::PathBuf};

use clap::Parser;
use death_calendar::{
    days_left, days_lived, death_day, lifespan_days, lifespan_weeks, weeks_left, weeks_lived,
    years_left, years_lived,
};
use gregorian::Date;
use svg::{node::element::Rectangle, Document, Node};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

// This should work for now until https://github.com/clap-rs/clap/issues/1546 is resolved.
#[derive(Parser, Debug)]
struct CommonArgs {
    /// A birthday in `YYYY-MM-DD` format.
    birthday: Date,
    /// Expected lifespan in years.
    #[clap(short, long, default_value_t = 100)]
    lifespan_years: i16,
}

#[derive(Parser, Debug)]
enum Commands {
    /// Print info about your ultimate demise.
    Info {
        #[clap(flatten)]
        common_args: CommonArgs,
    },
    /// Generate an SVG Image of the calendar.
    Svg {
        #[clap(flatten)]
        common_args: CommonArgs,
        /// Dimensions of your SVG in `WxH` format.
        #[clap(short, long, value_parser, default_value_t = Dimensions {width: 1920, height: 1080})]
        dimensions: Dimensions,
        /// Save SVG to a file instead of printing to stdout.
        #[clap(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Debug, Clone)]
struct Dimensions {
    width: i16,
    height: i16,
}

impl fmt::Display for Dimensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}x{}", self.width, self.height))
    }
}

impl Default for Dimensions {
    fn default() -> Self {
        Self {
            width: Default::default(),
            height: Default::default(),
        }
    }
}

#[derive(Debug)]
struct ParseDimensionsError;

impl Error for ParseDimensionsError {}

impl fmt::Display for ParseDimensionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Could not parse input dimensions. Expected format `WxH`.")
    }
}

impl From<ParseIntError> for ParseDimensionsError {
    fn from(_: ParseIntError) -> Self {
        ParseDimensionsError
    }
}

impl std::str::FromStr for Dimensions {
    type Err = ParseDimensionsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split('x').collect();
        let width = i16::from_str(tokens.first().ok_or(ParseDimensionsError)?)?;
        let height = i16::from_str(tokens.last().ok_or(ParseDimensionsError)?)?;

        Ok(Self { width, height })
    }
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

fn render_svg(bday: Date, years: i16, d: &Dimensions) -> Document {
    let color_primary = "black";
    let color_secondary = "white";

    let mut document = Document::new()
        .set("viewBox", (0, 0, d.width, d.height))
        .set("style", format!("background-color:{}", color_primary));

    let today = Date::today_utc();
    let end = death_day(bday, years);

    let weeks_in_year = 52;

    let mut count = 0;
    let mut curr_date = bday;
    let outer_square_size: f64 = (d.height / weeks_in_year).min(d.width / years).into();
    let padding = outer_square_size as f64 * 0.2;

    while curr_date < end {
        let fill = if curr_date < today {
            color_secondary
        } else {
            color_primary
        };
        let x_offset = (f64::from(d.width) - (outer_square_size * f64::from(years))) / 2.0;
        let x = f64::from(count / weeks_in_year).mul_add(outer_square_size, x_offset);
        let y_offset = (f64::from(d.height) - (outer_square_size * f64::from(weeks_in_year))) / 2.0;
        let y = f64::from(count % weeks_in_year).mul_add(outer_square_size, y_offset);
        let square_side_size = outer_square_size as f64 - padding;
        let square = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", square_side_size)
            .set("height", square_side_size)
            .set("fill", fill)
            .set("stroke", color_secondary);
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

    document
}

fn main() {
    // Use these blocks of code for testing purposes.
    // let args = Args::parse_from(["death-calendar", "--help"]);
    // let args = Args::parse_from(["death-calendar", "info", "--help"]);
    // let args = Args::parse_from(["death-calendar", "info", "1998-07-26", "-l", "100"]);
    // let args = Args::parse_from([
    //     "death-calendar",
    //     "svg",
    //     "2000-01-01",
    //     "-l",
    //     "100",
    //     "-d",
    //     "2880x1800",
    //     "-o",
    //     "test.svg",
    // ]);

    let args = Args::parse();

    match args.command {
        Commands::Svg {
            dimensions,
            output,
            // This should work for now until https://github.com/clap-rs/clap/issues/1546 is resolved.
            common_args,
        } => {
            let document = render_svg(
                common_args.birthday,
                common_args.lifespan_years,
                &dimensions,
            );
            output.map_or_else(
                || println!("{}", document.to_string()),
                |file| svg::save(file, &document).expect("Couldn't save SVG to file."),
            );
        }
        Commands::Info { common_args } => {
            death_info(common_args.birthday, common_args.lifespan_years)
        }
    }
}
