#![allow(unused_imports)]

use std::{error::Error, num::ParseIntError, str::FromStr};

use death_calendar::{days_lived, death_day, years_left, years_lived};
use gregorian::{Date, DateResultExt};
use hex_color::HexColor;
use svg::{
	node::element::{Circle, Element, Line, Rectangle},
	Document, Node,
};

#[must_use]
pub fn render_svg(
	bday: Date,
	lifespan_years: i16,
	scale_factor: u32,
	color_primary_hexcolor: HexColor,
	color_secondary_hexcolor: Option<HexColor>,
) -> Document {
	// I'm displaying fonts with length defined as pixels.
	let font_size_pixels = 24 * scale_factor;
	// The length of a whole number must be a multiple of the pixel length of digits
	let text_size = font_size_pixels * 2;

	let inner_width = font_size_pixels * 100;
	let inner_height = font_size_pixels * 9;

	let padding = text_size;

	let viewbox_width = inner_width + text_size;
	let viewbox_height = inner_height;

	let color_secondary = color_secondary_hexcolor
		.map_or_else(|| color_primary_hexcolor.invert(), |color| color)
		.to_string();

	let color_primary = color_primary_hexcolor.to_string();

	let mut document = Document::new()
		.set("viewBox", (0_u8, 0_u8, viewbox_width, viewbox_height))
		.set("style", format!("background-color:{}", color_primary));

	let background = Rectangle::new()
		.set("x", 0_u8)
		.set("y", 0_u8)
		.set("width", viewbox_width)
		.set("height", viewbox_height)
		.set("fill", color_secondary.as_str());

	document.append(background);

	let stroke_width = 2 * scale_factor;

	// Baseline
	let line = Line::new()
		.set("x1", padding / 2)
		.set("y1", inner_height - font_size_pixels)
		.set("x2", inner_width + (padding / 2))
		.set("y2", inner_height - font_size_pixels)
		.set("stroke", color_primary.as_str())
		.set("stroke-width", stroke_width);
	document.append(line);

	fn position_from_0_to_1(lifespan: i16, inc: f64) -> f64 {
		1.0 - (f64::powf(lifespan as f64 + 1.0, 1.0 - (inc / lifespan as f64)) - 1.0)
			/ lifespan as f64
	}

	const AVERAGE_DAYS_IN_YEAR: f64 = 365.2425;
	let years_lived_so_far = days_lived(Date::today_utc(), bday) as f64 / AVERAGE_DAYS_IN_YEAR;

	let scale_pos_bday = position_from_0_to_1(lifespan_years, years_lived_so_far as f64)
		* inner_width as f64
		+ (padding / 2) as f64;

	let birthday_dot_color = "red";
	let birthday_dot = Circle::new()
		.set("cy", font_size_pixels * 2)
		.set("cx", scale_pos_bday)
		.set("r", font_size_pixels / 2)
		.set("fill", birthday_dot_color);
	document.append(birthday_dot);

	let mut prev = 0.0;
	let mut curr = 0.0;
	let last_scale_pos =
		position_from_0_to_1(lifespan_years, lifespan_years.into()) * inner_width as f64;

	for i in 0..=lifespan_years {
		let current_scale_pos = position_from_0_to_1(lifespan_years, i as f64) * inner_width as f64;
		let year_num = i;

		let mut add_year_label = || {
			// Number text
			document.append(
				svg::node::element::Text::new()
					.set("y", font_size_pixels)
					.set("x", current_scale_pos + (padding / 2) as f64)
					.set("fill", color_primary.as_str())
					.set("font-size", format!("{}px", font_size_pixels))
					.set("text-anchor", "middle")
					.add(svg::node::Text::new(format!("{}", year_num))),
			);
			// Vertical Lines
			document.append(
				Line::new()
					.set("y1", font_size_pixels * 2)
					.set("x1", current_scale_pos + (padding / 2) as f64)
					.set("y2", inner_height - font_size_pixels)
					.set("x2", current_scale_pos + (padding / 2) as f64)
					.set("stroke-width", stroke_width)
					.set("stroke", color_primary.as_str()),
			);
		};

		let d_prev_curr = curr - prev;
		let d_curr_last = last_scale_pos - curr;

		if i == 0
			|| i == 1 || (d_prev_curr > text_size.into() && d_curr_last > text_size.into())
			|| i == lifespan_years
		{
			add_year_label();
			prev = curr;
		}

		curr = current_scale_pos;
	}

	document
}
