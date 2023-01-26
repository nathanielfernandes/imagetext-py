use pyo3::{prelude::*, types::PyBytes};

use crate::objects::Color;

#[pyclass]
pub struct Canvas {
    pub im: image::RgbaImage,
}

#[pymethods]
impl Canvas {
    #[new]
    fn new(width: u32, height: u32, color: Color) -> Self {
        Canvas {
            im: image::RgbaImage::from_pixel(width, height, image::Rgba(color.0)),
        }
    }

    fn save(&self, path: &str) -> PyResult<()> {
        self.im.save(path).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("Failed to save image: {}", e))
        })
    }

    fn to_bytes(&self) -> PyResult<((u32, u32), PyObject)> {
        let (width, height) = self.im.dimensions();
        Python::with_gil(|py| Ok(((width, height), PyBytes::new(py, &self.im).into())))
    }

    fn to_buffer(&self) -> PyResult<Vec<u8>> {
        Ok(self.im.to_vec())
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
            im: image::RgbaImage::from_raw(width, height, buffer).ok_or(PyErr::new::<
                pyo3::exceptions::PyValueError,
                _,
            >(
                "Failed to convert image",
            ))?,
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
