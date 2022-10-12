use death_calendar::days_lived;
use gregorian::Date;
use svg::{
	node::element::{Circle, Line, Rectangle},
	Document, Node,
};

use crate::{BirthInfo, DrawingInfoValidated};

#[allow(clippy::default_numeric_fallback)] // This is due to a false positive
const AVERAGE_DAYS_IN_YEAR: f64 = 365.2425;

fn position_from_0_to_1(lifespan: i16, inc: f64) -> f64 {
	let lifespanf = f64::from(lifespan);
	1.0_f64 - (f64::powf(lifespanf + 1.0_f64, 1.0_f64 - (inc / lifespanf)) - 1.0_f64) / lifespanf
}

#[must_use]
pub fn render_svg(common_args: &BirthInfo, drawing_info: &DrawingInfoValidated) -> Document {
	let color_primary = drawing_info.color_primary.to_string();
	let color_secondary = drawing_info.color_secondary.to_string();
	let scale_factor = drawing_info.scale_factor;

	let bday = common_args.birthday;
	let lifespan_years = common_args.lifespan_years;

	// I'm displaying fonts with length defined as pixels.
	let font_size_pixels = 24 * scale_factor;
	// The length of a whole number must be a multiple of the pixel length of digits
	let text_size = font_size_pixels * 2;

	let inner_width = font_size_pixels * 100;
	let inner_height = font_size_pixels * 9;

	let padding = text_size;

	let viewbox_width = inner_width + text_size;
	let viewbox_height = inner_height;

	let mut document = Document::new()
		.set("viewBox", (0_u8, 0_u8, viewbox_width, viewbox_height))
		.set("style", format!("background-color:{color_primary}"));

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

	let years_lived_so_far = f64::from(days_lived(Date::today_utc(), bday)) / AVERAGE_DAYS_IN_YEAR;

	let scale_pos_bday = position_from_0_to_1(lifespan_years, years_lived_so_far)
		.mul_add(f64::from(inner_width), f64::from(padding / 2));

	let birthday_dot_color = "red";
	let birthday_dot = Circle::new()
		.set("cy", font_size_pixels * 2)
		.set("cx", scale_pos_bday)
		.set("r", font_size_pixels / 2)
		.set("fill", birthday_dot_color);
	document.append(birthday_dot);

	let mut prev = 0.0_f64;
	let mut curr = 0.0_f64;
	let last_scale_pos =
		position_from_0_to_1(lifespan_years, lifespan_years.into()) * f64::from(inner_width);

	for i in 0..=lifespan_years {
		let current_scale_pos = position_from_0_to_1(lifespan_years, f64::from(i))
			.mul_add(f64::from(inner_width), f64::from(padding / 2));
		let year_num = i;

		let mut add_year_label = || {
			// Number text
			document.append(
				svg::node::element::Text::new()
					.set("y", font_size_pixels)
					.set("x", current_scale_pos)
					.set("fill", color_primary.as_str())
					.set("font-size", format!("{font_size_pixels}px"))
					.set("text-anchor", "middle")
					.add(svg::node::Text::new(format!("{year_num}"))),
			);
			// Vertical Lines
			document.append(
				Line::new()
					.set("y1", font_size_pixels * 2)
					.set("x1", current_scale_pos)
					.set("y2", inner_height - font_size_pixels)
					.set("x2", current_scale_pos)
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
