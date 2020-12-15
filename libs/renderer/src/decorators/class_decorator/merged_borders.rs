use crate::decorators::{ClassDecorator, CssMunger};
use grids::{Color, GridTrait};

static ONCE: std::sync::Once = std::sync::Once::new();

#[derive(Default)]
pub struct MergedBorderDecorator;

impl ClassDecorator for MergedBorderDecorator {
    fn register(&self, munger: &CssMunger) {
        ONCE.call_once(|| munger.insert_rule("td.mthick{border-left:3px solid black} "));
    }
    fn cell_class(
        &self,
        _grid: &dyn GridTrait,
        _row: usize,
        col: usize,
        _contents: Color,
    ) -> Vec<&'static str> {
        if col % 2 == 0 {
            vec!["mthick"]
        } else {
            vec![]
        }
    }
}
