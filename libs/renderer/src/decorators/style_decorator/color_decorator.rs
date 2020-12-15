use crate::decorators::StyleDecorator;
use grids::Color;

pub struct ColorDecorator {}

impl Default for ColorDecorator {
    fn default() -> Self {
        ColorDecorator {}
    }
}

impl StyleDecorator for ColorDecorator {
    fn cell_style(&self, _row: usize, _col: usize, contents: Color) -> Option<Vec<String>> {
        Some(vec![format!("background: {}", contents.to_string())])
    }
}
