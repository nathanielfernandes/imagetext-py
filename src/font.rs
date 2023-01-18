use imagetext::drawing::prelude::*;
use pyo3::prelude::*;

#[pyclass]
pub struct Font {
    pub font: imagetext::drawing::prelude::Font<'static>,
    pub fallbacks: Vec<imagetext::drawing::prelude::Font<'static>>,
}

#[pymethods]
impl Font {
    #[new]
    fn new(path: &str, fallbacks: Option<Vec<String>>) -> PyResult<Self> {
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

        Ok(Font { font, fallbacks })
    }
}

impl Font {
    pub fn superfont<'a>(&'a self) -> SuperFont<'a, 'a> {
        SuperFont::new(&self.font, &self.fallbacks)
    }
}
