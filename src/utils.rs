use pyo3::prelude::*;

use crate::font::Font;
use imagetext::drawing::prelude::*;

#[pyfunction]
pub fn text_size(text: &str, size: f32, font: &Font) -> (i32, i32) {
    imagetext::measure::text_size(scale(size), &font.superfont(), text)
}

#[pyfunction]
pub fn text_size_multiline(
    lines: Vec<String>,
    size: f32,
    font: &Font,
    line_spacing: Option<f32>,
) -> (i32, i32) {
    imagetext::measure::text_size_multiline(
        &lines,
        &font.superfont(),
        scale(size),
        line_spacing.unwrap_or(1.0),
    )
}

#[pyfunction]
pub fn split_on_space(text: &str) -> Vec<&str> {
    imagetext::wrap::split_on_space(text)
}

#[pyfunction]
pub fn word_wrap(text: &str, width: i32, size: f32, font: &Font) -> Vec<String> {
    imagetext::wrap::word_wrap(text, width, &font.superfont(), scale(size))
}
