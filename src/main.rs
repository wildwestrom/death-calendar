pub mod svg_generator;
use svg_generator::{render_svg, BorderUnit, DrawingRatios, SvgShape};

use death_calendar::{
    days_left, days_lived, death_day, lifespan_days, lifespan_weeks, weeks_left, weeks_lived,
    years_left, years_lived,
};

use std::path::PathBuf;
use clap::Parser;
use gregorian::Date;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

// This should work for now until https://github.com/clap-rs/clap/issues/1546 is resolved.
#[derive(Parser, Debug)]
struct CommonArgs {
    /// A birthday in `YYYY-MM-DD` format
    birthday: Date,
    /// Expected lifespan in years
    #[clap(short, long, default_value_t = 100)]
    lifespan_years: i16,
}

// This won't work until https://github.com/clap-rs/clap/issues/1546 is fixed.
#[derive(Parser, Debug)]
enum Commands {
    /// Print info about your ultimate demise
    Info {
        #[clap(flatten)]
        common_args: CommonArgs,
    },
    /// Generate an SVG Image of the calendar
    Svg {
        #[clap(flatten)]
        common_args: CommonArgs,
        /// Save SVG to a file instead of printing to stdout
        #[clap(short, long)]
        output: Option<PathBuf>,
        /// How to measure space around calendar
        #[clap(short, long, value_enum, default_value_t = BorderUnit::Pixel)]
        border_unit: BorderUnit,
        /// Ratios used to create the image.
        /// Comma separated list of integers in order:
        /// [stroke_width,padding,shape_length,border_size]
        #[clap(
            short = 'r',
            long = "ratios",
            value_parser,
            default_value = "1,1,15,3",
            name = "RATIO_STRING"
        )]
        drawing_ratios: DrawingRatios,
        #[clap(short, long, value_enum, default_value_t = SvgShape::Square)]
        /// Shape used to represent a week
        shape: SvgShape,
    },
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
    //     "-o",
    //     "test.svg",
    // ]);

    let args = Args::parse();

    match args.command {
        Commands::Svg {
            output,
            border_unit,
            drawing_ratios,
            shape,
            // This should work for now until https://github.com/clap-rs/clap/issues/1546 is resolved.
            common_args,
        } => {
            let document = render_svg(
                common_args.birthday,
                common_args.lifespan_years,
                &drawing_ratios,
                &border_unit,
                &shape,
            );
            output.map_or_else(
                || println!("{}", document),
                |file| svg::save(file, &document).expect("Couldn't save SVG to file."),
            );
        }
        Commands::Info { common_args } => {
            death_info(common_args.birthday, common_args.lifespan_years);
        }
    }
}
