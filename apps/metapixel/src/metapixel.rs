use bootstrap::main_container;
use components::{Compass, CompassDirection, Input};
use grids::{
    shift_cols, shift_rows, BigGrid, CellId, Color, GridId, GridTrait, MetaGrid, ShiftDirection,
};
use renderer::decorators::{
    BorderedCellDecorator, ColorDecorator, CssMunger, MetagridDecorator, PrintableColorDecorator,
    RegularSizedTableDecorator,
};
use renderer::interact::{ColoredInteractor, Interactions, Interactor};
use renderer::TableRenderer;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use yew::format::Json;
use yew::prelude::*;
use yew::services::storage::Area;
use yew::services::StorageService;

static NAV_ONCE: std::sync::Once = std::sync::Once::new();
static CONTROLS_ONCE: std::sync::Once = std::sync::Once::new();
static PALETTE_ONCE: std::sync::Once = std::sync::Once::new();
static FOOTER_ONCE: std::sync::Once = std::sync::Once::new();

const STORAGE_KEY: &str = "SAVED_META_CHART";
const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize)]
struct Stored {
    base_grid: BigGrid,
    row_grid_cols: Vec<u8>,
    col_grid_cols: Vec<u8>,
}

pub struct MetapixelApp {
    link: ComponentLink<Self>,
    interact: ColoredInteractor,
    stored: Stored,
}

#[derive(Debug)]
pub enum Message {
    Down(CellId),
    Up(CellId),
    Enter(CellId),
    Leave(CellId),

    MetaXShift(CompassDirection),
    MetaYShift(CompassDirection),
    BaseShift(CompassDirection),
    MetagridShift(CompassDirection),
    SelectColor(Color),

    NewRowVec(Vec<u8>),
    NewColVec(Vec<u8>),

    Clear,
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

impl MetapixelApp {
    // TODO: can we get rid of this uglyness?
    fn grid_with_interact(&mut self, id: GridId) -> (&mut dyn GridTrait, &mut ColoredInteractor) {
        if id != GridId::Main {
            panic!(
                "Only GridId::Main is defined for the metapixel app. Received request for {:?}",
                id
            );
        }

        (&mut self.stored.base_grid, &mut self.interact)
    }

    fn render_nav(&self) -> Html {
        NAV_ONCE.call_once(|| {
            let munger = CssMunger::new();
            munger.insert_rule("body{background:lightgray}");
            munger.insert_rule("nav{background:darkgray}");
        });
        html! {
          <>
            <nav class="navbar navbar-expand-md">
              <a style="color:black" class="navbar-brand">{"Meta-pixel chart generator"}</a>
            </nav>
          </>
        }
    }

    fn render_footer(&self) -> Html {
        FOOTER_ONCE.call_once(|| {
            let munger = CssMunger::new();
            munger.insert_rule(".footer{position:fixed;bottom:0;width:100%;height:3em;line-height:3em;background:darkgray;text-align:right}");
            // munger.insert_rule(".footer small{float:left}");
        });
        html! {
          <footer class="footer">
            <div class="container">
              <small class="float-left">{"Version "}{VERSION_NUMBER}</small>
              <a href="https://double-knitting.com/" class="text-muted">{"Fallingblox Designs"}</a>
            </div>
          </footer>
        }
    }

    fn render_palette(&self) -> Html {
        PALETTE_ONCE.call_once(|| {
            let munger = CssMunger::new();
            munger.insert_rule("#palettetable{table-layout:fixed}");
            munger.insert_rule("#palettetable td{height:40px;width:40px;border:2px solid black}");
            munger.insert_rule("#palettetable td.selected{border-width:5px");
        });
        html! {
          <>
            <table id="palettetable">
              <tr>
                {self.render_palette_cell(Color::White)}
                {self.render_palette_cell(Color::Gray)}
                {self.render_palette_cell(Color::Blue)}
                {self.render_palette_cell(Color::Orange)}
                {self.render_palette_cell(Color::Yellow)}
                {self.render_palette_cell(Color::Red)}
                {self.render_palette_cell(Color::Green)}
                {self.render_palette_cell(Color::Brown)}
              </tr>
            </table>
          </>
        }
    }

    fn render_palette_cell(&self, color: Color) -> Html {
        let class = if color == self.interact.current_color() {
            "selected"
        } else {
            ""
        };
        let style_str = format!("background: {}", color.to_string());
        let click_callback = self.link.callback(move |_| Message::SelectColor(color));
        html! {
            <td class=class onclick=click_callback style={style_str}></td>
        }
    }

    fn render_controls(&self) -> Html {
        CONTROLS_ONCE.call_once(|| {
            let munger = CssMunger::new();
            munger.insert_rule("#topstuff .card{height:100%}");
        });

        let x_compass = self.link.callback(Message::MetaXShift);
        let y_compass = self.link.callback(Message::MetaYShift);
        let x_callback = self.link.callback(Message::NewColVec);
        let y_callback = self.link.callback(Message::NewRowVec);
        let row_count = self.stored.base_grid.num_rows();
        let col_count = self.stored.base_grid.num_cols();

        html! {
          <div id="topstuff" class="row">
            {bootstrap::col(bootstrap::card("Palette", "", self.render_palette()))}
            {bootstrap::col(bootstrap::card("Metapixel config", "", html!{
              <>
                <div>{"Metapixel config (x-axis):"}
                  <Input vec=self.stored.col_grid_cols.clone() callback=x_callback/>
                  <small>{" Pixel count: "}{col_count}</small>
                </div>
                <Compass callback=x_compass vert=false/>
                <div>{"Metapixel config (y-axis):"}
                  <Input vec=self.stored.row_grid_cols.clone() callback=y_callback/>
                  <small>{" Pixel count: "}{row_count}</small>
                </div>
                <Compass callback=y_compass vert=false/>
              </>
            }))}
          </div>
        }
    }

    fn render_base_grid(&self) -> Html {
        let mut renderer = TableRenderer::new(&self.stored.base_grid);
        renderer.add_class_decorator(RegularSizedTableDecorator::default());
        renderer.add_style_decorator(ColorDecorator::default());
        renderer.add_class_decorator(BorderedCellDecorator::default());
        renderer.add_class_decorator(PrintableColorDecorator::default());
        self.interact.install(&self.link, &mut renderer);

        let cb = self.link.callback(Message::BaseShift);
        let clear_cb = self.link.callback(|_| Message::Clear);

        bootstrap::row(bootstrap::col(bootstrap::card(
            "Pixel grid",
            "",
            html! {
              <>
                {renderer.render()}
                <Compass callback=cb/>
                <button onclick=clear_cb class="btn btn-outline-primary">{"Clear"}</button>
              </>
            },
        )))
    }

    fn render_meta_grid(&self) -> Html {
        let metagrid = MetaGrid::new(
            GridId::SmallOne,
            &self.stored.base_grid,
            &self.stored.row_grid_cols,
            &self.stored.col_grid_cols,
        );
        let mut renderer = TableRenderer::new(&metagrid);
        renderer.add_class_decorator(RegularSizedTableDecorator::default());
        renderer.add_style_decorator(ColorDecorator::default());
        renderer.add_class_decorator(BorderedCellDecorator::default());
        renderer.add_class_decorator(PrintableColorDecorator::default());
        renderer.add_class_decorator(MetagridDecorator::new(
            &self.stored.row_grid_cols,
            &self.stored.col_grid_cols,
        ));
        // TODO: for some reason, this is crashing.
        self.interact.install(&self.link, &mut renderer);

        // let cb = self.link.callback(Message::MetagridShift);

        bootstrap::row(bootstrap::col(bootstrap::card(
            "Metapixel grid",
            "",
            html! {
                <>
                    {renderer.render()}
                    //<Compass callback=cb/>
                </>
            },
        )))
    }
}

/*
  __*_____*__
  ___*___*___
  __*******__
  _**_***_**_
  ***********
  *_*******_*
  *_*_____*_*
  ___**_**___
*/
const INVADER_SIZE: (usize, usize) = (11, 8);
const INVADER: [(usize, usize); 46] = [
    (0, 2),
    (0, 8),
    (1, 3),
    (1, 7),
    (2, 2),
    (2, 3),
    (2, 4),
    (2, 5),
    (2, 6),
    (2, 7),
    (2, 8),
    (3, 1),
    (3, 2),
    (3, 4),
    (3, 5),
    (3, 6),
    (3, 8),
    (3, 9),
    (4, 0),
    (4, 1),
    (4, 2),
    (4, 3),
    (4, 4),
    (4, 5),
    (4, 6),
    (4, 7),
    (4, 8),
    (4, 9),
    (4, 10),
    (5, 0),
    (5, 2),
    (5, 3),
    (5, 4),
    (5, 5),
    (5, 6),
    (5, 7),
    (5, 8),
    (5, 10),
    (6, 0),
    (6, 2),
    (6, 8),
    (6, 10),
    (7, 3),
    (7, 4),
    (7, 6),
    (7, 7),
];

fn make_invader_grid() -> BigGrid {
    let mut grid = BigGrid::new(GridId::Main, INVADER_SIZE.0, INVADER_SIZE.1);
    for (row, col) in INVADER.iter() {
        grid.set_cell(*row, *col, Color::Green)
    }
    grid
}

impl Component for MetapixelApp {
    type Message = Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).unwrap();
        let Json(stored) = storage.restore(STORAGE_KEY);

        let mut grid = BigGrid::new(GridId::Main, INVADER_SIZE.0, INVADER_SIZE.1);
        for (row, col) in INVADER.iter() {
            grid.set_cell(*row, *col, Color::Green)
        }

        let mut interact: ColoredInteractor = Default::default();
        interact.set_current_color(Color::Orange);

        MetapixelApp {
            link,
            stored: stored.unwrap_or_else(|_| Stored {
                base_grid: make_invader_grid(),
                row_grid_cols: vec![1, 2, 1, 3, 3, 1, 2, 1],
                col_grid_cols: vec![1, 2, 3, 2, 1, 1, 1, 2, 3, 2, 1],
            }),
            interact,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut save = false;
        let should_render = match msg {
            Message::Clear => {
                self.stored.base_grid.clear();
                true
            }

            Message::NewRowVec(vec) => {
                // TODO: check for equality
                self.stored.row_grid_cols = vec;
                self.stored.base_grid.resize(
                    self.stored.row_grid_cols.len(),
                    self.stored.col_grid_cols.len(),
                );
                save = true;
                true
            }
            Message::NewColVec(vec) => {
                // TODO: check for equality
                self.stored.col_grid_cols = vec;
                self.stored.base_grid.resize(
                    self.stored.row_grid_cols.len(),
                    self.stored.col_grid_cols.len(),
                );
                save = true;
                true
            }

            Message::SelectColor(color) => {
                self.interact.set_current_color(color);
                true
            }

            Message::MetaXShift(direction) => {
                if self.stored.col_grid_cols.is_empty() {
                    false
                } else {
                    if direction == CompassDirection::Left {
                        let val = self.stored.col_grid_cols.remove(0);
                        self.stored.col_grid_cols.push(val);
                    } else {
                        /* CompassDirection::Right */
                        // unwrap: we have checked the length of the Vec above.
                        let val = self.stored.col_grid_cols.pop().unwrap();
                        self.stored.col_grid_cols.insert(0, val);
                    }
                    save = true;
                    true
                }
            }
            Message::MetaYShift(direction) => {
                if self.stored.row_grid_cols.is_empty() {
                    false
                } else {
                    if direction == CompassDirection::Left {
                        let val = self.stored.row_grid_cols.remove(0);
                        self.stored.row_grid_cols.push(val);
                    } else {
                        /* CompassDirection::Right */
                        // unwrap: we have checked the length of the Vec above.
                        let val = self.stored.row_grid_cols.pop().unwrap();
                        self.stored.row_grid_cols.insert(0, val);
                    }
                    save = true;
                    true
                }
            }

            Message::BaseShift(direction) => {
                match direction {
                    CompassDirection::Left => {
                        shift_rows(&mut self.stored.base_grid, ShiftDirection::Left)
                    }
                    CompassDirection::Right => {
                        shift_rows(&mut self.stored.base_grid, ShiftDirection::Right)
                    }
                    CompassDirection::Up => {
                        shift_cols(&mut self.stored.base_grid, ShiftDirection::Left)
                    }
                    CompassDirection::Down => {
                        shift_cols(&mut self.stored.base_grid, ShiftDirection::Right)
                    }
                }
                save = true;
                true
            }
            Message::MetagridShift(_direction) => false,

            m @ Message::Up(_)
            | m @ Message::Down(_)
            | m @ Message::Enter(_)
            | m @ Message::Leave(_) => {
                // Only save on Up.
                if let Message::Up(_) = &m {
                    save = true;
                }

                let (grid, interact) = self.grid_with_interact(m.cell_id().grid_id);
                if let Some(should_render) = interact.update(grid, &m) {
                    should_render
                } else {
                    false
                }
            }
        };

        if save {
            let mut storage = StorageService::new(Area::Local).unwrap();
            storage.store(STORAGE_KEY, Json(&self.stored));
        }

        should_render
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <>
          {self.render_nav()}
          {bootstrap::spacer()}
          {main_container(html! {
              <>
                  {self.render_controls()}
                  {bootstrap::spacer()}
                  {self.render_base_grid()}
                  {bootstrap::spacer()}
                  {self.render_meta_grid()}
                  {bootstrap::spacer()}
                  {bootstrap::spacer()}
              </>
          })}
          {self.render_footer()}
          </>
        }
    }
}
