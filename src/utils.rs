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
    py: Python,
    text: &str,
    width: i32,
    size: f32,
    font: &Font,
    draw_emojis: Option<bool>,
    wrap_style: Option<WrapStyle>,
) -> Vec<String> {
    py.allow_threads(|| {
        if draw_emojis.unwrap_or(false) {
            let (text, emojis) = imagetext::emoji::parse::parse_out_emojis(
                text,
                font.superfont.emoji_options.parse_shortcodes,
                font.superfont.emoji_options.parse_discord_emojis,
            );

            let mut lines = imagetext::wrap::text_wrap(
                &text,
                width,
                &font.superfont,
                scale(size),
                wrap_style.unwrap_or(WrapStyle::Word).to_wrap_style(),
                text_width_with_emojis,
            );

            let mut emojis_iter = emojis.iter();
            lines.iter_mut().for_each(|line| {
                let found = line.matches("😀").count();

                for _ in 0..found {
                    if let Some(emoji) = emojis_iter.next() {
                        let rep = match emoji {
                            EmojiType::Discord(id) => format!("<:ee:{}>", id),
                            EmojiType::Regular(e) => {
                                if let Some(code) = e.shortcode() {
                                    format!(":{}:", code)
                                } else {
                                    e.as_str().to_string()
                                }
                            }
                        };
                        *line = line.replacen("😀", &rep, 1);
                    }
                }
            });

            lines
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
    })
}
