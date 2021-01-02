use grids::{CellId, GridId, GridTrait, MergedGrid, SimpleGrid};
use renderer::decorators::{
    EvenPurlDecorator, FlatLabels, MergedBorderDecorator, MergedFlatLabels, ThickBorders,
};
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
    fn grid_with_id(&self, id: GridId) -> &dyn GridTrait {
        match id {
            GridId::LayerOne => &self.stored.front,
            GridId::LayerTwo => &self.stored.back,
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

    fn render_pattern_table(&self) -> Html {
        let combined_grid =
            MergedGrid::new(GridId::Combined, &self.stored.front, &self.stored.back);
        let mut renderer = TableRenderer::regular_renderer(&combined_grid);
        renderer.add_class_decorator(MergedBorderDecorator::default());
        renderer.add_class_decorator(ThickBorders::thick_horizontal());
        renderer.set_label_decorator(MergedFlatLabels::starting_at(3, 3));
        renderer.set_purl_decorator(EvenPurlDecorator::default());
        self.interact.install(&self.link, &mut renderer);

        renderer.render()
    }

    fn render_reference_card(&self) -> Html {
        bootstrap::empty()
    }

    fn render_bottom_row(&self) -> Html {
        let printable_callback = self.link.callback(|_| Message::TogglePrintable);
        let printable_text = if self.printable_page {
            "Return to input page"
        } else {
            "Show printable pattern"
        };

        bootstrap::row(bootstrap::col(bootstrap::concat(
            bootstrap::card(
                html! {
                <>
                  <span>{"Chart 2"}</span>

                  <a class="d-print-none"
                      onclick=printable_callback
                      style="float:right" href="#"><small>{printable_text}</small></a>
                </>},
                "Follow this chart within the green box in Chart 1 on Page 7.",
                self.render_pattern_table(),
            ),
            self.render_reference_card(),
        )))
    }

    fn render_main_container(&self) -> Html {
        let contents = html! {
          <>
            {self.render_input_grids()}
            {bootstrap::spacer()}
            {self.render_bottom_row()}
          </>
        };

        bootstrap::main_container(contents)
    }

    fn render_grid_table(&self, grid_id: GridId) -> Html {
        let grid = self.grid_with_id(grid_id);
        let mut renderer = TableRenderer::regular_renderer(grid);
        renderer.add_class_decorator(ThickBorders::default());
        renderer.set_label_decorator(FlatLabels::starting_at(3, 3));
        self.interact.install(&self.link, &mut renderer);

        html! {{renderer.render()}}
    }

    fn render_grid_card(&self, title: &str, subtitle: &str, grid_id: GridId) -> Html {
        let clear_callback = self.link.callback(move |_| Message::Clear(grid_id));
        bootstrap::card(
            title,
            subtitle,
            bootstrap::concat(
                self.render_grid_table(grid_id),
                html! {<a href="#" class="btn btn-primary" onclick=clear_callback>{"Clear"}</a>},
            ),
        )
    }

    fn render_input_grids(&self) -> Html {
        if self.printable_page {
            return bootstrap::empty();
        }

        html! {
          <>
            {bootstrap::row(
                bootstrap::concat(
                    bootstrap::col(self.render_grid_card("Layer 1", "Transcribe a chart from Page 5 here.", GridId::LayerOne)),
                    bootstrap::col(self.render_grid_card("Layer 2", "Transcribe a chart from Page 6 here.", GridId::LayerTwo)),
            ))}
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
        match msg {
            Message::TogglePrintable => self.msg_toggle_printable(),
            Message::Clear(grid_id) => self.msg_clear(grid_id),

            m @ Message::Up(_)
            | m @ Message::Down(_)
            | m @ Message::Enter(_)
            | m @ Message::Leave(_) => {
                let (grid, interact) = self.grid_with_interact(m.cell_id().grid_id);
                if let Some(should_render) = interact.update(grid, &m) {
                    return should_render;
                } else {
                    return false;
                }
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <>
            {self.render_nav()}
            {self.render_main_container()}
          </>
        }
    }
}
