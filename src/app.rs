use crate::bootstrap;
use crate::gridtrait::GridTrait;
use crate::simplegrid::SimpleGrid;
use crate::tablerender::{InputRenderer, PatternRenderer};
use yew::prelude::*;

const GRID_HEIGHT: usize = 15;
const GRID_WIDTH: usize = 15;

pub struct App {
    link: ComponentLink<Self>,
    front: SimpleGrid<bool>,
    back: SimpleGrid<bool>,

    value: Option<bool>,
    hover: Option<(GridId, usize, usize)>,
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
    SetCell(GridId, usize, usize, bool), // (id, row, col, value)
    NoOp,

    // User actions
    Clear(GridId),
    Export,
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

    fn combined_view_row(&self, row: usize) -> Html {
        html! {
            <tr>
            { for (0..(self.front.num_cols() * 2)).map(|cn| self.combined_view_cell(row, cn)) }
            </tr>
        }
    }

    fn combined_view_cell(&self, row: usize, combined_col: usize) -> Html {
        let mut classes = vec![];
        let (grid_id, content) = if combined_col % 2 == 1 {
            (GridId::Front, no_dot())
        } else {
            classes.push("purl");
            (GridId::Back, dot())
        };
        let grid = self.grid_by_id(grid_id);

        let real_col = combined_col / 2;

        let value = grid.cell(row, real_col);
        let cls = if value { "on" } else { "off" };
        classes.push(cls);

        if (grid.num_rows() - row) % 5 == 1 {
            classes.push("fiver")
        }

        let down_callback = self
            .link
            .callback(move |_| Msg::Down(grid_id, row, real_col));
        let up_callback = self.link.callback(move |_| Msg::Up);
        let enter_callback = self
            .link
            .callback(move |_| Msg::Enter(grid_id, row, real_col));
        let exit_callback = self.link.callback(|_| Msg::Exit);

        if Some((grid_id, row, real_col)) == self.hover {
            classes.push("hover");
        }

        html! {
            <td class=classes
             onmousedown=down_callback
             onmouseenter=enter_callback
             onmouseup=up_callback
             onmouseleave=exit_callback
             >{content}</td>
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

    fn export(&self) -> bool {
        let text = "George was here again";
        let filename = "george_too.txt";
        crate::download::download(text, filename);
        false
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

    fn msg_set_cell(&mut self, grid_id: GridId, row: usize, col: usize, value: bool) -> bool {
        let grid = self.grid_by_id_mut(grid_id);
        grid.set_cell(row, col, value);
        true
    }
}

fn no_dot() -> Html {
    html! {
        <svg width="1em" height="1em" viewBox="0 0 15 15" class="bi bi-circle-fill" fill="black" xmlns="http://www.w3.org/2000/svg">
        </svg>
    }
}

fn dot() -> Html {
    html! {
        <svg width="1em" height="1em" viewBox="0 0 15 15" class="bi bi-circle-fill" fill="black" xmlns="http://www.w3.org/2000/svg">
          <circle cx="7.5" cy="7.5" r="4"/>
        </svg>
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
            hover: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Down(id, row, col) => self.msg_down(id, row, col),
            Msg::Enter(id, row, col) => self.msg_enter(id, row, col),
            Msg::Exit => self.msg_exit(),
            Msg::Up => self.msg_up(),

            Msg::Clear(grid_id) => self.msg_clear(grid_id),
            Msg::Export => self.export(),

            Msg::NoOp => false,
            Msg::SetCell(id, row, col, value) => self.msg_set_cell(id, row, col, value),
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

            // {grid_play(&self.link)}

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
                <a href="https://double-knitting.com/" class="text-muted">{"Fallingblox Designs"}</a>
              </div>
            </footer>
          </>
        }
    }
}
