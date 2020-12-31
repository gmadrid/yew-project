use crate::TableRenderer;
use grids::{CellId, GridTrait};
use yew::prelude::*;

/*
  NOTES

  Want to install() it, and know that it will fire the correst Messages.
  Will have to check for messages and call correct thing in update().


*/

// enum Interactions {
//     MouseDown(CellId),
//     MouseEnter(CellId),
//     MouseExit(CellId),
//     MouseUp,
// }
//
pub trait Interactor<C: Component> {
    fn install<'a, GRID: GridTrait>(renderer: &mut TableRenderer<'a, GRID>) {
        renderer.set_interactions(
            Self::mousedown_callback(),
            Self::mouseup_callback(),
            Self::mouseenter_callback(),
            Self::mouseleave_callback(),
        );
    }

    fn mousedown_callback() -> Callback<CellId> {
        Callback::noop()
    }

    fn mouseup_callback() -> Callback<CellId> {
        Callback::noop()
    }

    fn mouseenter_callback() -> Callback<CellId> {
        Callback::noop()
    }

    fn mouseleave_callback() -> Callback<CellId> {
        Callback::noop()
    }
}

pub struct OneColorInteractor;

//impl Interactor for OneColorInteractor {}
