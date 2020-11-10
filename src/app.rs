use crate::bootstrap;
use crate::gridtrait::GridTrait;
use crate::simplegrid::SimpleGrid;
use crate::tablerender::{InputRenderer, PatternRenderer};
use yew::prelude::*;

const GRID_HEIGHT: usize = 15;
const GRID_WIDTH: usize = 15;

const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");

pub struct App {
    link: ComponentLink<Self>,
    front: SimpleGrid<bool>,
    back: SimpleGrid<bool>,

    value: Option<bool>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GridId {
    Front,
    Back,
}

/*
impl GridId {
    pub fn table_id(&self) -> &str {
        match self {
            GridId::Front => "front",
            GridId::Back => "back",
        }
    }
}
*/

pub enum Msg {
    // Mouse events
    Down(GridId, usize, usize), // (id, row, col)
    Up,
    Enter(GridId, usize, usize), // (id, row, col)
    Exit,

    // State changes
    //    SetCell(GridId, usize, usize, bool), // (id, row, col, value)
    //    NoOp,

    // User actions
    Clear(GridId),
    //    Export,
}

impl App {
    fn grid_by_id(&self, id: GridId) -> &SimpleGrid<bool> {
        match id {
            GridId::Front => &self.front,
            GridId::Back => &self.back,
        }
    }

    fn grid_by_id_mut(&mut self, id: GridId) -> &mut SimpleGrid<bool> {
        match id {
            GridId::Front => &mut self.front,
            GridId::Back => &mut self.back,
        }
    }

    fn grid_table(&self, grid_id: GridId) -> Html {
        let grid = self.grid_by_id(grid_id);
        InputRenderer::<SimpleGrid<bool>>::render_table(&self.link, grid_id, grid)
    }

    fn pattern_table(&self) -> Html {
        PatternRenderer::<SimpleGrid<bool>, SimpleGrid<bool>>::render_table(
            &self.link,
            GridId::Front,
            &self.front,
            GridId::Back,
            &self.back,
        )
    }

    fn msg_down(&mut self, id: GridId, row: usize, col: usize) -> bool {
        let grid = self.grid_by_id_mut(id);
        let value = !grid.cell(row, col);
        grid.toggle_cell(row, col);
        self.value = Some(value);

        true
    }

    fn msg_enter(&mut self, id: GridId, row: usize, col: usize) -> bool {
        if let Some(value) = self.value {
            let grid = self.grid_by_id_mut(id);
            grid.set_cell(row, col, value);
            true
        } else {
            false
        }
    }

    fn msg_exit(&self) -> bool {
        false
    }

    fn msg_up(&mut self) -> bool {
        self.value = None;
        false
    }

    fn msg_clear(&mut self, grid_id: GridId) -> bool {
        let grid = self.grid_by_id_mut(grid_id);
        grid.clear();
        true
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            front: SimpleGrid::new(GRID_HEIGHT, GRID_WIDTH),
            back: SimpleGrid::new(GRID_HEIGHT, GRID_WIDTH),
            value: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Down(id, row, col) => self.msg_down(id, row, col),
            Msg::Enter(id, row, col) => self.msg_enter(id, row, col),
            Msg::Exit => self.msg_exit(),
            Msg::Up => self.msg_up(),

            Msg::Clear(grid_id) => self.msg_clear(grid_id),
            // Msg::Export => self.export(),
            //
            // Msg::NoOp => false,
            // Msg::SetCell(id, row, col, value) => self.msg_set_cell(id, row, col, value),
        }
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let click_front_callback = self.link.callback(|_| Msg::Clear(GridId::Front));
        let click_back_callback = self.link.callback(|_| Msg::Clear(GridId::Back));

        html! {
          <>
            <nav class="navbar navbar-expand-md">
              <a style="color:black" class="navbar-brand">{"Two-color double-knitting pattern generator"}</a>
            </nav>

            <main class="main container">
              { bootstrap::spacer() }
              { bootstrap::row(bootstrap::concat(
                  bootstrap::col(
                      bootstrap::card("Front", "Lorem ipsum",
                        bootstrap::concat(
                            self.grid_table(GridId::Front),
                            html!{<a href="#" class="btn btn-primary" onclick=click_front_callback>{"Clear"}</a>}))),
                  bootstrap::col(
                      bootstrap::card("Back", "Some explanation stuff",
                        bootstrap::concat(
                            self.grid_table(GridId::Back),
                            html!{<a href="#" class="btn btn-primary" onclick=click_back_callback>{"Clear"}</a>}))),
              ))}

              { bootstrap::spacer() }

              { bootstrap::row(bootstrap::col(bootstrap::card("Pattern", "Some explanation of the pattern",
              self.pattern_table()
              )))}
            </main>
            <footer class="footer">
              <div class="container">
                 <small>{"Version "}{VERSION_NUMBER}</small>
                 <a href="https://double-knitting.com/" class="text-muted">{"Fallingblox Designs"}</a>
              </div>
            </footer>
          </>
        }
    }
}
