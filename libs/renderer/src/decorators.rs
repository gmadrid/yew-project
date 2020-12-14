mod cell_size;
mod color_decorator;
mod cssmunger;
mod printable;
mod traits;

pub use traits::ClassDecoratorTrait;
pub use traits::StyleDecorator;

pub use cell_size::{CellSizeDecorator, RegularSizedTableDecorator};
pub use color_decorator::ColorDecorator;
pub use cssmunger::CssMunger;
pub use printable::PrintableColorDecorator;
