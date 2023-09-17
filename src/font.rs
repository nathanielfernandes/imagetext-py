use imagetext::prelude::*;
use pyo3::prelude::*;

use crate::objects::EmojiOptions;

#[pyclass]
pub struct Font {
    pub superfont: SuperFont<'static>,
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
            superfont: SuperFont::with_emoji_options(
                font,
                fallbacks,
                emoji_options.unwrap_or_default().to_emoji_options(),
            ),
        })
    }

    pub fn set_emoji_options(&mut self, emoji_options: EmojiOptions) {
        self.superfont.emoji_options = emoji_options.to_emoji_options();
    }
}

#[pyclass]
pub struct FontDB;

#[allow(non_snake_case)]
#[pymethods]
impl FontDB {
    #[staticmethod]
    pub fn LoadFromPath(name: &str, path: &str) -> PyResult<()> {
        imagetext::fontdb::FontDB::load_from_path(name, path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to load font: {}", e))
        })
    }

    #[staticmethod]
    pub fn LoadFromDir(path: &str) {
        imagetext::fontdb::FontDB::load_from_dir(path)
    }

    #[staticmethod]
    pub fn LoadSystemFonts() {
        imagetext::fontdb::FontDB::load_system_fonts()
    }

    #[staticmethod]
    pub fn Query(query: &str) -> PyResult<Font> {
        let font = imagetext::fontdb::FontDB::query(query).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "No fonts found for query: {}",
                query
            ))
        })?;
        Ok(Font { superfont: font })
    }

    #[staticmethod]
    pub fn Get(name: &str) -> PyResult<Font> {
        let font = imagetext::fontdb::FontDB::get(name).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "No fonts found for name: {}",
                name
            ))
        })?;
        Ok(Font {
            superfont: SuperFont::new(font, vec![]),
        })
    }

    #[staticmethod]
    pub fn QueryWithEmoji(query: &str, emoji_options: EmojiOptions) -> PyResult<Font> {
        let font =
            imagetext::fontdb::FontDB::query_with_emoji(query, emoji_options.to_emoji_options())
                .ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "No fonts found for query: {}",
                        query
                    ))
                })?;
        Ok(Font { superfont: font })
    }

    #[staticmethod]
    pub fn Remove(name: &str) -> PyResult<()> {
        imagetext::fontdb::FontDB::remove(name).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to remove font: {}", e))
        })
    }

    #[staticmethod]
    pub fn SetDefaultEmojiOptions(emoji_options: EmojiOptions) {
        imagetext::fontdb::FontDB::set_default_emoji_options(emoji_options.to_emoji_options())
    }
}
