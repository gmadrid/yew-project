use crate::decorators::{ClassDecorator, CssMunger};
use grids::{Color, GridTrait};

static REGULAR_ONCE: std::sync::Once = std::sync::Once::new();
static SMALL_ONCE: std::sync::Once = std::sync::Once::new();

pub struct RegularSizedTableDecorator;

impl Default for RegularSizedTableDecorator {
    fn default() -> Self {
        RegularSizedTableDecorator
    }
}

impl ClassDecorator for RegularSizedTableDecorator {
    fn register(&self, munger: &CssMunger) {
        REGULAR_ONCE.call_once(|| munger.insert_rule(".rszcell { height: 20px; width: 20px }"));
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

pub struct SmallSizedTableDecorator;

impl Default for SmallSizedTableDecorator {
    fn default() -> Self {
        SmallSizedTableDecorator
    }
}

impl ClassDecorator for SmallSizedTableDecorator {
    // TODO: LOTS of repetition here. DRY!
    fn register(&self, munger: &CssMunger) {
        SMALL_ONCE.call_once(|| munger.insert_rule(".sszcell { height: 12px; width: 12px }"));
    }

    fn cell_class(
        &self,
        _grid: &dyn GridTrait,
        _row: usize,
        _col: usize,
        _contents: Color,
    ) -> Vec<&'static str> {
        vec!["sszcell"]
    }

    fn label_class(&self, _grid: &dyn GridTrait, _row: usize) -> Vec<&'static str> {
        vec!["sszcell"]
    }
}
