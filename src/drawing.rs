use pyo3::prelude::*;

use crate::canvas;
use crate::font::Font;
use crate::objects::TextAlign;
use crate::paint::Paint;

use imagetext::drawing::prelude::*;

#[pyfunction]
pub fn draw_text(
    canvas: &mut canvas::Canvas,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    font: &Font,
    fill: &Paint,
    stroke: Option<f32>,
    stroke_color: Option<&Paint>,
) -> PyResult<()> {
    imagetext::drawing::text::draw_text_mut(
        &mut canvas.im,
        &fill.0,
        stroke
            .map(|s| imagetext::drawing::utils::stroke(s))
            .as_ref(),
        stroke_color.map(|c| &c.0),
        x,
        y,
        scale(size),
        &font.superfont(),
        text,
    )
    .map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to draw text: {}", e))
    })
}

#[pyfunction]
pub fn draw_text_anchored(
    canvas: &mut canvas::Canvas,
    text: &str,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    size: f32,
    font: &Font,
    fill: &Paint,
    stroke: Option<f32>,
    stroke_color: Option<&Paint>,
) -> PyResult<()> {
    imagetext::drawing::text::draw_text_anchored(
        &mut canvas.im,
        &fill.0,
        stroke
            .map(|s| imagetext::drawing::utils::stroke(s))
            .as_ref(),
        stroke_color.map(|c| &c.0),
        x,
        y,
        ax,
        ay,
        scale(size),
        &font.superfont(),
        text,
    )
    .map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to draw text: {}", e))
    })
}

#[pyfunction]
pub fn draw_text_multiline(
    canvas: &mut canvas::Canvas,
    lines: Vec<String>,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    width: f32,
    size: f32,
    font: &Font,
    fill: &Paint,
    line_spacing: Option<f32>,
    align: Option<&TextAlign>,
    stroke: Option<f32>,
    stroke_color: Option<&Paint>,
) -> PyResult<()> {
    imagetext::drawing::text::draw_text_multiline(
        &mut canvas.im,
        &fill.0,
        stroke
            .map(|s| imagetext::drawing::utils::stroke(s))
            .as_ref(),
        stroke_color.map(|c| &c.0),
        x,
        y,
        ax,
        ay,
        width,
        scale(size),
        &font.superfont(),
        &lines,
        line_spacing.unwrap_or(1.0),
        align.unwrap_or(&TextAlign::Left).to_align(),
    )
    .map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to draw text: {}", e))
    })
}

#[pyfunction]
pub fn draw_text_wrapped(
    canvas: &mut canvas::Canvas,
    text: &str,
    x: f32,
    y: f32,
    ax: f32,
    ay: f32,
    width: f32,
    size: f32,
    font: &Font,
    fill: &Paint,
    line_spacing: Option<f32>,
    align: Option<&TextAlign>,
    stroke: Option<f32>,
    stroke_color: Option<&Paint>,
) -> PyResult<()> {
    imagetext::drawing::text::draw_text_wrapped(
        &mut canvas.im,
        &fill.0,
        stroke
            .map(|s| imagetext::drawing::utils::stroke(s))
            .as_ref(),
        stroke_color.map(|c| &c.0),
        x,
        y,
        ax,
        ay,
        width,
        scale(size),
        &font.superfont(),
        text,
        line_spacing.unwrap_or(1.0),
        align.unwrap_or(&TextAlign::Left).to_align(),
    )
    .map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to draw text: {}", e))
    })
}
