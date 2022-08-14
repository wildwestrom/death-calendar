use std::path::PathBuf;

use death_calendar::{
    days_left, days_lived, death_day, lifespan_days, lifespan_weeks, weeks_left, weeks_lived,
    years_left, years_lived,
};

use clap::Parser;
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

// This won't work until https://github.com/clap-rs/clap/issues/1546 is fixed.
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
        #[clap(short, long)]
        output: Option<PathBuf>,
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

const WEEKS_IN_A_YEAR: i16 = 52;
fn render_svg(bday: Date, years: i16) -> Document {
    let color_primary = "black";
    let color_secondary = "white";

    let today = Date::today_utc();
    let end = death_day(bday, years);

    let mut count = 0;
    let mut curr_date = bday;

    // Adding a scale factor seems to make the image render more crisply.
    let scale_factor = 4;
    let inner_square_size = 12 * scale_factor;
    let padding = 1 * scale_factor;
    let outer_square_size = inner_square_size + (padding * 3);

    let border_size = 2;
    let grid_width = outer_square_size * years;
    let grid_height = outer_square_size * WEEKS_IN_A_YEAR;
    let viewbox_width = years * outer_square_size + outer_square_size * border_size;
    let viewbox_height = WEEKS_IN_A_YEAR * outer_square_size + outer_square_size * border_size;

    let mut document = Document::new()
        .set("viewBox", (0, 0, viewbox_width, viewbox_height))
        .set("style", format!("background-color:{}", color_primary));

    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", viewbox_width)
        .set("height", viewbox_height)
        .set("fill", color_secondary);

    document.append(background);

    while curr_date < end {
        let fill = if curr_date < today {
            color_primary
        } else {
            color_secondary
        };
        let x_offset = (viewbox_width - grid_width) / 2;
        let x = ((count / WEEKS_IN_A_YEAR) * outer_square_size) + x_offset;
        let y_offset = (viewbox_height - grid_height) / 2;
        let y = ((count % WEEKS_IN_A_YEAR) * outer_square_size) + y_offset;
        let square = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", inner_square_size)
            .set("height", inner_square_size)
            .set("fill", fill)
            .set("stroke", color_primary)
            .set("stroke-width", padding);

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
    //     "-o",
    //     "test.svg",
    // ]);

    let args = Args::parse();

    match args.command {
        Commands::Svg {
            output,
            // This should work for now until https://github.com/clap-rs/clap/issues/1546 is resolved.
            common_args,
        } => {
            let document = render_svg(common_args.birthday, common_args.lifespan_years);
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
