use crate::decorators::CssMunger;

pub trait PurlDecorator {
    fn register(&self, _munger: &CssMunger) {}

    fn add_purl(&self, _row: usize, _col: usize) -> bool {
        false
    }
}

#[derive(Default)]
pub struct NoPurlDecorator;

impl PurlDecorator for NoPurlDecorator {}

#[derive(Default)]
pub struct EvenPurlDecorator;

impl PurlDecorator for EvenPurlDecorator {
    fn add_purl(&self, _row: usize, col: usize) -> bool {
        col % 2 == 0
    }
}
