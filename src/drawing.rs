use imagetext::emoji::default_resolver::DefaultEmojiResolver;
use pyo3::prelude::*;

use crate::canvas;
use crate::font::Font;
use crate::objects::TextAlign;
use crate::paint::Paint;

use imagetext::prelude::*;

#[pyfunction]
pub fn draw_text(
    py: Python,
    canvas: canvas::Canvas,
    text: &str,
    x: f32,
    y: f32,
    size: f32,
    font: &Font,
    fill: &Paint,
    stroke: Option<f32>,
    stroke_color: Option<&Paint>,
    draw_emojis: Option<bool>,
) -> PyResult<()> {
    fn draw_text_inner(
        im: &mut image::RgbaImage,
        text: &str,
        x: f32,
        y: f32,
        size: f32,
        font: &Font,
        fill: &Paint,
        stroke: Option<f32>,
        stroke_color: Option<&Paint>,
        draw_emojis: Option<bool>,
    ) -> PyResult<()> {
        if draw_emojis.unwrap_or(false) {
            imagetext::drawing::text::draw_text_mut_with_emojis(
                im,
                &fill.0,
                stroke
                    .map(|s| imagetext::drawing::utils::stroke(s))
                    .as_ref(),
                stroke_color.map(|c| &c.0),
                x,
                y,
                scale(size),
                &font.superfont,
                DefaultEmojiResolver::<true>,
                text,
            )
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to draw text: {}",
                    e
                ))
            })
        } else {
            imagetext::drawing::text::draw_text_mut(
                im,
                &fill.0,
                stroke
                    .map(|s| imagetext::drawing::utils::stroke(s))
                    .as_ref(),
                stroke_color.map(|c| &c.0),
                x,
                y,
                scale(size),
                &font.superfont,
                text,
            )
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to draw text: {}",
                    e
                ))
            })
        }
    }

    py.allow_threads(|| match canvas.im.write() {
        Ok(mut im) => draw_text_inner(
            &mut im,
            text,
            x,
            y,
            size,
            font,
            fill,
            stroke,
            stroke_color,
            draw_emojis,
        ),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to draw text: {}",
            e
        ))),
    })
}

#[pyfunction]
pub fn draw_text_anchored(
    py: Python,
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
    draw_emojis: Option<bool>,
) -> PyResult<()> {
    fn draw_text_anchored_inner(
        im: &mut image::RgbaImage,
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
        draw_emojis: Option<bool>,
    ) -> PyResult<()> {
        if draw_emojis.unwrap_or(false) {
            imagetext::drawing::text::draw_text_anchored_with_emojis(
                im,
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
                &font.superfont,
                DefaultEmojiResolver::<true>,
                text,
            )
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to draw text: {}",
                    e
                ))
            })
        } else {
            imagetext::drawing::text::draw_text_anchored(
                im,
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
                &font.superfont,
                text,
            )
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to draw text: {}",
                    e
                ))
            })
        }
    }

    py.allow_threads(|| match canvas.im.write() {
        Ok(mut im) => draw_text_anchored_inner(
            &mut im,
            text,
            x,
            y,
            ax,
            ay,
            size,
            font,
            fill,
            stroke,
            stroke_color,
            draw_emojis,
        ),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to draw text: {}",
            e
        ))),
    })
}

#[pyfunction]
pub fn draw_text_multiline(
    py: Python,
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
    draw_emojis: Option<bool>,
) -> PyResult<()> {
    fn draw_text_multiline_inner(
        im: &mut image::RgbaImage,
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
        draw_emojis: Option<bool>,
    ) -> PyResult<()> {
        if draw_emojis.unwrap_or(false) {
            imagetext::drawing::text::draw_text_multiline_with_emojis(
                im,
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
                &font.superfont,
                DefaultEmojiResolver::<true>,
                &lines,
                line_spacing.unwrap_or(1.0),
                align.unwrap_or(&TextAlign::Left).to_align(),
            )
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to draw text: {}",
                    e
                ))
            })
        } else {
            imagetext::drawing::text::draw_text_multiline(
                im,
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
                &font.superfont,
                &lines,
                line_spacing.unwrap_or(1.0),
                align.unwrap_or(&TextAlign::Left).to_align(),
            )
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to draw text: {}",
                    e
                ))
            })
        }
    }

    py.allow_threads(|| match canvas.im.write() {
        Ok(mut im) => draw_text_multiline_inner(
            &mut im,
            lines,
            x,
            y,
            ax,
            ay,
            width,
            size,
            font,
            fill,
            line_spacing,
            align,
            stroke,
            stroke_color,
            draw_emojis,
        ),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to draw text: {}",
            e
        ))),
    })
}

#[pyfunction]
pub fn draw_text_wrapped(
    py: Python,
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
    draw_emojis: Option<bool>,
    wrap_style: Option<&crate::objects::WrapStyle>,
) -> PyResult<()> {
    fn draw_text_wrapped_inner(
        im: &mut image::RgbaImage,
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
        draw_emojis: Option<bool>,
        wrap_style: Option<&crate::objects::WrapStyle>,
    ) -> PyResult<()> {
        if draw_emojis.unwrap_or(false) {
            imagetext::drawing::text::draw_text_wrapped_with_emojis(
                im,
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
                &font.superfont,
                DefaultEmojiResolver::<true>,
                text,
                line_spacing.unwrap_or(1.0),
                align.unwrap_or(&TextAlign::Left).to_align(),
                wrap_style
                    .unwrap_or(&crate::objects::WrapStyle::Word)
                    .to_wrap_style(),
            )
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to draw text: {}",
                    e
                ))
            })
        } else {
            imagetext::drawing::text::draw_text_wrapped(
                im,
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
                &font.superfont,
                text,
                line_spacing.unwrap_or(1.0),
                align.unwrap_or(&TextAlign::Left).to_align(),
                wrap_style
                    .unwrap_or(&crate::objects::WrapStyle::Word)
                    .to_wrap_style(),
            )
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Failed to draw text: {}",
                    e
                ))
            })
        }
    }

    py.allow_threads(|| match canvas.im.write() {
        Ok(mut im) => draw_text_wrapped_inner(
            &mut im,
            text,
            x,
            y,
            ax,
            ay,
            width,
            size,
            font,
            fill,
            line_spacing,
            align,
            stroke,
            stroke_color,
            draw_emojis,
            wrap_style,
        ),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
            "Failed to draw text: {}",
            e
        ))),
    })
}
