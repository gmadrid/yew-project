mod class_decorator;
mod cssmunger;
mod style_decorator;

pub use class_decorator::ClassDecorator;
pub use style_decorator::StyleDecorator;

pub use class_decorator::{
    BorderedCellDecorator, PrintableColorDecorator, RegularSizedTableDecorator, ThickBorders,
};
pub use style_decorator::ColorDecorator;

pub use cssmunger::CssMunger;
