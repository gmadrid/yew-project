use grids::GridTrait;
use yew::prelude::*;

pub struct TableRenderer<'a, GRID: GridTrait> {
    grid: &'a GRID,
}

impl<'a, GRID: GridTrait> TableRenderer<'a, GRID> {
    pub fn new(grid: &'a GRID) -> Self {
        TableRenderer { grid }
    }

    fn render_full_row(&self, row: usize) -> Html {
        html! {
            <tr>
              {for (0..self.grid.num_cols()).map(|cn| {
                  {self.render_cell(row, cn)}
              })}
            </tr>
        }
    }

    fn render_cell(&self, row: usize, col: usize) -> Html {
        let style_string = self.grid.cell(row, col).style_str();
        html! {
            <td style=style_string></td>
        }
    }

    pub fn render(&self) -> Html {
        html! {
          <table>
            {for (0..self.grid.num_rows()).map(|rn| {
                {self.render_full_row(rn)}
            })}
          </table>
        }
    }
}
