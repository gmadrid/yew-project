use crate::bootstrap;
use crate::components::InputComponent;
use crate::gridtrait::GridTrait;
use crate::meta_grid::MetaGrid;
use yew::prelude::*;

fn sq() -> Html {
    html! {
        <svg width="20px" height="20px" viewBox="0 0 50 50" class="bi bi-circle-fill" xmlns="http://www.w3.org/2000/svg">
        </svg>
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    White,
    Gray,
    Blue,
    Orange,
    Yellow,
    Red,
    Green,
    Brown,
}

impl Color {
    pub fn style_str(&self) -> String {
        format!("background: {}", self.to_string())
    }
}

impl From<u8> for Color {
    fn from(f: u8) -> Color {
        match f {
            1 => Color::Gray,
            2 => Color::Blue,
            3 => Color::Orange,
            4 => Color::Yellow,
            5 => Color::Red,
            6 => Color::Green,
            7 => Color::Brown,
            _ => Color::White,
        }
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::White => "white".to_owned(),
            Color::Gray => "gray".to_owned(),
            Color::Blue => "blue".to_owned(),
            Color::Orange => "orange".to_owned(),
            Color::Yellow => "yellow".to_owned(),
            Color::Red => "red".to_owned(),
            Color::Green => "green".to_owned(),
            Color::Brown => "brown".to_owned(),
        }
    }
}

const MAX_ROWS: usize = 256;
const MAX_COLS: usize = 256;

struct ColorGrid {
    cells: Vec<Color>,
    num_rows: usize,
    num_cols: usize,
}

impl ColorGrid {
    fn new() -> ColorGrid {
        ColorGrid {
            num_rows: Default::default(),
            num_cols: Default::default(),
            cells: vec![Color::White; MAX_ROWS * MAX_COLS],
        }
    }

    fn resize(&mut self, rows: usize, cols: usize) {
        self.num_rows = rows;
        self.num_cols = cols;
    }
}

impl GridTrait<Color> for ColorGrid {
    fn num_rows(&self) -> usize {
        self.num_rows
    }

    fn num_cols(&self) -> usize {
        self.num_cols
    }

    fn cell(&self, row: usize, col: usize) -> Color {
        let index = row * MAX_COLS + col;
        self.cells[index]
    }

    fn set_cell(&mut self, row: usize, col: usize, value: Color) {
        let index = row * MAX_COLS + col;
        self.cells[index] = value;
    }

    fn clear(&mut self) {
        for index in 0..self.cells.len() {
            self.cells[index] = Color::Red;
        }
    }
}

pub struct Other {
    link: ComponentLink<Self>,

    base_grid: ColorGrid,

    row_grid_cols: Vec<u8>,
    col_grid_cols: Vec<u8>,
    current_color: Color,
}

pub enum Msg {
    NewRowVec(Vec<u8>),
    NewColVec(Vec<u8>),
    SelectColor(Color),
    SetCell(usize, usize),
}

impl Other {
    fn render_palette_cell(&self, color: Color) -> Html {
        let class = if color == self.current_color {
            "selected"
        } else {
            ""
        };
        html! {
            <td class=class onclick=self.link.callback(move |_| Msg::SelectColor(color))
            style={color.style_str()}
            ></td>
        }
    }

    fn render_palette(&self) -> Html {
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

    fn render_base_cell(&self, row_num: usize, col_num: usize) -> Html {
        let click_callback = self.link.callback(move |_| Msg::SetCell(row_num, col_num));
        let style_str = self.base_grid.cell(row_num, col_num).style_str();

        html! {
            <td style=style_str onclick=click_callback>
            </td>
        }
    }

    fn render_base_row(&self, row_num: usize) -> Html {
        html! {
            <tr>
            {for (0..self.base_grid.num_cols()).map(|cn| self.render_base_cell(row_num, cn))}
            </tr>
        }
    }

    fn render_base_grid(&self) -> Html {
        html! {
            <table>
              {for (0..self.base_grid.num_rows()).map(|rn| self.render_base_row(rn))}
            </table>
            //<span style={color_string}>{"Some text"}</span>
        }
    }

    fn render_meta_cell(
        &self,
        grid: &impl GridTrait<Color>,
        row_num: usize,
        col_num: usize,
    ) -> Html {
        let style_str = grid.cell(row_num, col_num).style_str();
        html! {
            <td style=style_str>
            </td>
        }
    }

    fn render_meta_row(&self, grid: &impl GridTrait<Color>, row_num: usize) -> Html {
        html! {
            <tr>
            {for (0..grid.num_cols()).map(|cn| {
                self.render_meta_cell(grid, row_num, cn)
            })}
            </tr>
        }
    }

    fn render_meta_grid(&self) -> Html {
        let meta_grid = MetaGrid::new(&self.base_grid, &self.row_grid_cols, &self.col_grid_cols);

        html! {
            <table>
              {for (0..meta_grid.num_rows()).map(|rn| self.render_meta_row(&meta_grid, rn))}
            </table>
        }
    }
}

impl Component for Other {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut grid = ColorGrid::new();
        for row in 0..100 {
            for col in 0..100 {
                let color = if row % 2 == col % 2 {
                    Color::Orange
                } else {
                    Color::White
                };
                // NOTE: This only works because ColorGrid doesn't bounds check.
                grid.set_cell(row, col, color);
            }
        }
        Other {
            link,
            base_grid: grid,
            row_grid_cols: Default::default(),
            col_grid_cols: Default::default(),
            current_color: Color::Gray,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewRowVec(vec) => self.row_grid_cols = vec,
            Msg::NewColVec(vec) => self.col_grid_cols = vec,
            Msg::SelectColor(color) => self.current_color = color,
            Msg::SetCell(row, col) => self.base_grid.set_cell(row, col, self.current_color),
        }

        self.base_grid
            .resize(self.row_grid_cols.len(), self.col_grid_cols.len());
        true
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let row_count = self.base_grid.num_rows();
        let col_count = self.base_grid.num_cols();

        let x_callback = self.link.callback(|vec| Msg::NewColVec(vec));
        let y_callback = self.link.callback(|vec| Msg::NewRowVec(vec));

        html! {
          <main class="main container">
          {bootstrap::spacer()}
          <div id="topstuff" class="row">
            <div class="col">
              {bootstrap::card("Palette", "", self.render_palette())}
            </div>
            <div class="col">
              {bootstrap::card("Metapixel config", "", html!{
                <>
                <div>{"Metapixel config (x-axis):"}
                  <InputComponent start="1,1,2,2,3,2,2,1,1" callback=x_callback/>
                  <small>{" Pixel count: "}{col_count}</small>
                </div>
                <div>{"Metapixel config (y-axis):"}
                  <InputComponent start="1,1,2,2,3,2,2,1,1" callback=y_callback/>
                  <small>{" Pixel count: "}{row_count}</small>
                </div>
                </>
            })}
            </div>
          </div>
            <h1>{"Pixel grid"}</h1>
            {self.render_base_grid()}
            <h1>{"Metapixel grid"}</h1>
            {self.render_meta_grid()}
          </main>
        }
    }
}
