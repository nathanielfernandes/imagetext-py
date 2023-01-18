use crate::objects::Color;

use imagetext::drawing::prelude::*;
use pyo3::prelude::*;

#[pyclass]
pub struct Paint(pub imagetext::drawing::prelude::Paint<'static>);

#[allow(non_snake_case)]
#[pymethods]
impl Paint {
    #[new]
    fn new(color: Option<Color>, anti_alias: Option<bool>) -> Self {
        let mut paint = imagetext::drawing::prelude::Paint::default();

        if let Some(color) = color {
            paint.set_color_rgba8(color.0[0], color.0[1], color.0[2], color.0[3]);
        }

        paint.anti_alias = anti_alias.unwrap_or(true);

        Paint(paint)
    }

    fn set_color(&mut self, color: Color) {
        self.0
            .set_color_rgba8(color.0[0], color.0[1], color.0[2], color.0[3]);
    }

    fn set_anti_alias(&mut self, anti_alias: bool) {
        self.0.anti_alias = anti_alias;
    }

    #[staticmethod]
    fn Color(color: Color) -> Self {
        Paint(paint_from_rgba_slice(&color.0))
    }

    #[staticmethod]
    fn Gradient(start: (f32, f32), stop: (f32, f32), colors: Vec<Color>) -> Self {
        Paint(ez_gradient(
            point(start.0, start.1),
            point(stop.0, stop.1),
            colors.iter().map(|c| color_from_rgba_slice(&c.0)).collect(),
        ))
    }

    #[staticmethod]
    fn Rainbow(start: (f32, f32), stop: (f32, f32)) -> Self {
        Paint(rainbow(point(start.0, start.1), point(stop.0, stop.1)))
    }
}
