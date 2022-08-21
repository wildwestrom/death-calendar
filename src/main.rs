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
    /// A birthday in `YYYY-MM-DD` format
    birthday: Date,
    /// Expected lifespan in years
    #[clap(short, long, default_value_t = 100)]
    lifespan_years: i16,
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum BorderUnit {
    Pixel,
    Shape,
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
    },
}

#[derive(Debug)]
struct ParseDrawingRatiosError;

impl std::fmt::Display for ParseDrawingRatiosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not parse the ratio string")
    }
}

impl std::error::Error for ParseDrawingRatiosError {}

impl std::str::FromStr for DrawingRatios {
    type Err = ParseDrawingRatiosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_str: Vec<&str> = s.split(',').collect();
        let ratio_vals: Vec<i16> = split_str
            .iter()
            .filter_map(|n| i16::from_str(n).ok())
            .collect();
        if ratio_vals.len() > 4 {
            return Err(ParseDrawingRatiosError);
        }
        let stroke = ratio_vals[0];
        let padding = ratio_vals[1];
        let length = ratio_vals[2];
        let border = ratio_vals[3];
        Ok(Self {
            stroke,
            padding,
            length,
            border,
        })
    }
}

#[derive(Debug, Clone)]
struct DrawingRatios {
    // How thick should the line around the shape be?
    stroke: i16,
    // How much space should be around each shape?
    padding: i16,
    // How long should the shape be on the inside?
    length: i16,
    // How much space should be around the grid?
    border: i16,
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
fn render_svg(
    bday: Date,
    years: i16,
    drawing_ratios: &DrawingRatios,
    border_unit: &BorderUnit,
) -> Document {
    let color_primary = "black";
    let color_secondary = "white";

    let today = Date::today_utc();
    let end = death_day(bday, years);

    // Adding a scale factor seems to make the image render more crisply.
    let scale_factor = 2; // Ensure this scale factor is greater than 0.
    let stroke_width = drawing_ratios.stroke * scale_factor * 2;

    let padding = drawing_ratios.padding * scale_factor;
    let inner_shape_size = (drawing_ratios.length * 2) * scale_factor + stroke_width;
    let outer_shape_size = inner_shape_size + (padding * 2) + stroke_width;

    let border = match border_unit {
        BorderUnit::Pixel => drawing_ratios.border,
        BorderUnit::Shape => drawing_ratios.border * outer_shape_size,
    };

    // In total, the outer dimensions of a shape is a function of its stroke-width x 2,
    // hence the variable `space_around_shape`.
    let grid_width = (outer_shape_size * years) + stroke_width;
    let grid_height = (outer_shape_size * WEEKS_IN_A_YEAR) + stroke_width;

    let viewbox_width = grid_width + (border * 2) + (padding * 2) + (stroke_width / 2);
    let viewbox_height = grid_height + (border * 2) + (padding * 2) + (stroke_width / 2);

    let mut document = Document::new()
        .set("viewBox", (0, 0, viewbox_width, viewbox_height))
        .set("style", format!("background-color:{}", color_primary));

    dbg!(stroke_width);
    dbg!(viewbox_width - grid_width);
    dbg!(viewbox_height - grid_height);

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
        let x_offset = ((viewbox_width - grid_width) / 2) + padding + stroke_width;
        let x = ((count / WEEKS_IN_A_YEAR) * outer_shape_size) + x_offset;
        let y_offset = ((viewbox_height - grid_height) / 2) + padding + stroke_width;
        let y = ((count % WEEKS_IN_A_YEAR) * outer_shape_size) + y_offset;
        let shape = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", inner_shape_size)
            .set("height", inner_shape_size)
            .set("fill", fill)
            .set("stroke", color_primary)
            .set("stroke-width", stroke_width);

        document.append(shape);
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
            border_unit,
            drawing_ratios,
            // This should work for now until https://github.com/clap-rs/clap/issues/1546 is resolved.
            common_args,
        } => {
            dbg!(&drawing_ratios);
            let document = render_svg(
                common_args.birthday,
                common_args.lifespan_years,
                &drawing_ratios,
                &border_unit,
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
