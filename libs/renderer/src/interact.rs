use crate::TableRenderer;
use grids::{CellId, GridTrait};
use std::convert::TryInto;
use yew::prelude::*;

mod one_color;

pub use one_color::OneColorInteractor;

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
    MouseUp,
}

impl Interactions {
    fn cell_id(&self) -> CellId {
        match self {
            Interactions::MouseDown(cell_id) => *cell_id,
            Interactions::MouseEnter(cell_id) => *cell_id,
            Interactions::MouseExit(cell_id) => *cell_id,
            Interactions::MouseUp => panic!("Cannot get a CellId from a MouseUp."),
        }
    }
}

pub trait Interactor<COMP>
where
    COMP: Component,
    COMP::Message: From<Interactions>,
{
    type State;

    fn update(&mut self, state: &mut Self::State, msg: impl TryInto<Interactions>) -> Option<bool>;

    /*
        need to install callbacks.
        - these callbacks will possibly send no message, but need a way to convert
          from Interactions -> Component::Messages
        - Then, in update(), we want to just call handle which will return
          Option<bool>. None indicates that it wasn't handled. Some(rendered: bool) is
          returned if it could handle it.

        So, for example:
          - mousedown happens
          - calls the Interactor installed callback
          - that callback will either do nothing, or link.send_message(Component::Message).
            - that message will be converted with Interactions::into::<Component::Message>().

          - then update will be called.
            - update will call interactor.handle() which will attempt
              Interactions::try_from(Component::Message)
            - if it works, it will try to handle the message.
    */
    fn install<'a, GRID: GridTrait>(
        &self,
        link: &ComponentLink<COMP>,
        renderer: &mut TableRenderer<'a, GRID>,
    ) {
        renderer.set_interactions(
            self.mousedown_callback(link.clone()),
            self.mouseup_callback(link.clone()),
            self.mouseenter_callback(link.clone()),
            self.mouseleave_callback(link.clone()),
        );
    }

    fn mousedown_callback(&self, link: ComponentLink<COMP>) -> Callback<CellId> {
        Callback::from(move |cell_id: CellId| {
            link.send_message(Interactions::MouseDown(cell_id));
        })
    }

    fn mouseup_callback(&self, link: ComponentLink<COMP>) -> Callback<CellId> {
        Callback::from(move |_: CellId| {
            link.send_message(Interactions::MouseUp);
        })
    }

    fn mouseenter_callback(&self, link: ComponentLink<COMP>) -> Callback<CellId> {
        Callback::from(move |cell_id: CellId| {
            link.send_message(Interactions::MouseEnter(cell_id));
        })
    }

    fn mouseleave_callback(&self, link: ComponentLink<COMP>) -> Callback<CellId> {
        Callback::from(move |cell_id: CellId| {
            link.send_message(Interactions::MouseExit(cell_id));
        })
    }
}
