use crate::bootstrap;
use crate::components::InputComponent;
use crate::grids::{Color, ColorGrid, GridTrait};
use crate::meta_grid::{IndirectGrid, MetaGrid};
use yew::prelude::*;

pub struct Metapixel {
    // link: ComponentLink<Self>,
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

impl Metapixel {
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
        bootstrap::card(
            "Pixel grid",
            "",
            html! {
                <table>
                  {for (0..self.base_grid.num_rows()).map(|rn| self.render_base_row(rn))}
                </table>
            },
        )
    }

    fn render_meta_cell(
        &self,
        grid: &(impl GridTrait + IndirectGrid),
        row_num: usize,
        col_num: usize,
    ) -> Html {
        let (base_row, base_col) = grid.to_base(row_num, col_num);

        let mut classes = vec![];
        if row_num > 0 {
            let (prev_base_row, _) = grid.to_base(row_num - 1, col_num);
            if prev_base_row != base_row {
                classes.push("meta_grid_horiz");
            }
        }
        if col_num > 0 {
            let (_, prev_base_col) = grid.to_base(row_num, col_num - 1);
            if prev_base_col != base_col {
                classes.push("meta_grid_vert");
            }
        }

        let click_callback = self
            .link
            .callback(move |_| Msg::SetCell(base_row, base_col));
        let style_str = grid.cell(row_num, col_num).style_str();
        html! {
            <td class=classes onclick=click_callback style=style_str>
            </td>
        }
    }

    fn render_meta_row(&self, grid: &(impl GridTrait + IndirectGrid), row_num: usize) -> Html {
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
        bootstrap::card(
            "Metapixel grid",
            "",
            html! {
                <table>
                  {for (0..meta_grid.num_rows()).map(|rn| self.render_meta_row(&meta_grid, rn))}
                </table>
            },
        )
    }
}

impl Component for Metapixel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
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
        Metapixel {
            // link,
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

        let x_callback = self.link.callback(Msg::NewColVec);
        let y_callback = self.link.callback(Msg::NewRowVec);

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
          {bootstrap::spacer()}
          <div class="row"><div class="col">
            {self.render_base_grid()}
          </div></div>
          {bootstrap::spacer()}
          <div class="row"><div class="col">
            {self.render_meta_grid()}
          </div></div>
          </main>
        }
    }
}
