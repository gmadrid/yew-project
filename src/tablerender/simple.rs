use super::{render_table, TableRenderer};
use crate::bootstrap::empty;
use crate::grids::GridTrait;
use yew::prelude::*;

pub struct SimpleRenderer<'a, G>
where
    G: GridTrait<bool>,
{
    grid: &'a G,
}

impl<'a, G> SimpleRenderer<'a, G>
where
    G: GridTrait<bool>,
{
    pub fn render_table(grid: &'a impl GridTrait<bool>) -> Html {
        render_table(SimpleRenderer::<'a> { grid })
    }
}

impl<'a, G> TableRenderer for SimpleRenderer<'a, G>
where
    G: GridTrait<bool>,
{
    fn num_data_rows(&self) -> usize {
        self.grid.num_rows()
    }

    fn num_data_cols(&self) -> usize {
        self.grid.num_cols()
    }

    fn render_data_row(&self, row: usize) -> Html {
        html! {
            <>
            {for (0..self.num_data_cols()).map(|cn| self.render_data_cell(row, cn))}
            </>
        }
    }
    fn render_data_cell(&self, row: usize, col: usize) -> Html {
        let value = self.grid.cell(row, col);
        let classes = vec![if value { "on" } else { "off" }];

        html! {
            <td class=classes></td>
        }
    }

    fn render_left_cell(&self, _: usize) -> Html {
        empty()
    }
    fn render_right_cell(&self, _: usize) -> Html {
        empty()
    }
    fn render_footer_row(&self) -> Html {
        empty()
    }
    fn render_footer_cell(&self, _: usize) -> Html {
        empty()
    }
}
