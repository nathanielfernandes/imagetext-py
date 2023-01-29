use pyo3::prelude::*;

use crate::{font::Font, objects::WrapStyle};
use imagetext::prelude::*;

#[pyfunction]
pub fn text_size(
    py: Python,
    text: &str,
    size: f32,
    font: &Font,
    draw_emojis: Option<bool>,
) -> (i32, i32) {
    py.allow_threads(|| {
        if draw_emojis.unwrap_or(false) {
            imagetext::measure::text_size_with_emojis(scale(size), &font.superfont, text)
        } else {
            imagetext::measure::text_size(scale(size), &font.superfont, text)
        }
    })
}

#[pyfunction]
pub fn text_size_multiline(
    py: Python,
    lines: Vec<String>,
    size: f32,
    font: &Font,
    line_spacing: Option<f32>,
    draw_emojis: Option<bool>,
) -> (i32, i32) {
    py.allow_threads(|| {
        if draw_emojis.unwrap_or(false) {
            imagetext::measure::text_size_multiline_with_emojis(
                &lines,
                &font.superfont,
                scale(size),
                line_spacing.unwrap_or(1.0),
            )
        } else {
            imagetext::measure::text_size_multiline(
                &lines,
                &font.superfont,
                scale(size),
                line_spacing.unwrap_or(1.0),
            )
        }
    })
}

#[pyfunction]
pub fn text_wrap(
    text: &str,
    width: i32,
    size: f32,
    font: &Font,
    draw_emojis: Option<bool>,
    wrap_style: Option<WrapStyle>,
) -> Vec<String> {
    if draw_emojis.unwrap_or(false) {
        imagetext::wrap::text_wrap(
            text,
            width,
            &font.superfont,
            scale(size),
            wrap_style.unwrap_or(WrapStyle::Word).to_wrap_style(),
            text_width_with_emojis,
        )
    } else {
        imagetext::wrap::text_wrap(
            text,
            width,
            &font.superfont,
            scale(size),
            wrap_style.unwrap_or(WrapStyle::Word).to_wrap_style(),
            imagetext::measure::text_width,
        )
    }
}
