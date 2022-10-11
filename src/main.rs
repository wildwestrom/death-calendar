use std::path::PathBuf;

use clap::{value_parser, Parser};
use death_calendar::{
	days_left, days_lived, death_day, lifespan_days, lifespan_months, lifespan_weeks, months_left,
	months_lived, weeks_left, weeks_lived, years_left, years_lived,
};
use gregorian::Date;
use hex_color::HexColor;
mod grid_calendar;
use grid_calendar::{SvgShape, BorderUnit};
mod logarithmic_calendar;
mod parse_color;
use parse_color::parse_svg_color;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(subcommand)]
	command: Commands,
}

#[derive(Parser, Debug)]
struct CommonArgs {
	/// A birthday in `YYYY-MM-DD` format
	birthday: Date,
	/// Expected lifespan in years
	#[clap(short, long, default_value_t = 100)]
	lifespan_years: i16,
}

#[derive(Parser, Debug, Clone)]
pub struct DrawingRatios {
	#[clap(long, default_value_t = 1)]
	/// How thick should the line around each shape be?
	stroke: u32,
	#[clap(long, default_value_t = 1)]
	/// How much space should be around each shape?
	padding: u32,
	#[clap(long, default_value_t = 15)]
	/// How long should the shape be on the inside?
	length: u32,
	#[clap(long, default_value_t = 3)]
	/// How much space should be around the grid?
	border: u32,
	#[clap(long, default_value_t = BorderUnit::Pixel)]
	/// Should the border be measured in pixels or the shape?
	border_unit: BorderUnit,
}

#[derive(Parser, Debug, Clone)]
struct CommonDrawingArgs {
	/// Optionally increase the scale of the svg.
	/// This can help improve scaling quality on some image viewers.
	/// Must be a number greater than 0.
	#[clap(long, value_parser(value_parser!(u32).range(1..)), default_value_t = 1)]
	scale_factor: u32,
	/// Add a primary color.
	/// You can use a string containing (almost) any valid <color> type from the SVG 1.1
	/// specification. https://www.w3.org/Graphics/SVG/1.1/types.html#DataTypeColor
	#[clap(long, value_parser = clap::builder::ValueParser::new(parse_svg_color), default_value = "black")]
	color_primary: HexColor,
	/// Add a secondary color.
	#[clap(long, value_parser = clap::builder::ValueParser::new(parse_svg_color))]
	color_secondary: Option<HexColor>,
}

#[derive(Parser, Debug)]
enum Commands {
	/// Print info about your ultimate demise
	Info {
		#[clap(flatten)]
		common_args: CommonArgs,
	},
	/// Generate an image of a grid-style calendar
	Grid {
		#[clap(flatten)]
		common_args: CommonArgs,
		#[clap(flatten)]
		common_drawing_args: CommonDrawingArgs,
		#[clap(flatten)]
		drawing_ratios: DrawingRatios,
		#[clap(long, value_enum, default_value_t = SvgShape::Square)]
		/// Shape used to represent a week
		shape: SvgShape,
		#[clap(short, long)]
		/// Save SVG to a file instead of printing to stdout
		output: Option<PathBuf>,
	},
	/// Generate an image of a logarithmic calendar
	Log {
		#[clap(flatten)]
		common_args: CommonArgs,
		/// Save SVG to a file instead of printing to stdout
		#[clap(flatten)]
		common_drawing_args: CommonDrawingArgs,
		#[clap(short, long)]
		output: Option<PathBuf>,
	},
}

#[allow(clippy::print_stdout)]
fn death_info(bday: Date, years: i16) {
	let today: Date = Date::today_utc();
	println!("Your birthday is {}.", bday);
	println!();
	println!("You will live for approximately:");
	println!("- {} days", lifespan_days(bday, years));
	println!("- {} weeks", lifespan_weeks(years));
	println!("- {} months", lifespan_months(years));
	println!("- {} years", years);
	println!();
	println!("You will probably die around {}.", death_day(bday, years));
	println!("You have lived for:");
	println!("- {} days", days_lived(today, bday));
	println!("- {} weeks", weeks_lived(today, bday));
	println!("- {} months", months_lived(today, bday));
	println!("- {} years", years_lived(today, bday));
	println!();
	println!("You have remaining:");
	println!("- {} days", days_left(today, bday, years).abs());
	println!("- {} weeks", weeks_left(today, bday, years).abs());
	println!("- {} months", months_left(today, bday, years).abs());
	println!("- {} years", years_left(today, bday, years).abs());
}

fn main() -> Result<(), std::io::Error> {
	let args = Args::parse();

	match args.command {
		Commands::Grid {
			output,
			drawing_ratios,
			shape,
			common_drawing_args,
			// This should work for now until https://github.com/clap-rs/clap/issues/1546 is
			// resolved.
			common_args,
		} => {
			let document = grid_calendar::render_svg(
				common_args.birthday,
				common_args.lifespan_years,
				&drawing_ratios,
				&shape,
				common_drawing_args.scale_factor,
				common_drawing_args.color_primary,
				common_drawing_args.color_secondary,
			);
			output.map_or_else(
				#[allow(clippy::print_stdout)]
				|| {
					println!("{document}");
					Ok(())
				},
				|file| {
					svg::save(file, &document)?;
					Ok(())
				},
			)
		},
		Commands::Log {
			common_args,
			output,
			common_drawing_args,
		} => {
			let document = logarithmic_calendar::render_svg(
				common_args.birthday,
				common_args.lifespan_years,
				common_drawing_args.scale_factor,
				common_drawing_args.color_primary,
				common_drawing_args.color_secondary,
			);
			output.map_or_else(
				#[allow(clippy::print_stdout)]
				|| {
					println!("{document}");
					Ok(())
				},
				|file| {
					svg::save(file, &document)?;
					Ok(())
				},
			)
		},
		Commands::Info { common_args } => {
			death_info(common_args.birthday, common_args.lifespan_years);
			Ok(())
		},
	}
}
