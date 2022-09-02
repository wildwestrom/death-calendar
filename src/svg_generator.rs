use std::{error::Error, num::ParseIntError, str::FromStr};

use death_calendar::death_day;

use gregorian::Date;
use hex_color::HexColor;
use svg::{
    node::element::{Circle, Element, Rectangle},
    Document, Node,
};

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum BorderUnit {
    Pixel,
    Shape,
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

impl Error for ParseBorderUnitError {}

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

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum SvgShape {
    Square,
    Circle,
}

#[derive(Debug, Clone)]
pub struct DrawingRatios {
    // How thick should the line around the shape be?
    stroke: u32,
    // How much space should be around each shape?
    padding: u32,
    // How long should the shape be on the inside?
    length: u32,
    // How much space should be around the grid?
    border: u32,
    // Should the border be measured in pixels or the shape?
    border_unit: BorderUnit,
}

#[derive(Debug)]
pub struct ParseDrawingRatiosError {
    message: String,
}

impl std::fmt::Display for ParseDrawingRatiosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nCould not parse the ratio string: {}.", self.message)
    }
}

impl Error for ParseDrawingRatiosError {}

impl From<ParseIntError> for ParseDrawingRatiosError {
    fn from(_: ParseIntError) -> Self {
        Self {
            message: "Value must be a number".into(),
        }
    }
}

impl From<ParseBorderUnitError> for ParseDrawingRatiosError {
    fn from(_: ParseBorderUnitError) -> Self {
        Self {
            message: "Border unit can only be 'pixel' or 'shape'".into(),
        }
    }
}

impl FromStr for DrawingRatios {
    type Err = ParseDrawingRatiosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_str: Vec<&str> = s.split(',').collect();
        if split_str.len() != 5 {
            return Err(ParseDrawingRatiosError {
                message: "You do not have the correct number of values".into(),
            });
        }
        Ok(Self {
            stroke: split_str[0].parse()?,
            padding: split_str[1].parse()?,
            length: split_str[2].parse()?,
            border: split_str[3].parse()?,
            border_unit: split_str[4].parse()?,
        })
    }
}

fn invert_color(color: HexColor) -> HexColor {
    HexColor::new(255, 255, 255) - color
}

const WEEKS_IN_A_YEAR: u32 = 52;

#[must_use]
pub fn render_svg(
    bday: Date,
    years: i16,
    drawing_ratios: &DrawingRatios,
    shape_type: &SvgShape,
    scale_factor: u32,
    color_primary_hexcolor: HexColor,
    color_secondary_hexcolor: Option<HexColor>,
) -> Document {
    let color_secondary = match color_secondary_hexcolor {
        Some(color) => color,
        None => invert_color(color_primary_hexcolor),
    }
    .to_string();

    let color_primary = color_primary_hexcolor.to_string();

    let today = Date::today_utc();
    let end = death_day(bday, years);

    let stroke_width = drawing_ratios.stroke * scale_factor * 2;

    let padding = drawing_ratios.padding * scale_factor;
    let inner_shape_size = (drawing_ratios.length * 2) * scale_factor + stroke_width;
    let outer_shape_size = inner_shape_size + (padding * 2) + stroke_width;

    let border = match drawing_ratios.border_unit {
        BorderUnit::Pixel => drawing_ratios.border * scale_factor,
        BorderUnit::Shape => drawing_ratios.border * scale_factor * outer_shape_size,
    };

    // In total, the outer dimensions of a shape is a function of its stroke-width x 2,
    // hence the variable `space_around_shape`.
    let grid_width = outer_shape_size
        * u32::try_from(years).expect("Couldn't convert number of years to a u32.");
    let grid_height = outer_shape_size * WEEKS_IN_A_YEAR;

    let viewbox_width = grid_width + (border * 2) + (padding * 2);
    let viewbox_height = grid_height + (border * 2) + (padding * 2);

    let mut document = Document::new()
        .set("viewBox", (0, 0, viewbox_width, viewbox_height))
        .set("style", format!("background-color:{}", color_primary));

    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("width", viewbox_width)
        .set("height", viewbox_height)
        .set("fill", color_secondary.as_str());

    document.append(background);

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
        let shape: Element = match shape_type {
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
        /* All this below is just to make sure there are always 52 weeks. To do this, we change the
        number of days in a week and skip the 29th of February whenever it comes. */
        let week_length = if count % 52 == 0 { 8 } else { 7 };
        for _ in 0..week_length {
            curr_date = curr_date.next();
            if curr_date.month() == 2 && curr_date.day() == 29 {
                curr_date = curr_date.next();
            }
        }
        count += 1;
    }

    document
}
