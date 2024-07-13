use std::{error::Error as StdError, path::PathBuf};

mod calendar_image;
mod death_info;
use anyhow::Result;
use calendar_image::grid::{BorderUnit, SvgShape};
use clap::{value_parser, Parser};
use csscolorparser::{parse as parse_css_color, Color};
use directories::ProjectDirs;
use gregorian::Date;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

const QUALIFIER: &str = "xyz";
const ORGANIZATION: &str = "Westrom";
const APPLICATION: &str = "death-calendar";
static APP_NAME: Lazy<String> = Lazy::new(|| format!("{QUALIFIER}.{ORGANIZATION}.{APPLICATION}"));

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
		.map_or("Could not find path to config file.".to_string(),
						|p| p.to_string_lossy().to_string());
	let bday_file_or_msg = BIRTHDAY_FILE_PATH
		.as_ref()
		.map_or("Could not find path to birthday data file".to_string(),
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
	/// Unknown arguments or everything after -- gets passed through to GTK.
	#[arg(allow_hyphen_values = true, trailing_var_arg = true)]
	pub gtk_options: Vec<String>,
}

/// Information about a person's life.
#[derive(Parser, Debug, Clone, Serialize, Deserialize)]
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
	/// Visualize your ultimate demise
	Gui,
}

#[non_exhaustive]
#[derive(Parser, Debug, Serialize, Deserialize)]
struct Gui;

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

impl Default for DrawingInfo {
	fn default() -> Self {
		Self {
			scale_factor: 1,
			color_primary: Color::default(),
			color_secondary: Some(Color::from_rgba8(255, 255, 255, 255)),
			output: None,
		}
	}
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

impl Default for GridRatios {
	fn default() -> Self {
		Self {
			stroke: 1,
			padding: 1,
			length: 15,
			border: 3,
			border_unit: BorderUnit::Pixel,
		}
	}
}

// use gtk::gio::prelude::*;
// use gtk::glib::prelude::*;
use adw::prelude::*;
use gtk::gdk_pixbuf;
// use gtk::gdk_pixbuf::prelude::*;
// use gtk::prelude::*;
use relm4::{
	gtk::{gdk, glib},
	prelude::*,
};

type ErrMsg = String;

#[derive(Debug)]
enum MaybeTexture {
	Texture(gdk::Texture),
	None(ErrMsg),
}

#[derive(Debug)]
enum AppMsg {
	UpdateBirthday(Date),
	UpdateLifespan(u16),
}

const MARGIN: i32 = 5;
const DEFAULT_HEIGHT: i32 = 480;
const DEFAULT_WIDTH: i32 = 360;

#[derive(Debug)]
struct CalendarPic {
	image_texture: MaybeTexture,
}

impl Default for CalendarPic {
	fn default() -> Self {
		Self {
			image_texture: MaybeTexture::None("No image loaded yet".into()),
		}
	}
}

impl CalendarPic {
	fn new_texture(life_info: &LifeInfo, drawing_type: Drawing) -> MaybeTexture {
		let res: Result<gdk::Texture> = (|| {
			let svg =
				calendar_image::draw_calendar(drawing_type, DrawingInfo::default(), life_info)?;
			let loader = gdk_pixbuf::PixbufLoader::with_type("svg")?;
			let svg_bytes = glib::Bytes::from(svg.to_string().as_bytes());
			loader.write_bytes(&svg_bytes)?;
			loader.close()?;
			let texture = gdk::Texture::from_bytes(&svg_bytes)?;
			Ok(texture)
		})();
		match res {
			Ok(texture) => MaybeTexture::Texture(texture),
			Err(e) => MaybeTexture::None(format!("Could not create an image:\n{e}")),
		}
	}
}

struct NewCalendarPic(LifeInfo, Drawing);

#[relm4::component]
impl SimpleComponent for CalendarPic {
	type Input = LifeInfo;
	type Output = ();
	type Init = NewCalendarPic;

	view! {
		#[root]
		gtk::Box {
			set_margin_all: MARGIN,
			set_orientation: gtk::Orientation::Vertical,
			gtk::Label {
				#[watch]
				set_label: match model.image_texture {
					MaybeTexture::Texture(_) => "",
					MaybeTexture::None(ref e) => e,
				}
			},
			append =
				&gtk::Picture::builder()
					.height_request(match model.image_texture {
						MaybeTexture::Texture(ref t) => t.height().min(50),
						MaybeTexture::None(_) => 0,
					})
					.width_request(match model.image_texture {
						MaybeTexture::Texture(ref t) => t.width().min(DEFAULT_WIDTH),
						MaybeTexture::None(_) => 0,
					})
					.margin_start(MARGIN)
					.margin_end(MARGIN)
					.margin_top(MARGIN)
					.margin_bottom(MARGIN)
					.overflow(gtk::Overflow::Visible)
					.content_fit(gtk::ContentFit::Contain)
					.build() {
						#[watch]
						set_paintable:
							match model.image_texture {
									MaybeTexture::Texture(ref t) => Some(t),
									MaybeTexture::None(ref _s) => None,
							},
			},
			gtk::Button {
				set_label: "Export Image:",
			}
		}
	}

	fn init(
		init: Self::Init,
		root: Self::Root,
		_sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let model = Self {
			image_texture: Self::new_texture(&init.0, init.1),
		};
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}
}

#[derive(Debug)]
struct App {
	life_info: LifeInfo,
	calendar_pic_grid: Controller<CalendarPic>,
	calendar_pic_log: Controller<CalendarPic>,
}

#[relm4::component]
impl SimpleComponent for App {
	type Init = LifeInfo;
	type Input = AppMsg;
	type Output = ();

	view! {
		adw::ApplicationWindow {
			set_title: Some("Death Calendar"),
			set_default_size: (DEFAULT_WIDTH, DEFAULT_HEIGHT),
			set_size_request: (DEFAULT_WIDTH, DEFAULT_HEIGHT),
			set_decorated: false,
			adw::ToolbarView {
				set_top_bar_style: adw::ToolbarStyle::Raised,
				add_top_bar = &adw::HeaderBar {
					#[wrap(Some)]
					set_title_widget = &adw::WindowTitle::new("Death Calendar","Visualize your demise"),
					set_show_title: true,
				},
				#[wrap(Some)]
				set_content = &gtk::ScrolledWindow {
					adw::Clamp {
						set_maximum_size: 1080,
						set_tightening_threshold: 480,
						gtk::Box {
							set_orientation: gtk::Orientation::Vertical,
							set_margin_all: MARGIN,
							set_halign: gtk::Align::Start,
							set_hexpand: false,
							set_vexpand: false,
							gtk::Calendar::builder()
								.day(model.life_info.birthday.day().into())
								.month(model.life_info.birthday.month().to_number().into())
								.year(model.life_info.birthday.year().to_number().into())
								.margin_top(MARGIN)
								.margin_bottom(MARGIN)
								.margin_start(MARGIN)
								.margin_end(MARGIN)
								.hexpand(true)
								.build() {
								connect_day_selected[sender] => move |calendar| {
									if let Ok(new_calendar) = Date::new(
										calendar.year() as i16,
										(calendar.month() + 1) as u8, // GTK gives a number from 0-11.
										calendar.day() as u8,
									) {
										sender
											.input(AppMsg::UpdateBirthday(new_calendar));
									}
								}
							},
							gtk::Label {
								set_halign: gtk::Align::Start,
								set_margin_all: MARGIN,
								#[watch]
								set_text: &format!("Your birthday is {}.", &model.life_info.birthday),
							},
							gtk::Separator {
								set_margin_all: MARGIN,
							},
							gtk::Box {
								set_orientation: gtk::Orientation::Horizontal,
								set_halign: gtk::Align::Start,
								set_margin_all: MARGIN,
								adw::EntryRow {
									#[watch]
									set_title: "Number of years expected to live:",
									set_text: &model.life_info.lifespan_years.to_string(),
									// set_placeholder_text: Some("Nubmer of years"),
									connect_changed[sender] => move |entry| {
										if let Ok(years) = entry.text().parse::<u16>() {
											sender.input(AppMsg::UpdateLifespan(years))
										}
									}
								}
							},
							gtk::Label {
								set_halign: gtk::Align::Start,
								set_margin_all: MARGIN,
								#[watch]
								set_text: &match death_info::info(model.life_info.birthday, model.life_info.lifespan_years) {
										Ok(info) => info,
										Err(e) => format!("Unable to show info: {e}"),
									}
							},
							model.calendar_pic_grid.widget(),
							model.calendar_pic_log.widget(),
						}
					}
				},
			},
		}
	}

	// Initialize the component.
	fn init(
		life_info: Self::Init,
		root: Self::Root,
		sender: ComponentSender<Self>,
	) -> ComponentParts<Self> {
		let calendar_pic_grid = CalendarPic::builder()
			.launch(NewCalendarPic(
				life_info.clone(),
				Drawing::Grid {
					grid_ratios: GridRatios::default(),
					week_shape: SvgShape::Square,
				},
			))
			.forward(sender.input_sender(), |msg| match msg {
				_ => todo!(),
			});
		let calendar_pic_log = CalendarPic::builder()
			.launch(NewCalendarPic(
				life_info.clone(),
				Drawing::Logarithmic {
					width_height_ratio: 8.0,
				},
			))
			.forward(sender.input_sender(), |msg| match msg {
				_ => todo!(),
			});

		let model = Self {
			life_info: life_info.clone(),
			calendar_pic_grid,
			calendar_pic_log,
		};
		let widgets = view_output!();
		ComponentParts { model, widgets }
	}

	fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
		match msg {
			Self::Input::UpdateBirthday(new_birthday) => {
				self.life_info.birthday = new_birthday;
			},
			Self::Input::UpdateLifespan(years) => {
				self.life_info.lifespan_years = years;
			},
		}
	}
}

fn main() {
	let app = RelmApp::new(&APP_NAME);
	app.run::<App>(LifeInfo {
		birthday: Date::new(2000, 1, 1).unwrap(),
		lifespan_years: 100,
	});
}
