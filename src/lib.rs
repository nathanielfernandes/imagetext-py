pub mod canvas;
pub mod drawing;
pub mod font;
pub mod objects;
pub mod paint;
pub mod utils;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn imagetext_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<canvas::Canvas>()?;
    m.add_class::<font::Font>()?;
    m.add_class::<font::FontDB>()?;
    m.add_class::<paint::Paint>()?;
    m.add_class::<objects::TextAlign>()?;
    m.add_class::<objects::EmojiSource>()?;
    m.add_class::<objects::WrapStyle>()?;

    m.add_function(wrap_pyfunction!(drawing::draw_text, m)?)?;
    m.add_function(wrap_pyfunction!(drawing::draw_text_anchored, m)?)?;
    m.add_function(wrap_pyfunction!(drawing::draw_text_multiline, m)?)?;
    m.add_function(wrap_pyfunction!(drawing::draw_text_wrapped, m)?)?;

    m.add_function(wrap_pyfunction!(utils::text_size, m)?)?;
    m.add_function(wrap_pyfunction!(utils::text_size_multiline, m)?)?;
    m.add_function(wrap_pyfunction!(utils::text_wrap, m)?)?;

    m.add_function(wrap_pyfunction!(utils::prebuild_static_vars, m)?)?;

    Ok(())
}
