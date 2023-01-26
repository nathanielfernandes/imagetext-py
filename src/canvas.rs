use std::sync::{Arc, RwLock};

use pyo3::{prelude::*, types::PyBytes};

use crate::objects::Color;

#[derive(Clone)]
#[pyclass]
pub struct Canvas {
    pub im: Arc<RwLock<image::RgbaImage>>,
}

#[pymethods]
impl Canvas {
    #[new]
    fn new(width: u32, height: u32, color: Color) -> Self {
        Canvas {
            im: Arc::new(RwLock::new(image::RgbaImage::from_pixel(
                width,
                height,
                image::Rgba(color.0),
            ))),
        }
    }

    fn save(&self, path: &str) -> PyResult<()> {
        match self.im.read() {
            Ok(im) => im.save(path).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to save image: {}", e))
            }),
            Err(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Failed to lock image",
            )),
        }
    }

    fn to_bytes(&self) -> PyResult<((u32, u32), PyObject)> {
        match self.im.read() {
            Ok(im) => {
                let (width, height) = im.dimensions();
                Python::with_gil(|py| Ok(((width, height), PyBytes::new(py, &im).into())))
            }
            Err(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Failed to lock image",
            )),
        }
    }

    fn to_buffer(&self) -> PyResult<Vec<u8>> {
        match self.im.read() {
            Ok(im) => Ok(im.to_vec()),
            Err(_) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Failed to lock image",
            )),
        }
    }

    #[staticmethod]
    fn from_image(mut image: &PyAny) -> PyResult<Self> {
        let mode: &str = image.getattr("mode")?.extract()?;
        let width: u32 = image.getattr("width")?.extract()?;
        let height: u32 = image.getattr("height")?.extract()?;
        if mode != "RGBA" {
            image = image.call_method1("convert", ("RGBA",))?;
        }
        let buffer: Vec<u8> = image.call_method0("tobytes")?.extract()?;

        Ok(Canvas {
            im: Arc::new(RwLock::new(
                image::RgbaImage::from_raw(width, height, buffer).ok_or(PyErr::new::<
                    pyo3::exceptions::PyValueError,
                    _,
                >(
                    "Failed to convert image",
                ))?,
            )),
        })
    }

    fn to_image(&self) -> PyResult<PyObject> {
        let ((width, height), data) = self.to_bytes()?;
        Python::with_gil(|py| {
            let pil = PyModule::import(py, "PIL")?;
            let image: PyObject = pil
                .getattr("Image")?
                .getattr("frombytes")?
                .call1(("RGBA", (width, height), data))?
                .extract()?;
            Ok(image)
        })
    }
}
