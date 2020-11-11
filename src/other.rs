use crate::components::InputComponent;
use crate::gridtrait::GridTrait;
use yew::prelude::*;

fn no_dot() -> Html {
    html! {
        <svg width="1em" height="1em" viewBox="0 0 50 50" class="bi bi-circle-fill" fill="black" xmlns="http://www.w3.org/2000/svg">
        </svg>
    }
}

fn color_sq(color: Color) -> Html {
    let color_str = color.to_string();
    let style_str = format!("background: {}", color_str);
    html! {
        <svg style=style_str width="1em" height="1em" viewBox="0 0 50 50" class="bi bi-circle-fill" fill=color_str xmlns="http://www.w3.org/2000/svg">
        </svg>
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Color {
    White,
    Gray,
    Blue,
    Orange,
    Yellow,
    Red,
    Green,
    Brown,
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
}

pub enum Msg {
    NewRowVec(Vec<u8>),
    NewColVec(Vec<u8>),
}

impl Other {
    fn render_palette(&self) -> Html {
        html! {
          <>
            <h1>{"Palette"}</h1>
            <table id="palettetable">
            <tr><td class="colora">{no_dot()}</td><td>{"Color A"}</td>
            <td class="colorb">{no_dot()}</td><td>{"Color B"}</td>
            <td class="colorc">{no_dot()}</td><td>{"Color C"}</td>
            <td class="colord">{no_dot()}</td><td>{"Color D"}</td>
            <td class="colore">{no_dot()}</td><td>{"Color E"}</td>
            <td class="colorf">{no_dot()}</td><td>{"Color F"}</td>
            <td class="colorg">{no_dot()}</td><td>{"Color G"}</td>
            <td class="colorh">{no_dot()}</td><td>{"Color H"}</td></tr>
            </table>
          </>
        }
    }

    fn display_base_cell(&self, row_num: usize, col_num: usize) -> Html {
        html! {
            <td>
            {color_sq(Color::White)}
            </td>
        }
    }

    fn display_base_row(&self, row_num: usize) -> Html {
        html! {
            <tr>
            {for (0..self.base_grid.num_cols()).map(|cn| self.display_base_cell(row_num, cn))}
            </tr>
        }
    }

    fn display_base_grid(&self) -> Html {
        html! {
            <table>
              {for (0..self.base_grid.num_rows()).map(|rn| self.display_base_row(rn))}
            </table>
            //<span style={color_string}>{"Some text"}</span>
        }
    }
}

impl Component for Other {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Other {
            link,
            base_grid: ColorGrid::new(),
            row_grid_cols: Default::default(),
            col_grid_cols: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NewRowVec(vec) => self.row_grid_cols = vec,
            Msg::NewColVec(vec) => self.col_grid_cols = vec,
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

        let x_callback = self.link.callback(|vec| {
            yew::services::ConsoleService::log("GOT HERE");
            Msg::NewColVec(vec)
        });
        let y_callback = self.link.callback(|vec| {
            yew::services::ConsoleService::log("GOT HERE");
            Msg::NewRowVec(vec)
        });

        html! {
          <>
            {self.render_palette()}
            <h1>{"Metapixel config"}</h1>
            <div>{"Metapixel config (x-axis):"}<InputComponent callback=x_callback/><small>{" Pixel count: "}{col_count}</small></div>
            <div>{"Metapixel config (y-axis):"}<InputComponent callback=y_callback/><small>{" Pixel count: "}{row_count}</small></div>
            <h1>{"Pixel grid"}</h1>
            {self.display_base_grid()}
            <h1>{"Metapixel grid"}</h1>
          </>
        }
    }
}
