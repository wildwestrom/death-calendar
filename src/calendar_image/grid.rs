use std::str::FromStr;

use anyhow::Result;
use death_calendar::death_day;
use gregorian::Date;
use serde::{Deserialize, Serialize};
use svg::{
	node::element::{Circle, Element, Rectangle},
	Document, Node,
};

use super::{init_document, WEEKS_IN_A_YEAR};
use crate::{DrawingInfoValidated, GridRatios, LifeInfo};

#[derive(Debug, Clone, clap::ValueEnum, Serialize, Deserialize)]
pub enum BorderUnit {
	Pixel,
	Shape,
}

impl std::fmt::Display for BorderUnit {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match *self {
				Self::Pixel => "pixel",
				Self::Shape => "shape",
			}
		)
	}
}

#[derive(Debug)]
pub struct ParseBorderUnitError;

impl std::fmt::Display for ParseBorderUnitError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"Could not parse string as a border unit variant\nDid you misspell it?"
		)
	}
}

impl FromStr for BorderUnit {
	type Err = ParseBorderUnitError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
			"0" | "pixel" | "pixels" => Ok(Self::Pixel),
			"1" | "shape" | "shapes" => Ok(Self::Shape),
			_ => Err(ParseBorderUnitError),
		}
	}
}

#[derive(Debug, Clone, clap::ValueEnum, Serialize, Deserialize)]
pub enum SvgShape {
	Square,
	Circle,
}

pub fn render_svg(
	birth_info: &LifeInfo,
	drawing_info: &DrawingInfoValidated,
	drawing_ratios: &GridRatios,
	week_shape: &SvgShape,
) -> Result<Document> {
	let color_primary = drawing_info.color_primary.to_hex_string();
	let color_secondary = drawing_info.color_secondary.to_hex_string();
	let scale_factor = drawing_info.scale_factor;

	let bday = birth_info.birthday;
	let lifespan_years = birth_info.lifespan_years;

	let today = Date::today_utc();
	let end = death_day(bday, lifespan_years.try_into()?);

	let stroke_width = drawing_ratios.stroke * scale_factor * 2;

	let padding = drawing_ratios.padding * scale_factor;
	let inner_shape_size = (drawing_ratios.length * 2) * scale_factor + stroke_width;
	let outer_shape_size = inner_shape_size + (padding * 2) + stroke_width;

	let border = match drawing_ratios.border_unit {
		BorderUnit::Pixel => drawing_ratios.border * scale_factor,
		BorderUnit::Shape => drawing_ratios.border * outer_shape_size,
	};

	// In total, the outer dimensions of a shape is a function of its stroke-width x 2,
	// hence the variable `space_around_shape`.
	let grid_width = outer_shape_size
        // Here I convert an i16 to a u32.
        // I'm not sure what behavior I'll get if there's a negative value going in,
        // so for now I'll just set it to zero just in case.
        * u32::try_from(lifespan_years).unwrap_or(0);
	let grid_height = outer_shape_size * WEEKS_IN_A_YEAR;

	let viewbox_width = grid_width + (border * 2) + (padding * 2);
	let viewbox_height = grid_height + (border * 2) + (padding * 2);

	let mut document = init_document(
		viewbox_width.into(),
		viewbox_height.into(),
		&color_secondary,
	);

	let mut count = 0;
	let mut curr_date = bday;
	while curr_date < end {
		// There's an off-by-one error if we do not add 7 days to the current date. It will show
		// that one week has passed since the person's birthday on their birthday, which is not
		// correct.
		let fill = if curr_date.add_days(7) <= today {
			color_primary.as_str()
		} else {
			color_secondary.as_str()
		};

		let x_offset = ((viewbox_width - grid_width) / 2) + padding + (stroke_width / 2);
		let x = ((count / WEEKS_IN_A_YEAR) * outer_shape_size) + x_offset;
		let y_offset = ((viewbox_height - grid_height) / 2) + padding + (stroke_width / 2);
		let y = ((count % WEEKS_IN_A_YEAR) * outer_shape_size) + y_offset;

		let cx_offset = ((viewbox_width - grid_width) / 2) + (padding / 2) + (outer_shape_size / 2);
		let cx = ((count / WEEKS_IN_A_YEAR) * outer_shape_size) + cx_offset;
		let cy_offset =
			((viewbox_height - grid_height) / 2) + (padding / 2) + (outer_shape_size / 2);
		let cy = ((count % WEEKS_IN_A_YEAR) * outer_shape_size) + cy_offset;
		let shape: Element = match *week_shape {
			SvgShape::Square => Rectangle::new()
				.set("x", x)
				.set("y", y)
				.set("width", inner_shape_size)
				.set("height", inner_shape_size)
				.set("fill", fill)
				.set("stroke", color_primary.as_str())
				.set("stroke-width", stroke_width)
				.into(),
			SvgShape::Circle => Circle::new()
				.set("cx", cx)
				.set("cy", cy)
				.set("r", inner_shape_size / 2)
				.set("fill", fill)
				.set("stroke", color_primary.as_str())
				.set("stroke-width", stroke_width)
				.into(),
		};

		document.append(shape);
		// All this below is just to make sure there are always 52 weeks. To do this, we change the
		// number of days in a week and skip the 29th of February whenever it comes.
		let week_length = if count % 52 == 0 { 8 } else { 7 };
		for _ in 0_u8..week_length {
			curr_date = curr_date.next();
			if curr_date.month() == 2 && curr_date.day() == 29 {
				curr_date = curr_date.next();
			}
		}
		count += 1;
	}
	Ok(document)
}
