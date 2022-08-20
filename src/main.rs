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

#[derive(Debug, Clone, clap::ValueEnum)]
enum BorderUnit {
    Square,
    Pixel,
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
        /// Save SVG to a file instead of printing to stdout.
        #[clap(short, long)]
        output: Option<PathBuf>,
        /// Units used to measure the border around the calendar.
        #[clap(short, long, value_enum, default_value_t = BorderUnit::Pixel)]
        border_units: BorderUnit,
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

struct Scale {
    // How thick should the line around the square be?
    stroke: i16,
    // How much space should be around each square?
    padding: i16,
    // How big should the square be on the inside?
    square: i16,
    // How much space should be around the grid?
    border: i16,
}

const WEEKS_IN_A_YEAR: i16 = 52;
fn render_svg(bday: Date, years: i16, border_unit: BorderUnit) -> Document {
    let color_primary = "black";
    let color_secondary = "white";

    let today = Date::today_utc();
    let end = death_day(bday, years);

    // Adding a scale factor seems to make the image render more crisply.
    let scale_factor = 1; // Ensure this scale factor is greater than 0.
    let drawing_ratios = Scale {
        stroke: 1,
        padding: 1,
        square: 15,
        border: 3,
    };
    let stroke_width = drawing_ratios.stroke * 2;

    let padding = drawing_ratios.padding * scale_factor;
    let inner_square_size = drawing_ratios.square * scale_factor;
    let outer_square_size = inner_square_size + (padding * 2) + stroke_width;

    let border;
    match border_unit {
        BorderUnit::Pixel => {
            border = drawing_ratios.border;
        }
        BorderUnit::Square => {
            border = drawing_ratios.border * outer_square_size;
        }
    }

    let padding_x2 = padding * 2;
    let grid_width = outer_square_size * years + padding_x2;
    let grid_height = outer_square_size * WEEKS_IN_A_YEAR + padding_x2;

    let viewbox_width = grid_width + (border * 2);
    let viewbox_height = grid_height + (border * 2);

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

    let mut count = 0;
    let mut curr_date = bday;
    while curr_date < end {
        let fill = if curr_date < today {
            color_primary
        } else {
            color_secondary
        };
        let p2 = (padding * 2) + drawing_ratios.stroke;
        let x_offset = ((viewbox_width - grid_width) / 2) + p2;
        let x = ((count / WEEKS_IN_A_YEAR) * outer_square_size) + x_offset;
        let y_offset = ((viewbox_height - grid_height) / 2) + p2;
        let y = ((count % WEEKS_IN_A_YEAR) * outer_square_size) + y_offset;
        let square = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", inner_square_size)
            .set("height", inner_square_size)
            .set("fill", fill)
            .set("stroke", color_primary)
            .set("stroke-width", stroke_width);

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
            border_units,
            // This should work for now until https://github.com/clap-rs/clap/issues/1546 is resolved.
            common_args,
        } => {
            let document = render_svg(
                common_args.birthday,
                common_args.lifespan_years,
                border_units,
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
