use crate::bootstrap;
use crate::components::InputComponent;
use crate::grids::{GridTrait, TiledGrid};
use crate::meta_grid::{IndirectGrid, MetaGrid};
use crate::simplegrid::SimpleGrid;
use yew::prelude::*;

pub struct Tiles {
    link: ComponentLink<Self>,

    base_grid: SimpleGrid<bool>,
}

pub enum Msg {
    SetCell(usize, usize),
}

impl Tiles {
    fn render_base_grid(&self) -> Html {
        self.render_grid(&self.base_grid, "")
    }

    fn render_grid(&self, grid: &impl GridTrait<bool>, class: &str) -> Html {
        let mut classes = Vec::default();
        if !class.is_empty() {
            classes.push(class);
        }
        html! {
            <table class=classes>
              {for (0..grid.num_rows()).map(|rn| self.render_base_row(grid, rn))}
            </table>
        }
    }

    fn render_base_row(&self, grid: &impl GridTrait<bool>, row_num: usize) -> Html {
        html! {
            <tr>
            {for (0..grid.num_cols()).map(|cn| self.render_base_cell(grid, row_num, cn))}
            </tr>
        }
    }

    fn render_base_cell(&self, grid: &impl GridTrait<bool>, row: usize, col: usize) -> Html {
        let click_callback = self.link.callback(move |_| Msg::SetCell(row, col));
        let value = grid.cell(row, col);
        let style_str = if value {
            "background: darkgray; color-adjust:exact; -webkit-print-color-adjust: exact;"
        } else {
            ""
        };

        html! {
            <td style=style_str onclick=click_callback>
            </td>
        }
    }

    fn render_tiled_grid(&self) -> Html {
        let tiled = TiledGrid::new(&self.base_grid);

        html! {
            {self.render_grid(&tiled, "minitable")}
        }
    }
}

impl Component for Tiles {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut grid = SimpleGrid::new(12, 8);
        Tiles {
            link,
            base_grid: grid,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetCell(row, col) => {
                //TODO: maybe move this to the callback.
                let row = row % self.base_grid.num_rows();
                let col = col % self.base_grid.num_cols();

                let curr_val = self.base_grid.cell(row, col);
                self.base_grid.set_cell(row, col, !curr_val);
            }
        }
        true
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main class="main container">
                {bootstrap::spacer()}
                {bootstrap::row(bootstrap::col(
                    bootstrap::concat_spaced(
                    bootstrap::card("Pattern", "", html!{
                        {self.render_base_grid()}
                    }),
                    bootstrap::card("Tiled", "", html!{
                        {self.render_tiled_grid()}
                    })
                )))}
            </main>
        }
    }
}
