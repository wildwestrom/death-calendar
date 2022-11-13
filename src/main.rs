use std::path::PathBuf;

use anyhow::Result;
use clap::{value_parser, Parser};
mod calendar_image;
mod death_info;
use calendar_image::{
	grid::{self, BorderUnit, SvgShape},
	logarithmic,
};
use csscolorparser::{parse as parse_css_color, Color};
use svg::Document;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(subcommand)]
	command: Commands,
}

#[derive(Parser, Debug)]
pub struct BirthInfo {
	/// A birthday in `YYYY-MM-DD` format
	birthday: gregorian::Date,
	/// Expected lifespan in years
	#[clap(short, long, default_value_t = 100)]
	lifespan_years: u16,
}

#[derive(Parser, Debug)]
pub struct DrawingInfo {
	/// Optionally increase the scale of the SVG.
	///
	/// This can help improve scaling quality on some image viewers.
	/// Must be a number greater than 0.
	#[clap(long, value_parser(value_parser!(u32).range(1..)), default_value_t = 1)]
	scale_factor: u32,
	/// Add a primary color.
	///
	/// You can use a string containing any valid CSS3 color.
	/// Uses [csscolorparser](https://crates.io/crates/csscolorparser).
	#[clap(long, value_parser(parse_css_color), default_value = "black")]
	color_primary: Color,
	/// Add a secondary color.
	#[clap(long, value_parser(parse_css_color))]
	color_secondary: Option<Color>,
	/// Save SVG to a file instead of printing to stdout
	#[clap(short, long)]
	output: Option<PathBuf>,
}

pub struct DrawingInfoValidated {
	scale_factor: u32,
	color_primary: Color,
	color_secondary: Color,
}

#[derive(Parser, Debug)]
enum Drawing {
	/// Generate an image of a grid-style calendar
	Grid {
		#[clap(flatten)]
		grid_ratios: GridRatios,
		#[clap(long, value_enum, default_value_t = SvgShape::Square)]
		/// Shape used to represent a week
		week_shape: SvgShape,
	},
	#[clap(id = "log")]
	/// Generate an image of a logarithmic calendar
	Logarithmic {
		#[clap(long, default_value_t = 8.0)]
		width_height_ratio: f64,
	},
}

#[derive(Parser, Debug)]
enum Commands {
	/// Print info about your ultimate demise
	Info {
		#[clap(flatten)]
		birth_info: BirthInfo,
	},
	#[clap(id = "img")]
	/// Visualize your ultimate demise
	Image {
		#[clap(subcommand)]
		drawing_type: Drawing,
		#[clap(flatten)]
		birth_info: BirthInfo,
		#[clap(flatten)]
		drawing_info: DrawingInfo,
	},
}

#[derive(Parser, Debug)]
pub struct GridRatios {
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

fn main() -> Result<()> {
	let args = Args::parse();

	match args.command {
		Commands::Info {
			birth_info: common_args,
		} => death_info::show(common_args.birthday, common_args.lifespan_years),
		Commands::Image {
			drawing_type,
			drawing_info,
			birth_info,
		} => {
			let color = drawing_info.color_primary;
			let drawing_info_validated = DrawingInfoValidated {
				scale_factor: drawing_info.scale_factor,
				color_primary: color.clone(),
				color_secondary: {
					if let Some(color) = drawing_info.color_secondary {
						color
					} else {
						Color::new(1.0 - color.r, 1.0 - color.g, 1.0 - color.b, color.a)
					}
				},
			};

			let document: Document = match drawing_type {
				Drawing::Grid {
					grid_ratios,
					week_shape,
				} => grid::render_svg(
					&birth_info,
					&drawing_info_validated,
					&grid_ratios,
					&week_shape,
				)?,
				Drawing::Logarithmic { width_height_ratio } => logarithmic::render_svg(
					&birth_info,
					&drawing_info_validated,
					width_height_ratio,
				)?,
			};

			#[allow(clippy::print_stdout)]
			if let Some(filename) = drawing_info.output {
				svg::save(filename, &document)?;
			} else {
				println!("{document}");
			}
			Ok(())
		},
	}
}
