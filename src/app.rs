use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    front: Grid,
    back: Grid,

    value: Option<bool>,
    hover: Option<(GridId, u16, u16)>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GridId {
    Front,
    Back,
}

pub enum Msg {
    // Mouse events
    Down(GridId, u16, u16), // (id, row, col)
    Up,
    Enter(GridId, u16, u16), // (id, row, col)
    Exit,

    // User actions
    Clear(GridId),
}

/*

Two fields with "width" "height"

Two grids: same size. Click to toggle gray or white.
Dynamic grid: colors with purl dots.

Down: record the opposite of the state of the current cell.
Enter: set the new cell to the current value.
Up: clear the current value.

 */

pub struct Grid {
    id: GridId,
    // cells will contain (height x width) cells in row-major order.
    cells: Vec<bool>,
    height: u16,
    width: u16,
}

impl Grid {
    pub fn new(id: GridId, height: u16, width: u16) -> Grid {
        Grid {
            id,
            cells: vec![false; (height * width) as usize],
            height,
            width,
        }
    }

    pub fn num_rows(&self) -> u16 {
        self.height
    }

    pub fn num_cols(&self) -> u16 {
        self.width
    }

    fn coords_to_index(&self, row: u16, col: u16) -> usize {
        (row * self.width + col) as usize
    }

    pub fn cell(&self, row: u16, col: u16) -> bool {
        self.cells[self.coords_to_index(row, col)]
    }

    pub fn set_cell(&mut self, row: u16, col: u16, value: bool) {
        let index = self.coords_to_index(row, col);
        self.cells[index] = value;
    }

    pub fn toggle_cell(&mut self, row: u16, col: u16) {
        let old_value = self.cell(row, col);
        self.set_cell(row, col, !old_value);
    }

    pub fn rows(&self) -> std::ops::Range<u16> {
        0..self.height
    }

    pub fn cols(&self) -> std::ops::Range<u16> {
        0..self.width
    }

    pub fn clear(&mut self) {
        let old_size = self.cells.len();
        self.cells.clear();
        self.cells.resize_with(old_size, Default::default);
    }
}

impl App {
    pub fn grid_by_id_mut(&mut self, id: GridId) -> &mut Grid {
        match id {
            GridId::Front => &mut self.front,
            GridId::Back => &mut self.back,
        }
    }

    pub fn view_row(&self, grid: &Grid, row: u16) -> Html {
        html! {
            <tr>
            { for grid.cols().map(|cn| self.view_cell(grid, row, cn)) }
            </tr>
        }
    }

    pub fn view_cell(&self, grid: &Grid, row: u16, col: u16) -> Html {
        let value = grid.cell(row, col);
        let cls = if value { "on" } else { "off" };
        let mut classes = vec![cls];
        let grid_id = grid.id;

        let down_callback = self.link.callback(move |_| Msg::Down(grid_id, row, col));
        let enter_callback = self.link.callback(move |_| Msg::Enter(grid_id, row, col));
        let exit_callback = self.link.callback(|_| Msg::Exit);
        let up_callback = self.link.callback(|_| Msg::Up);

        if Some((grid_id, row, col)) == self.hover {
            classes.push("hover");
        }

        html! {
            <td class=classes
         onmousedown=down_callback
         onmouseenter=enter_callback
         onmouseleave=exit_callback
         onmouseup=up_callback
         > </td>
        }
    }

    pub fn combined_view_row(&self, row: u16) -> Html {
        html! {
            <tr>
            { for (0..(self.front.num_cols() * 2)).map(|cn| self.combined_view_cell(row, cn)) }
            </tr>
        }
    }

    pub fn combined_view_cell(&self, row: u16, col: u16) -> Html {
        let grid = if col % 2 == 1 {
            &self.front
        } else {
            &self.back
        };
        let value = grid.cell(row, col / 2);
        let mut classes = vec![];
        let cls = if value { "on" } else { "off" };
        classes.push(cls);

        if col % 2 == 0 {
            classes.push("purl");
        }
        let content = if col % 2 == 0 { "•" } else { "" };

        let grid_id = grid.id;
        let down_callback = self
            .link
            .callback(move |_| Msg::Down(grid_id, row, col / 2));
        let up_callback = self.link.callback(move |_| Msg::Up);
        let enter_callback = self
            .link
            .callback(move |_| Msg::Enter(grid_id, row, col / 2));
        let exit_callback = self.link.callback(|_| Msg::Exit);

        if Some((grid_id, row, col / 2)) == self.hover {
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
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            front: Grid::new(GridId::Front, 10, 10),
            back: Grid::new(GridId::Back, 10, 10),
            value: None,
            hover: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Down(id, row, col) => {
                let grid = self.grid_by_id_mut(id);
                let value = Some(!grid.cell(row, col));
                grid.toggle_cell(row, col);

                self.value = value;
                true
            }
            Msg::Enter(id, row, col) => {
                if let Some(value) = self.value {
                    let grid = self.grid_by_id_mut(id);
                    grid.set_cell(row, col, value);
                }

                self.hover = Some((id, row, col));

                true
            }
            Msg::Exit => {
                if self.hover.is_some() {
                    self.hover = None;
                    true
                } else {
                    false
                }
            }
            Msg::Up => {
                self.value = None;
                false
            }
            Msg::Clear(grid_id) => {
                let grid = self.grid_by_id_mut(grid_id);
                grid.clear();
                true
            }
        }
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let click_front_callback = self.link.callback(move |_| Msg::Clear(GridId::Front));
        let click_back_callback = self.link.callback(move |_| Msg::Clear(GridId::Back));

        html! {
            <div class={"container"}>
              <div class={"row"}>
                <div class={"col align-self-center"}>
                  <h4>{"Front"}</h4>
                  <table class={"user-select-none"}>
                    { for self.front.rows().map(|rn| self.view_row(&self.front, rn)) }
                  </table>
                </div>
                <div class={"col"}>
                  <button type="button" class={"btn btn-primary"} onclick=click_front_callback>{"Clear Front"}</button>
                </div>

                <div class={"col align-self-center"}>
                  <h4>{"Back"}</h4>
                  <table class={"user-select-none"}>
                    { for self.back.rows().map(|rn| self.view_row(&self.back, rn)) }
                  </table>
                </div>
                <div class={"col"}>
                  <button type="button" class={"btn btn-primary"} onclick=click_back_callback>{"Clear Back"}</button>
                </div>
              </div>

              <div class={"row"}>
                <div class={"col align-self-center"}>
                  <h4>{"Pattern"}</h4>
                  <table class={"user-select-none"}>
                    { for (0..(self.back.num_rows())).map(|rn| self.combined_view_row(rn)) }
                  </table>
                </div>
              </div>
            </div>
        }
    }
}
