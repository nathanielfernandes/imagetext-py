use imagetext::prelude::*;
use pyo3::prelude::*;

use crate::objects::EmojiOptions;

#[pyclass]
pub struct Font {
    pub font: imagetext::prelude::Font<'static>,
    pub fallbacks: Vec<imagetext::prelude::Font<'static>>,
    pub emoji_options: imagetext::emoji::EmojiOptions,
}

#[pymethods]
impl Font {
    #[new]
    fn new(
        path: &str,
        fallbacks: Option<Vec<String>>,
        emoji_options: Option<EmojiOptions>,
    ) -> PyResult<Self> {
        let font = load_font(path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to load font: {}", e))
        })?;

        let fallbacks = if let Some(fallbacks) = fallbacks {
            fallbacks
                .into_iter()
                .map(|path| {
                    load_font(&path).map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                            "Failed to load font: {}",
                            e
                        ))
                    })
                })
                .collect::<PyResult<Vec<_>>>()?
        } else {
            Vec::new()
        };

        Ok(Font {
            font,
            fallbacks,
            emoji_options: emoji_options.unwrap_or_default().to_emoji_options(),
        })
    }
}

impl Font {
    pub fn superfont<'a>(&'a self) -> SuperFont<'a, 'a> {
        SuperFont::with_emoji_options(&self.font, &self.fallbacks, self.emoji_options.clone())
    }
}
