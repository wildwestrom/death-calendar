use std::{error::Error as StdError, path::PathBuf};

mod calendar_image;
mod death_info;
use anyhow::Result;
use calendar_image::grid::{BorderUnit, SvgShape};
use clap::{value_parser, Parser};
use csscolorparser::{parse as parse_css_color, Color};
use directories::ProjectDirs;
use figment::{
	providers::{Format, Serialized, Toml},
	Figment,
};
use gregorian::Date;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

static QUALIFIER: &str = "xyz";
static ORGANIZATION: &str = "Westrom";
static APPLICATION: &str = "death-calendar";

#[derive(Debug)]
struct ProjectDirsNotFoundError;

impl StdError for ProjectDirsNotFoundError {
	fn description(&self) -> &str {
		"Could not find standard directories for storing application configuration and data."
	}
}

impl std::fmt::Display for ProjectDirsNotFoundError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "ProjectDirsNotFound")
	}
}

static PROJECT_DIRS: Lazy<Result<ProjectDirs, ProjectDirsNotFoundError>> = Lazy::new(|| {
	ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION).ok_or(ProjectDirsNotFoundError)
});

static CONFIG_FILE_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
	let mut file = PROJECT_DIRS.as_ref().ok()?.preference_dir().to_path_buf();
	file.push("config.toml");
	Some(file)
});

static BIRTHDAY_FILE_PATH: Lazy<Option<PathBuf>> = Lazy::new(|| {
	let mut file = PROJECT_DIRS.as_ref().ok()?.data_dir().to_path_buf();
	file.push("birthday");
	Some(file)
});

#[derive(Parser, Debug, Serialize, Deserialize)]
#[clap(author, version, about, long_about = {
	let conf_file_or_msg = CONFIG_FILE_PATH
		.as_ref()
		.map_or("Could not find directory for config file.".to_string(),
						|p| p.to_string_lossy().to_string());
	let bday_file_or_msg = BIRTHDAY_FILE_PATH
		.as_ref()
		.map_or("Could not find directory for birthday data file".to_string(),
						|p| p.to_string_lossy().to_string());
	format!("Calculate how much time you have until your ultimate demise.\n\nTo use the same options \
each time, you can put a config file in `{conf_file_or_msg}`. You can also put a file in \
`{bday_file_or_msg}` that contains a single string with your birthday in YYYY-MM-DD format to \
calculate your estimated time of death the same way each time.")
})]
struct Cli {
	#[clap(subcommand)]
	command: Commands,
	#[clap(flatten)]
	life_info: LifeInfo,
}

/// Information about a person's life.
#[derive(Parser, Debug, Serialize, Deserialize)]
pub struct LifeInfo {
	/// A birthday in `YYYY-MM-DD` format
	birthday: Date,
	/// Expected lifespan in years
	#[clap(short, long, default_value_t = 100)]
	lifespan_years: u16,
}

#[derive(Parser, Debug, Serialize, Deserialize)]
enum Commands {
	/// Print info about your ultimate demise
	Info,
	#[clap(id = "img")]
	/// Visualize your ultimate demise
	Image {
		#[clap(subcommand)]
		drawing_type: Drawing,
		#[clap(flatten)]
		drawing_info: DrawingInfo,
	},
}

/// Information about how to render an image.
#[serde_as]
#[derive(Parser, Debug, Serialize, Deserialize)]
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
	#[serde_as(as = "DisplayFromStr")]
	#[clap(long, value_parser(parse_css_color), default_value = "black")]
	color_primary: Color,
	/// Add a secondary color.
	#[serde_as(as = "Option<DisplayFromStr>")]
	#[clap(long, value_parser(parse_css_color))]
	color_secondary: Option<Color>,
	/// Save SVG to a file instead of printing to stdout
	#[clap(short, long)]
	output: Option<PathBuf>,
}

/// Information about how to render an image with no optional fields.
pub struct DrawingInfoValidated {
	scale_factor: u32,
	color_primary: Color,
	color_secondary: Color,
}

#[non_exhaustive]
#[derive(Parser, Debug, Serialize, Deserialize)]
pub enum Drawing {
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

/// Information about how to draw a grid calendar.
#[derive(Parser, Debug, Serialize, Deserialize)]
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

fn build_cli() -> Result<Cli> {
	let mut cli = Figment::new().merge(Serialized::defaults(Cli::parse()));
	if let Some(path) = CONFIG_FILE_PATH.as_ref() {
		cli = cli.merge(Toml::file(path.as_path()));
	}
	Ok(cli.extract()?)
}

fn main() -> Result<()> {
	let cli = build_cli()?;
	let life_info = cli.life_info;
	match cli.command {
		Commands::Info => death_info::show(life_info.birthday, life_info.lifespan_years),
		Commands::Image {
			drawing_type,
			drawing_info,
		} => calendar_image::draw_calendar(drawing_type, drawing_info, &life_info),
	}
}
