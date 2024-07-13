#![allow(clippy::default_numeric_fallback)]
use csscolorparser::Color;
// This is due to a false positive
use anyhow::Result;
use svg::{
	node::element::{Rectangle, SVG},
	Document, Node,
};

use crate::{Drawing, DrawingInfo, DrawingInfoValidated, LifeInfo};

pub mod grid;
pub mod logarithmic;

pub const AVERAGE_DAYS_IN_YEAR: f64 = 365.2425;
pub const PHI: f64 = 1.618_033_988_749_895;
pub const WEEKS_IN_A_YEAR: u32 = 52;

pub fn init_document(viewbox_width: f64, viewbox_height: f64, color_secondary: &str) -> Document {
	let mut document = Document::new()
		.set("viewBox", (0_u8, 0_u8, viewbox_width, viewbox_height))
		.set("style", format!("fill:{color_secondary}"));

	let background = Rectangle::new()
		.set("x", 0_u8)
		.set("y", 0_u8)
		.set("width", viewbox_width)
		.set("height", viewbox_height)
		.set("fill", color_secondary);
	document.append(background);

	document
}

fn linear_invert_color(c: &Color) -> Color {
	Color::new(1.0 - c.r, 1.0 - c.g, 1.0 - c.b, c.a)
}

pub fn draw_calendar(
	drawing_type: Drawing,
	drawing_info: DrawingInfo,
	life_info: &LifeInfo,
) -> Result<SVG> {
	let drawing_info_validated = DrawingInfoValidated {
		scale_factor: drawing_info.scale_factor,
		color_primary: drawing_info.color_primary.clone(),
		color_secondary: {
			if let Some(color) = drawing_info.color_secondary {
				color
			} else {
				// TODO: Use Oklab
				// ex `oklab(from color.primary l a b / 0.5)`
				linear_invert_color(&drawing_info.color_primary)
			}
		},
	};

	let document: Document = match drawing_type {
		Drawing::Grid {
			grid_ratios,
			week_shape,
		} => grid::render_svg(
			life_info,
			&drawing_info_validated,
			&grid_ratios,
			&week_shape,
		)?,
		Drawing::Logarithmic { width_height_ratio } => {
			logarithmic::render_svg(life_info, &drawing_info_validated, width_height_ratio)?
		},
	};

	Ok(document)
}
