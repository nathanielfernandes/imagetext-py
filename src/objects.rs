use pyo3::prelude::*;

#[derive(FromPyObject)]
pub struct Color(pub [u8; 4]);

#[pyclass]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

impl TextAlign {
    pub fn to_align(&self) -> imagetext::outliner::TextAlign {
        match self {
            TextAlign::Left => imagetext::outliner::TextAlign::Left,
            TextAlign::Center => imagetext::outliner::TextAlign::Center,
            TextAlign::Right => imagetext::outliner::TextAlign::Right,
        }
    }
}
