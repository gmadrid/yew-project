use crate::biggrid::BigGrid;
use crate::gridtrait::GridTrait;
use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    front: BigGrid<bool>,
    back: BigGrid<bool>,

    value: Option<bool>,
    hover: Option<(GridId, usize, usize)>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GridId {
    Front,
    Back,
}

pub enum Msg {
    // Mouse events
    Down(GridId, usize, usize), // (id, row, col)
    Up,
    Enter(GridId, usize, usize), // (id, row, col)
    Exit,

    // User actions
    Clear(GridId),
}

impl App {
    fn grid_by_id(&self, id: GridId) -> &BigGrid<bool> {
        match id {
            GridId::Front => &self.front,
            GridId::Back => &self.back,
        }
    }

    pub fn grid_by_id_mut(&mut self, id: GridId) -> &mut BigGrid<bool> {
        match id {
            GridId::Front => &mut self.front,
            GridId::Back => &mut self.back,
        }
    }

    pub fn view_row(&self, grid_id: GridId, row: usize) -> Html {
        let grid = self.grid_by_id(grid_id);
        let num_cols = grid.num_cols();
        html! {
            <tr>
            { for (0..num_cols).map(|cn| self.view_cell(grid_id, row, cn)) }
            </tr>
        }
    }

    pub fn view_cell(&self, grid_id: GridId, row: usize, col: usize) -> Html {
        let grid = self.grid_by_id(grid_id);
        let value = grid.cell(row, col);
        let cls = if value { "on" } else { "off" };
        let mut classes = vec![cls];

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

    pub fn combined_view_row(&self, row: usize) -> Html {
        html! {
            <tr>
            { for (0..(self.front.num_cols() * 2)).map(|cn| self.combined_view_cell(row, cn)) }
            </tr>
        }
    }

    pub fn combined_view_cell(&self, row: usize, combined_col: usize) -> Html {
        let mut classes = vec![];
        let (grid_id, content) = if combined_col % 2 == 1 {
            (GridId::Front, "")
        } else {
            classes.push("purl");
            (GridId::Back, "•")
        };
        let grid = self.grid_by_id(grid_id);

        let real_col = combined_col / 2;

        let value = grid.cell(row, real_col);
        let cls = if value { "on" } else { "off" };
        classes.push(cls);

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

    fn grid_table(&self, title: &str, grid_id: GridId /* grid: &BigGrid<bool>*/) -> Html {
        let num_rows = self.grid_by_id(grid_id).num_rows();
        html! {
          <table class={"user-select-none"}>
            { for (0..num_rows).map(|rn| self.view_row(grid_id, rn)) }
          </table>
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            front: BigGrid::new(10, 10),
            back: BigGrid::new(10, 10),
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
          <>
            <nav class="navbar navbar-expand-md">
              <a style="color:black" class="navbar-brand">{"Two-color double-knitting pattern generator"}</a>
            </nav>
            <main class={"container"}>
              <div class={"row"}>
                <div class={"col"} style={"min-height: 2em"}>
                </div>
              </div>
              <div class={"row"}>
                <div class={"col"}>
                  <div class="card">
                    <h4 class="card-header">{"Front"}</h4>
                    <div class="card-body">
                      <h6 class="card-subtitle">{"Some explanation goes here"}</h6>
                      {self.grid_table("XXX", GridId::Front)}
                      <a href="#" class="btn btn-primary" onclick=click_front_callback>{"Clear"}</a>
                    </div>
                  </div>
                </div>

                <div class={"col"}>
                  <div class="card">
                  <h4 class="card-header">{"Back"}</h4>
                    <div class="card-body">
                      <h6 class="card-subtitle">{"Some explanation goes here"}</h6>
                      {self.grid_table("XXX", GridId::Back)}
                      <a href="#" class="btn btn-primary" onclick=click_back_callback>{"Clear"}</a>
                    </div>
                  </div>
                </div>
              </div>

              <div class={"row"}>
                <div class={"col"} style={"min-height: 2em"}>
                </div>
              </div>

              <div class={"row"}>
                <div class={"col"}>
                  <div class="card">
                    <h4 class="card-header">{"Pattern"}</h4>
                    <div class="card-body">
                      <h6 class="card-subtitle">{"Some explanation goes here"}</h6>
                      <table class={"user-select-none"}>
                        { for (0..(self.back.num_rows())).map(|rn| self.combined_view_row(rn)) }
                      </table>
                      <a href="#" class="card-link" >{"Export (unimplemented)"}</a>
                      <a href="#" class="card-link" >{"(...and whatever functions we want to add)"}</a>
                    </div>
                  </div>
                </div>
              </div>

              <div class={"row"}>
                <div class={"col"} style={"min-height: 2em"}>
                </div>
              </div>

              <div class="row">
                <div class="col">
                  <div class="card">
                    <div class="card-body">
                      <h3 class="card-title">
                        {"Stuff"}
                      </h3>
                      <ul>
                        <li>{"We can explore 'local persistence' where data is stored in the browser. Of course, this complicates the UI and implementation. And I've never done it before, so I have to figure out how to do it."}</li>
                        <li>{"What should the 'output' be?"}</li>
                      </ul>
                    </div>
                  </div>
                </div>
              </div>

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
