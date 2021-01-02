use crate::decorators::{ClassDecorator, CssMunger};
use grids::{Color, GridTrait};

static REGULAR_ONCE: std::sync::Once = std::sync::Once::new();
static SMALL_ONCE: std::sync::Once = std::sync::Once::new();

#[derive(Default)]
pub struct RegularSizedTableDecorator;

impl ClassDecorator for RegularSizedTableDecorator {
    fn register(&self, munger: &CssMunger) {
        REGULAR_ONCE.call_once(|| {
            munger.insert_rule(
                ".rszcell { height: 20px; width: 20px; min-height: 20px; max-height: 20px; min-width: 20px; max-width: 20px }",
            )
        });
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

#[derive(Default)]
pub struct SmallSizedTableDecorator;

impl ClassDecorator for SmallSizedTableDecorator {
    // TODO: LOTS of repetition here. DRY!
    fn register(&self, munger: &CssMunger) {
        SMALL_ONCE.call_once(||
            munger.insert_rule(
                ".sszcell { height: 15px; width: 15px; min-height: 15px; max-height: 15px; min-width: 15px; max-width: 15px }"));
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
