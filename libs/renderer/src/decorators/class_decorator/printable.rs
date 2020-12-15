use super::ClassDecorator;
use crate::decorators::CssMunger;
use grids::{Color, GridTrait};

static ONCE: std::sync::Once = std::sync::Once::new();

#[derive(Default)]
pub struct PrintableColorDecorator;

impl ClassDecorator for PrintableColorDecorator {
    fn register(&self, munger: &CssMunger) {
        ONCE.call_once(|| {
            munger
                .insert_rule(".prtexact { color-adjust:exact; -webkit-print-color-adjust: exact }")
        });
    }
    fn cell_class(
        &self,
        _grid: &dyn GridTrait,
        _row: usize,
        _col: usize,
        _contents: Color,
    ) -> Vec<&'static str> {
        vec!["prtexact"]
    }
}
