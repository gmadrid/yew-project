use crate::decorators::{ClassDecoratorTrait, CssMunger, StyleDecorator};
use grids::Color;

pub struct CellSizeDecorator {
    cell_size: usize,

    cached_result: Vec<String>,
}

impl CellSizeDecorator {
    pub fn with_size(cell_size: usize) -> Self {
        let cached_result = vec![
            format!("height: {}px", cell_size),
            format!("width: {}px", cell_size),
        ];

        CellSizeDecorator {
            cell_size,
            cached_result,
        }
    }
}

impl StyleDecorator for CellSizeDecorator {
    fn cell_style(&self, _row: usize, _col: usize, _contents: Color) -> Option<Vec<String>> {
        Some(self.cached_result.clone())
    }
}

static ONCE: std::sync::Once = std::sync::Once::new();

pub struct RegularSizedTableDecorator;

impl RegularSizedTableDecorator {
    pub fn new() -> Self {
        RegularSizedTableDecorator
    }
}

impl ClassDecoratorTrait for RegularSizedTableDecorator {
    fn register(&self, munger: &CssMunger) {
        ONCE.call_once(|| munger.insert_rule(".rszcell { height: 20px; width: 20px }"));
    }

    fn cell_class(&self, _row: usize, _col: usize, _contents: Color) -> Vec<&'static str> {
        vec!["rszcell"]
    }
}
