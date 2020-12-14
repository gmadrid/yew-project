use crate::decorators::CssMunger;
use grids::Color;

pub trait ClassDecoratorTrait {
    fn register(&self, munger: &CssMunger) {}
    fn cell_class(&self, _row: usize, _col: usize, _contents: Color) -> Vec<&'static str> {
        vec![]
    }
}
