use crate::decorators::{ClassDecoratorTrait, CssMunger};
use grids::Color;

static ONCE: std::sync::Once = std::sync::Once::new();

pub struct PrintableColorDecorator;

impl PrintableColorDecorator {
    pub fn new() -> Self {
        PrintableColorDecorator
    }
}

impl ClassDecoratorTrait for PrintableColorDecorator {
    fn register(&self, munger: &CssMunger) {
        ONCE.call_once(|| {
            let munger = CssMunger::new();
            munger
                .insert_rule(".prtexact { color-adjust:exact; -webkit-print-color-adjust: exact }")
        });
    }
    fn cell_class(&self, _row: usize, _col: usize, _contents: Color) -> Vec<&'static str> {
        vec!["prtexact"]
    }
}
