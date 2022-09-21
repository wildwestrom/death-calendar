use std::path::PathBuf;

use clap::{value_parser, Parser};
use death_calendar::{
	days_left, days_lived, death_day, lifespan_days, lifespan_months, lifespan_weeks, months_left,
	months_lived, weeks_left, weeks_lived, years_left, years_lived,
};
use gregorian::Date;
use hex_color::HexColor;
mod svg_generator;
use svg_generator::{render_svg, DrawingRatios, SvgShape};
mod parse_color;
use parse_color::parse_svg_color;

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

#[derive(Parser, Debug)]
enum Commands {
	/// Print info about your ultimate demise
	Info {
		#[clap(flatten)]
		common_args: CommonArgs,
	},
	/// Generate an SVG image of the calendar
	Svg {
		#[clap(flatten)]
		common_args: CommonArgs,
		/// Save SVG to a file instead of printing to stdout
		#[clap(short, long)]
		output: Option<PathBuf>,
		/// Ratios used to create the image. Comma separated list of values in order. The first
		/// four values are integers. The last value represents the units used to measure
		/// the border size, which is either in 'pixel's or the size of the 'shape' itself.
		#[clap(
			short = 'r',
			long = "ratios",
			value_parser,
			default_value = "1,1,15,3,pixel",
			name = "RATIO_STRING"
		)]
		drawing_ratios: DrawingRatios,
		#[clap(long, value_enum, default_value_t = SvgShape::Square)]
		/// Shape used to represent a week
		shape: SvgShape,
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
		Commands::Svg {
			output,
			drawing_ratios,
			shape,
			scale_factor,
			color_primary,
			color_secondary,
			// This should work for now until https://github.com/clap-rs/clap/issues/1546 is
			// resolved.
			common_args,
		} => {
			let document = render_svg(
				common_args.birthday,
				common_args.lifespan_years,
				&drawing_ratios,
				&shape,
				scale_factor,
				color_primary,
				color_secondary,
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
