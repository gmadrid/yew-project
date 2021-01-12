mod class_decorator;
mod cssmunger;
mod label_decorator;
mod purl_decorator;
mod style_decorator;

pub use class_decorator::ClassDecorator;
pub use label_decorator::LabelDecorator;
pub use purl_decorator::PurlDecorator;
pub use style_decorator::StyleDecorator;

pub use class_decorator::{
    BorderedCellDecorator, MergedBorderDecorator, MetagridDecorator, PrintableColorDecorator,
    RegularSizedTableDecorator, SmallSizedTableDecorator, ThickBorders,
};
pub use label_decorator::{EmptyLabels, FlatLabels, MergedFlatLabels, RoundLabels};
pub use purl_decorator::{EvenPurlDecorator, NoPurlDecorator};
pub use style_decorator::ColorDecorator;

pub use cssmunger::CssMunger;
