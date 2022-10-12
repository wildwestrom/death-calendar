#![allow(clippy::default_numeric_fallback)] // This is due to a false positive
use svg::{node::element::Rectangle, Document, Node};

pub mod grid;
pub mod logarithmic;

pub const AVERAGE_DAYS_IN_YEAR: f64 = 365.2425;
pub const PHI: f64 = 1.618_033_988_749_895;
pub const WEEKS_IN_A_YEAR: u32 = 52;

pub fn init_document(
	viewbox_width: f64,
	viewbox_height: f64,
	color_primary: &str,
	color_secondary: &str,
) -> Document {
	let mut document = Document::new()
		.set("viewBox", (0_u8, 0_u8, viewbox_width, viewbox_height))
		.set("style", format!("background-color:{color_primary}"));

	let background = Rectangle::new()
		.set("x", 0_u8)
		.set("y", 0_u8)
		.set("width", viewbox_width)
		.set("height", viewbox_height)
		.set("fill", color_secondary);
	document.append(background);

	document
}
