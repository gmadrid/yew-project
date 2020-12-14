use crate::decorators::StyleDecorator;
use grids::Color;

pub struct ColorDecorator {}

impl ColorDecorator {
    pub fn new() -> Self {
        ColorDecorator {}
    }
}

impl StyleDecorator for ColorDecorator {
    fn cell_style(&self, _row: usize, _col: usize, contents: Color) -> Option<Vec<String>> {
        Some(vec![format!("background: {}", contents.to_string())])
    }
}
