use grids::{CellId, GridId, GridTrait, SimpleGrid};
use renderer::interact::{Interactions, Interactor, OneColorInteractor};
use renderer::TableRenderer;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use yew::prelude::*;

const GRID_HEIGHT: usize = 15;
const GRID_WIDTH: usize = 15;

const STORAGE_KEY: &str = "SAVED_CHART";
const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize)]
struct Stored {
    front: SimpleGrid,
    back: SimpleGrid,
}

impl Default for Stored {
    fn default() -> Self {
        Stored {
            front: SimpleGrid::new(GridId::LayerOne, GRID_HEIGHT, GRID_WIDTH),
            back: SimpleGrid::new(GridId::LayerTwo, GRID_HEIGHT, GRID_WIDTH),
        }
    }
}

pub struct TwoColorApp {
    link: ComponentLink<Self>,
    interact: OneColorInteractor,
    stored: Stored,
    printable_page: bool,
}

#[derive(Debug)]
pub enum Message {
    Down(CellId),
    Up(CellId),
    Enter(CellId),
    Leave(CellId),

    // User Interactions
    TogglePrintable,
    Clear(GridId),
}

impl Message {
    fn cell_id(&self) -> CellId {
        match self {
            Message::Down(cell_id) => *cell_id,
            Message::Enter(cell_id) => *cell_id,
            Message::Leave(cell_id) => *cell_id,
            Message::Up(cell_id) => *cell_id,
            _ => panic!("CellId unavailable for Message: {:?}", self),
        }
    }
}

impl From<Interactions> for Message {
    fn from(i: Interactions) -> Message {
        match i {
            Interactions::MouseDown(cell_id) => Message::Down(cell_id),
            Interactions::MouseUp(cell_id) => Message::Up(cell_id),
            Interactions::MouseEnter(cell_id) => Message::Enter(cell_id),
            Interactions::MouseExit(cell_id) => Message::Leave(cell_id),
        }
    }
}

impl TryFrom<&Message> for Interactions {
    type Error = ();

    fn try_from(msg: &Message) -> Result<Self, Self::Error> {
        match msg {
            Message::Down(cell_id) => Ok(Interactions::MouseDown(*cell_id)),
            Message::Up(cell_id) => Ok(Interactions::MouseUp(*cell_id)),
            Message::Enter(cell_id) => Ok(Interactions::MouseEnter(*cell_id)),
            Message::Leave(cell_id) => Ok(Interactions::MouseExit(*cell_id)),
            _ => Err(()),
        }
    }
}

impl TwoColorApp {
    fn grid_with_id(&mut self, id: GridId) -> &dyn GridTrait {
        match id {
            GridId::LayerOne => &mut self.stored.front,
            GridId::LayerTwo => &mut self.stored.back,
            _ => panic!("Bad mapping for grid id: {:?}", id),
        }
    }

    // TODO: can we get rid of this uglyness?
    fn grid_with_interact(&mut self, id: GridId) -> (&mut dyn GridTrait, &mut OneColorInteractor) {
        (
            match id {
                GridId::LayerOne => &mut self.stored.front,
                GridId::LayerTwo => &mut self.stored.back,
                _ => panic!("Bad mapping for grid id: {:?}", id),
            },
            &mut self.interact,
        )
    }

    fn msg_toggle_printable(&mut self) -> bool {
        self.printable_page = !self.printable_page;
        true
    }

    fn msg_clear(&mut self, grid_id: GridId) -> bool {
        let (grid, _) = self.grid_with_interact(grid_id);
        grid.clear();
        true
    }

    fn render_nav(&self) -> Html {
        if self.printable_page {
            return bootstrap::empty();
        }
        html! {
          <>
            <nav class="navbar navbar-expand-md">
              <a style="color:black" class="navbar-brand">{"Two-pattern double-knitting chart generator"}</a>
              <small>{"This tool can be used to plan out a two-pattern chart during Alasdair Post-Quinn's workshop, \"Two-pattern Double-knitting\". The handout for that workshop will further explain what to do."}</small>
            </nav>
          </>
        }
    }

    fn render_main_container(&self) -> Html {
        let contents = html! {
          <>
            {self.render_input_grids()}
          </>
        };

        bootstrap::main_container(contents)
    }

    fn render_grid_table(&self, grid_id: GridId) -> Html {
        let grid = self.grid_with_id(grid_id);
        let renderer = TableRenderer::regular_renderer(grid);

        html! {{renderer.render()}}
    }

    fn render_input_grids(&self) -> Html {
        if self.printable_page {
            return bootstrap::empty();
        }

        html! {
          <>
            {bootstrap::spacer()}

            {bootstrap::row(bootstrap::concat(
                bootstrap::col(bootstrap::card("Layer 1", "Transcribe a chart from Page 5 here",
                bootstrap::concat(
                    self.render_grid_table(GridId::LayerOne),
                    bootstrap::empty()  // turn this into the clear button
                )
            )),
                bootstrap::col("CARD2"),
            ))}

            {bootstrap::spacer()}
          </>
        }
    }
}

impl Component for TwoColorApp {
    type Properties = ();
    type Message = Message;

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TwoColorApp {
            link,
            interact: OneColorInteractor::new(),
            stored: Stored::default(),
            printable_page: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let (grid, interact) = self.grid_with_interact(msg.cell_id().grid_id);
        if let Some(should_render) = interact.update(grid, &msg) {
            return should_render;
        }

        match msg {
            Message::TogglePrintable => self.msg_toggle_printable(),
            Message::Clear(grid_id) => self.msg_clear(grid_id),

            // Tnese were handled by the interactor.
            // TODO: maybe find a more elegant way to do this
            Message::Up(_) | Message::Down(_) | Message::Enter(_) | Message::Leave(_) => {
                panic!("Should be handled by interact.");
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        //        let printable_callback = self.link.callback(|_| Message::TogglePrintable);
        let printable_text = if self.printable_page {
            "Return to input page"
        } else {
            "Show printable pattern"
        };

        html! {
          <>
            {self.render_nav()}
            {self.render_main_container()}
          </>
        }
    }
}
