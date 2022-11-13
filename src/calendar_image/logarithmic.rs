use std::num::TryFromIntError;

use anyhow::Result;
use death_calendar::days_lived;
use gregorian::Date;
use svg::{
	node::{
		element::{self, Line, Marker, Polyline, Text},
		Text as TextNode,
	},
	Document, Node,
};

use super::{init_document, AVERAGE_DAYS_IN_YEAR, PHI};
use crate::{BirthInfo, DrawingInfoValidated};

fn position_from_0_to_1(lifespan: u16, inc: f64) -> f64 {
	let lifespanf = f64::from(lifespan);
	1_f64 - (f64::powf(lifespanf + 1_f64, 1_f64 - (inc / lifespanf)) - 1_f64) / lifespanf
}

fn make_arrowhead(color_primary: &str) -> Marker {
	let scale = 4.0;
	let arrowhead_width = PHI * scale;
	let arrowhead_height = 1.0 * scale;
	let arrowhead_midpoint = arrowhead_height / 2.0;
	Marker::new()
		.set("id", "arrowhead")
		.set("markerWidth", arrowhead_width)
		.set("markerHeight", arrowhead_height)
		.set("refX", 0_u8)
		.set("refY", arrowhead_midpoint)
		.set("orient", "auto")
		.set("fill", color_primary)
		.add(element::Polygon::new().set(
			"points",
			format!("0 0, {arrowhead_width}, {arrowhead_midpoint}, 0 {arrowhead_height}"),
		))
}

fn str_to_charcount(s: &str) -> Result<u32, TryFromIntError> {
	u32::try_from(s.chars().count())
}

fn num_to_charcount(num: u32) -> Result<u32> {
	Ok(str_to_charcount(&num.to_string())?)
}

pub fn render_svg(
	common_args: &BirthInfo,
	drawing_info: &DrawingInfoValidated,
	width_to_height_ratio: f64,
) -> Result<Document> {
	let color_primary = drawing_info.color_primary.to_hex_string();
	let color_secondary = drawing_info.color_secondary.to_hex_string();
	let scale_factor = drawing_info.scale_factor;

	let bday = common_args.birthday;
	let lifespan_years = common_args.lifespan_years;

	// I'm displaying fonts with length defined as pixels.
	let font_size_pixels: f64 = (24 * scale_factor).into();
	// The length of a whole number must be a multiple of the pixel length of digits
	let text_size =
		font_size_pixels * <u32 as Into<f64>>::into(num_to_charcount(lifespan_years.into())?) / 2.0;

	let inner_width = 6.0 * width_to_height_ratio * font_size_pixels;
	let inner_height = 6.0 * font_size_pixels;

	let stroke_width = font_size_pixels / 12.0;
	let padding_y = (text_size / 0.8_f64) / 2.0;

	let label = "You Are Here";
	let label_width = f64::from(str_to_charcount(label)?);
	let padding_x = ((font_size_pixels / 2.0) * label_width) / 2.0;

	let viewbox_width = padding_x.mul_add(2.0, inner_width);
	let viewbox_height = padding_y.mul_add(2.0, inner_height);

	let mut document = init_document(viewbox_width, viewbox_height, &color_secondary);

	let position_within_inner_viewbox = |inc: f64| -> f64 {
		position_from_0_to_1(lifespan_years, inc).mul_add(inner_width, padding_x)
	};

	let arrow_length = font_size_pixels * 3.0;
	let arrowhead = make_arrowhead(&color_primary);
	document.append(arrowhead);

	let years_lived_so_far = f64::from(days_lived(Date::today_utc(), bday)) / AVERAGE_DAYS_IN_YEAR;
	let timeline_pos_today = position_within_inner_viewbox(years_lived_so_far);

	let today_arrow = Line::new()
		.set("x1", timeline_pos_today)
		.set("x2", timeline_pos_today)
		.set("y1", padding_y + font_size_pixels)
		.set("y2", padding_y + arrow_length)
		.set("marker-end", "url(#arrowhead)")
		.set("stroke-width", stroke_width)
		.set("stroke", color_primary.as_str());
	document.append(today_arrow);

	// Baseline
	let baseline_height = inner_height;
	let top_of_line_height = font_size_pixels + padding_y + arrow_length;
	let initial_x = position_within_inner_viewbox(0_f64);
	let final_x = position_within_inner_viewbox(lifespan_years.into());
	document.append(
		Polyline::new()
			.set(
				"points",
				format!(
					"{initial_x} {top_of_line_height},{initial_x} {baseline_height},{final_x} \
					 {baseline_height},{final_x} {top_of_line_height}"
				),
			)
			.set("fill", "none")
			.set("stroke-width", stroke_width)
			.set("stroke", color_primary.as_str()),
	);

	document.append(
		Text::new()
			.set("x", timeline_pos_today)
			.set("y", padding_y + (font_size_pixels / 2.0))
			.set("stroke", color_primary.as_str())
			.set("fill", color_primary.as_str())
			.set("font-size", format!("{font_size_pixels}px"))
			.set("text-anchor", "middle")
			.add(TextNode::new(label)),
	);

	let mut previous_x = f64::MIN;

	for year_num in 0..=lifespan_years {
		let current_x = position_within_inner_viewbox(f64::from(year_num));
		let d_prev_curr = current_x - previous_x;
		let d_curr_last = final_x - current_x;

		let year_digits = num_to_charcount(year_num.into())?;

		let gap_size = (font_size_pixels * f64::from(year_digits)) / PHI;

		let there_is_enough_space_between_lines = d_prev_curr >= gap_size && d_curr_last > gap_size;

		let year_label_text = Text::new()
			.set("x", current_x)
			.set("y", baseline_height + font_size_pixels)
			.set("stroke", color_primary.as_str())
			.set("fill", color_primary.as_str())
			.set("font-size", format!("{font_size_pixels}px"))
			.set("text-anchor", "middle")
			.add(TextNode::new(year_num.to_string()));
		let year_label_line = Line::new()
			.set("x1", current_x)
			.set("y1", baseline_height)
			.set("x2", current_x)
			.set("y2", font_size_pixels + padding_y + arrow_length)
			.set("stroke-width", stroke_width)
			.set("stroke", color_primary.as_str());

		if year_num == 0 || year_num == lifespan_years {
			document.append(year_label_text);
			previous_x = current_x;
		} else if there_is_enough_space_between_lines {
			document.append(year_label_line);
			document.append(year_label_text);
			previous_x = current_x;
		} else {
			// Do nothing
		}
	}

	Ok(document)
}
