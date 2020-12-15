use crate::decorators::{ClassDecorator, CssMunger};
use grids::{Color, GridTrait};

static ONCE: std::sync::Once = std::sync::Once::new();

pub struct RegularSizedTableDecorator;

impl Default for RegularSizedTableDecorator {
    fn default() -> Self {
        RegularSizedTableDecorator
    }
}

impl ClassDecorator for RegularSizedTableDecorator {
    fn register(&self, munger: &CssMunger) {
        ONCE.call_once(|| munger.insert_rule(".rszcell { height: 20px; width: 20px }"));
    }

    fn cell_class(
        &self,
        _grid: &dyn GridTrait,
        _row: usize,
        _col: usize,
        _contents: Color,
    ) -> Vec<&'static str> {
        vec!["rszcell"]
    }

    fn label_class(&self, _grid: &dyn GridTrait, _row: usize) -> Vec<&'static str> {
        vec!["rszcell"]
    }
}
