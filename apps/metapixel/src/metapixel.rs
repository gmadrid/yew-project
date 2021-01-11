use bootstrap::main_container;
use components::{Compass, CompassDirection, Input};
use grids::{
    shift_cols, shift_rows, BigGrid, CellId, Color, GridId, GridTrait, MetaGrid, ShiftDirection,
};
use renderer::decorators::{
    BorderedCellDecorator, ColorDecorator, CssMunger, PrintableColorDecorator,
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
}

pub struct MetapixelApp {
    link: ComponentLink<Self>,
    interact: ColoredInteractor,
    stored: Stored,
    row_grid_cols: Vec<u8>,
    col_grid_cols: Vec<u8>,
}

#[derive(Debug)]
pub enum Message {
    Down(CellId),
    Up(CellId),
    Enter(CellId),
    Leave(CellId),

    ControlShift(CompassDirection),
    BaseShift(CompassDirection),
    MetagridShift(CompassDirection),
    SelectColor(Color),

    NewRowVec(Vec<u8>),
    NewColVec(Vec<u8>),

    NoOp,
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

        let x_compass = self
            .link
            .callback(|direction| Message::ControlShift(direction));
        let y_compass = self
            .link
            .callback(|direction| Message::ControlShift(direction));
        let x_callback = self.link.callback(|v| Message::NewColVec(v));
        let y_callback = self.link.callback(|v| Message::NewRowVec(v));
        let row_count = self.stored.base_grid.num_rows();
        let col_count = self.stored.base_grid.num_cols();

        html! {
          <div id="topstuff" class="row">
            {bootstrap::col(bootstrap::card("Palette", "", self.render_palette()))}
            {bootstrap::col(bootstrap::card("Metapixel config", "", html!{
              <>
                <div>{"Metapixel config (x-axis):"}
                  <Input vec=self.col_grid_cols.clone() callback=x_callback/>
                  <small>{" Pixel count: "}{col_count}</small>
                </div>
                <Compass callback=x_compass vert=false/>
                <div>{"Metapixel config (y-axis):"}
                  <Input vec=self.row_grid_cols.clone() callback=y_callback/>
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

        let cb = self
            .link
            .callback(|direction| Message::BaseShift(direction));

        bootstrap::row(bootstrap::col(bootstrap::card(
            "Pixel grid",
            "",
            html! {
              <>
                {renderer.render()}
                <Compass callback=cb/>
              </>
            },
        )))
    }

    fn render_meta_grid(&self) -> Html {
        let metagrid = MetaGrid::new(
            GridId::SmallOne,
            &self.stored.base_grid,
            &self.row_grid_cols,
            &self.col_grid_cols,
        );
        let mut renderer = TableRenderer::new(&metagrid);
        renderer.add_class_decorator(RegularSizedTableDecorator::default());
        renderer.add_style_decorator(ColorDecorator::default());
        renderer.add_class_decorator(BorderedCellDecorator::default());
        renderer.add_class_decorator(PrintableColorDecorator::default());
        // TODO: for some reason, this is crashing.
        self.interact.install(&self.link, &mut renderer);

        let cb = self
            .link
            .callback(|direction| Message::MetagridShift(direction));

        bootstrap::row(bootstrap::col(bootstrap::card(
            "Metapixel grid",
            "",
            html! {
                <>
                    {renderer.render()}
                    <Compass callback=cb/>
                </>
            },
        )))
    }
}

impl Component for MetapixelApp {
    type Message = Message;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut grid = BigGrid::new(GridId::Main, 5, 5);

        // Create a simple grid with a red diagonal stripe.
        for index in 0..(std::cmp::max(grid.num_rows(), grid.num_cols())) {
            grid.set_cell(index, index, Color::Red);
        }

        let mut interact: ColoredInteractor = Default::default();
        interact.set_current_color(Color::Orange);
        MetapixelApp {
            link,
            stored: Stored { base_grid: grid },
            interact,
            row_grid_cols: vec![3, 2, 1, 2, 3],
            col_grid_cols: vec![1, 2, 3, 2, 1],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let mut save = false;
        let should_render = match msg {
            Message::NoOp => false,

            Message::NewRowVec(vec) => {
                // TODO: check for equality
                self.row_grid_cols = vec;
                self.stored
                    .base_grid
                    .resize(self.row_grid_cols.len(), self.col_grid_cols.len());
                true
            }
            Message::NewColVec(vec) => {
                // TODO: check for equality
                self.col_grid_cols = vec;
                self.stored
                    .base_grid
                    .resize(self.row_grid_cols.len(), self.col_grid_cols.len());
                true
            }

            Message::SelectColor(color) => {
                self.interact.set_current_color(color);
                true
            }

            Message::ControlShift(_direction) => false,
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
                true
            }
            Message::MetagridShift(_direction) => false,

            m @ Message::Up(_)
            | m @ Message::Down(_)
            | m @ Message::Enter(_)
            | m @ Message::Leave(_) => {
                // Only save on Up.
                match &m {
                    Message::Up(_) => save = true,
                    _ => {}
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
//
// impl MetapixelApp {
//     fn new() -> Self {
//         let mut x_grid = SimpleGrid::new("left", 15, 15);
//         for coord in 0..std::cmp::min(x_grid.num_cols(), x_grid.num_rows()) {
//             x_grid.set_cell(coord, coord, Color::Red);
//             x_grid.set_cell(14 - coord, coord, Color::Green);
//         }
//
//         let mut y_grid = SimpleGrid::new("right", 15, 15);
//         for row in (0..y_grid.num_rows()).step_by(4) {
//             for col in 0..y_grid.num_cols() {
//                 y_grid.set_cell(row, col, Color::Gray);
//             }
//         }
//
//         MetapixelApp { x_grid, y_grid }
//     }
// }
//
// impl Component for MetapixelApp {
//     type Message = ();
//     type Properties = ();
//
//     fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
//         MetapixelApp::new()
//     }
//
//     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
//         false
//     }
//
//     fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
//         false
//     }
//
//     fn view(&self) -> Html {
//         let combined_grid = MergedGrid::new("merged", &self.x_grid, &self.y_grid);
//         let mut combined_renderer = TableRenderer::new(&combined_grid);
//         combined_renderer.add_class_decorator(RegularSizedTableDecorator::default());
//         combined_renderer.add_style_decorator(ColorDecorator::default());
//         combined_renderer.add_class_decorator(BorderedCellDecorator::default());
//         combined_renderer.add_class_decorator(MergedBorderDecorator::default());
//         combined_renderer.set_label_decorator(MergedFlatLabels::default());
//
//         let mut renderer = TableRenderer::new(&self.x_grid);
//         renderer.add_class_decorator(RegularSizedTableDecorator::default());
//         renderer.add_style_decorator(ColorDecorator::default());
//         renderer.add_class_decorator(PrintableColorDecorator::default());
//         renderer.add_class_decorator(BorderedCellDecorator::default());
//         renderer.add_class_decorator(ThickBorders::default());
//         renderer.set_label_decorator(FlatLabels::default());
//
//         let mut y_renderer = TableRenderer::new(&self.y_grid);
//         y_renderer.add_class_decorator(RegularSizedTableDecorator::default());
//         y_renderer.add_style_decorator(ColorDecorator::default());
//         y_renderer.add_class_decorator(PrintableColorDecorator::default());
//         y_renderer.add_class_decorator(BorderedCellDecorator::default());
//         y_renderer.add_class_decorator(ThickBorders::default());
//         y_renderer.set_label_decorator(RoundLabels::default());
//
//         let mut small_x_renderer = TableRenderer::new(&self.x_grid);
//         small_x_renderer.add_class_decorator(SmallSizedTableDecorator::default());
//         small_x_renderer.add_style_decorator(ColorDecorator::default());
//         small_x_renderer.add_class_decorator(PrintableColorDecorator::default());
//         small_x_renderer.add_class_decorator(BorderedCellDecorator::default());
//
//         let inverted_grid = InvertedGrid::new("inverted", &self.y_grid);
//         let mut inverted_renderer = TableRenderer::new(&inverted_grid);
//         inverted_renderer.add_class_decorator(SmallSizedTableDecorator::default());
//         inverted_renderer.add_style_decorator(ColorDecorator::default());
//         inverted_renderer.add_class_decorator(PrintableColorDecorator::default());
//         inverted_renderer.add_class_decorator(BorderedCellDecorator::default());
//
//         html! {
//             <main class="main container">
//               {bootstrap::div!("small", "JOIJO", "boop", "boopie")}
//               {bootstrap::col!("foo", "bar", "baz", "quux")}
//               <h1>{"X grid"}</h1>
//               {renderer.render()}
//               <h1>{"Y grid"}</h1>
//               {y_renderer.render()}
//               <h1>{"combined"}</h1>
//               {combined_renderer.render()}
//               <h1>{"X small"}</h1>
//               {small_x_renderer.render()}
//               <h1>{"Y inverted"}</h1>
//               {inverted_renderer.render()}
//             </main>
//         }
//     }
// }
