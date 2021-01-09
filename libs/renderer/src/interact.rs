use crate::TableRenderer;
use grids::{CellId, GridTrait};
use std::convert::TryInto;
use yew::prelude::*;

mod one_color;
pub use one_color::OneColorInteractor;

mod colored;
pub use colored::ColoredInteractor;

/*
  NOTES

  Want to install() it, and know that it will fire the correst Messages.
  Will have to check for messages and call correct thing in update().


*/

#[derive(Clone, Copy)]
pub enum Interactions {
    MouseDown(CellId),
    MouseEnter(CellId),
    MouseExit(CellId),
    MouseUp(CellId),
}

impl Interactions {
    pub fn cell_id(&self) -> CellId {
        match self {
            Interactions::MouseDown(cell_id) => *cell_id,
            Interactions::MouseEnter(cell_id) => *cell_id,
            Interactions::MouseExit(cell_id) => *cell_id,
            Interactions::MouseUp(cell_id) => *cell_id,
        }
    }
}

pub trait Interactor {
    fn update(&mut self, grid: &mut dyn GridTrait, msg: impl TryInto<Interactions>)
        -> Option<bool>;

    fn install<'a, COMP>(&self, link: &ComponentLink<COMP>, renderer: &mut TableRenderer<'a>)
    where
        COMP: Component,
        COMP::Message: From<Interactions>,
    {
        renderer.set_interactions(
            self.mousedown_callback(link.clone()),
            self.mouseup_callback(link.clone()),
            self.mouseenter_callback(link.clone()),
            self.mouseleave_callback(link.clone()),
        );
    }

    fn mousedown_callback<COMP>(&self, link: ComponentLink<COMP>) -> Callback<CellId>
    where
        COMP: Component,
        COMP::Message: From<Interactions>,
    {
        Callback::from(move |cell_id: CellId| {
            link.send_message(Interactions::MouseDown(cell_id));
        })
    }

    fn mouseup_callback<COMP>(&self, link: ComponentLink<COMP>) -> Callback<CellId>
    where
        COMP: Component,
        COMP::Message: From<Interactions>,
    {
        Callback::from(move |cell_id: CellId| {
            link.send_message(Interactions::MouseUp(cell_id));
        })
    }

    fn mouseenter_callback<COMP>(&self, link: ComponentLink<COMP>) -> Callback<CellId>
    where
        COMP: Component,
        COMP::Message: From<Interactions>,
    {
        Callback::from(move |cell_id: CellId| {
            link.send_message(Interactions::MouseEnter(cell_id));
        })
    }

    fn mouseleave_callback<COMP>(&self, link: ComponentLink<COMP>) -> Callback<CellId>
    where
        COMP: Component,
        COMP::Message: From<Interactions>,
    {
        Callback::from(move |cell_id: CellId| {
            link.send_message(Interactions::MouseExit(cell_id));
        })
    }
}
