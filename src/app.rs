use crate::biggrid::BigGrid;
use crate::bootstrap;
use crate::components::GridTable;
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
    Export,
}

impl App {
    fn grid_by_id(&self, id: GridId) -> &BigGrid<bool> {
        match id {
            GridId::Front => &self.front,
            GridId::Back => &self.back,
        }
    }

    fn grid_by_id_mut(&mut self, id: GridId) -> &mut BigGrid<bool> {
        match id {
            GridId::Front => &mut self.front,
            GridId::Back => &mut self.back,
        }
    }

    fn view_row(&self, grid_id: GridId, row: usize) -> Html {
        let grid = self.grid_by_id(grid_id);
        let num_cols = grid.num_cols();
        html! {
            <tr>
            { for (0..num_cols).map(|cn| self.view_cell(grid_id, row, cn)) }
            </tr>
        }
    }

    fn view_cell(&self, grid_id: GridId, row: usize, col: usize) -> Html {
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

    fn grid_table(&self, grid_id: GridId /* grid: &BigGrid<bool>*/) -> Html {
        let num_rows = self.grid_by_id(grid_id).num_rows();
        html! {
          <table class={"user-select-none"}>
            { for (0..num_rows).map(|rn| self.view_row(grid_id, rn)) }
          </table>
        }
    }

    fn export(&self) -> bool {
        let text = "George was here again";
        let filename = "george_too.txt";
        crate::download::download(text, filename);
        false
    }

    fn msg_down(&mut self, id: GridId, row: usize, col: usize) -> bool {
        let grid = self.grid_by_id_mut(id);
        let value = Some(!grid.cell(row, col));
        grid.toggle_cell(row, col);

        self.value = value;
        true
    }

    fn msg_enter(&mut self, id: GridId, row: usize, col: usize) -> bool {
        if let Some(value) = self.value {
            let grid = self.grid_by_id_mut(id);
            grid.set_cell(row, col, value);
            true
        } else {
            //self.hover = Some((id, row, col));
            false
        }
    }

    fn msg_exit(&self) -> bool {
        false
        //                 if self.hover.is_some() {
        //                     self.hover = None;
        //                     true
        //                 } else {
        //                     false
        //                 }
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
            front: BigGrid::new(8, 11),
            back: BigGrid::new(8, 11),
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
        }
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let click_front_callback = self.link.callback(|_| Msg::Clear(GridId::Front));
        let click_back_callback = self.link.callback(|_| Msg::Clear(GridId::Back));

        let export_callback = self.link.callback(|_| Msg::Export);

        html! {
          <>
            <nav class="navbar navbar-expand-md">
              <a style="color:black" class="navbar-brand">{"Two-color double-knitting pattern generator"}</a>
            </nav>

            <GridTable/>

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
                 html! {
                     <>
                       <table class={"user-select-none"}>
                         { for (0..(self.back.num_rows())).map(|rn| self.combined_view_row(rn)) }
                       </table>
                       <a href="#" class="btn btn-primary" onclick=export_callback>{"Test export"}</a>
                       <a href="#" class="card-link" >{"Export (unimplemented)"}</a>
                       <a href="#" class="card-link" >{"(...and whatever functions we want to add)"}</a>
                     </>
                 }
              )))}

              { bootstrap::spacer() }
              {
                bootstrap::row(bootstrap::col(
                  bootstrap::card("Notes", "",
                    bootstrap::ul(&[
                        "We can explore 'local persistence' where data is stored in the browser. Of course, this complicates the UI and implementation. And I've never done it before, so I have to figure out how to do it.",
                        "What should the 'output' be?",
                        "What about sizing? Is this specifically for the class, so the size should be fixed, or is it a general-purpose tool where we should allow resizing?"
                        ]))))
              }
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
