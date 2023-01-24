use pyo3::prelude::*;

use crate::font::Font;
use imagetext::prelude::*;

#[pyfunction]
pub fn text_size(text: &str, size: f32, font: &Font, draw_emojis: Option<bool>) -> (i32, i32) {
    if draw_emojis.unwrap_or(false) {
        imagetext::measure::text_size_with_emojis(scale(size), &font.superfont(), text)
    } else {
        imagetext::measure::text_size(scale(size), &font.superfont(), text)
    }
}

#[pyfunction]
pub fn text_size_multiline(
    lines: Vec<String>,
    size: f32,
    font: &Font,
    line_spacing: Option<f32>,
    draw_emojis: Option<bool>,
) -> (i32, i32) {
    if draw_emojis.unwrap_or(false) {
        imagetext::measure::text_size_multiline_with_emojis(
            &lines,
            &font.superfont(),
            scale(size),
            line_spacing.unwrap_or(1.0),
        )
    } else {
        imagetext::measure::text_size_multiline(
            &lines,
            &font.superfont(),
            scale(size),
            line_spacing.unwrap_or(1.0),
        )
    }
}

#[pyfunction]
pub fn word_wrap(
    text: &str,
    width: i32,
    size: f32,
    font: &Font,
    draw_emojis: Option<bool>,
) -> Vec<String> {
    if draw_emojis.unwrap_or(false) {
        imagetext::wrap::word_wrap_with_emojis(text, width, &font.superfont(), scale(size))
    } else {
        imagetext::wrap::word_wrap(text, width, &font.superfont(), scale(size))
    }
}
