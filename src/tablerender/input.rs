use super::{no_dot, TableRenderer};
use crate::app::{self, GridId};
use crate::bootstrap;
use crate::gridtrait::GridTrait;
use yew::prelude::*;

use super::render_table;

pub struct InputRenderer<'a, G>
where
    G: GridTrait<bool>,
{
    grid_id: GridId,
    grid: &'a G,
    link: &'a ComponentLink<app::App>,
}

impl<'a, G> InputRenderer<'a, G>
where
    G: GridTrait<bool>,
{
    pub fn render_table(
        link: &'a ComponentLink<crate::app::App>,
        grid_id: GridId,
        grid: &'a impl GridTrait<bool>,
    ) -> Html {
        render_table(InputRenderer::<'a> {
            grid,
            grid_id,
            link,
        })
    }
}

impl<'a, G> TableRenderer for InputRenderer<'a, G>
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
        let mut classes = vec![if value { "on" } else { "off" }];

        let display_col_num = self.grid.num_cols() - col;
        if display_col_num % 5 == 0 && display_col_num != self.grid.num_cols() {
            classes.push("vfiver");
        }

        let display_row_num = self.grid.num_rows() - row;
        if display_row_num % 5 == 0 && display_row_num != self.grid.num_rows() {
            classes.push("hfiver");
        };

        let grid_id = self.grid_id.clone();
        type Message = <app::App as Component>::Message;
        let down_callback = self
            .link
            .callback(move |_| Message::Down(grid_id, row, col));
        let enter_callback = self
            .link
            .callback(move |_| Message::Enter(grid_id, row, col));
        let exit_callback = self.link.callback(|_| Message::Exit);
        let up_callback = self.link.callback(|_| Message::Up);

        html! {
            <td class=classes
            onmousedown=down_callback
            onmouseenter=enter_callback
            onmouseleave=exit_callback
            onmouseup=up_callback
            >{no_dot()}</td>
        }
    }

    fn render_left_cell(&self, row: usize) -> Html {
        let displayed_row_num = self.num_data_rows() - row;
        html! {
            <td class="left">
              { if displayed_row_num % 2 == 0 {
                  // +2 is a weird wrinkle specifically for Alasdair's class.
                  html!{<small>{displayed_row_num +2}</small>}
              } else {
                  bootstrap::empty()
              }}
            </td>
        }
    }

    fn render_right_cell(&self, row: usize) -> Html {
        let displayed_row_num = self.num_data_rows() - row;
        html! {
            <td class="right">
              { if displayed_row_num % 2 != 0 {
                  // +2 is a weird wrinkle specifically for Alasdair's class.
                  html!{<small>{displayed_row_num+2}</small>}
              } else {
                  bootstrap::empty()
              }}
            </td>
        }
    }

    fn render_footer_row(&self) -> Html {
        let num_data_cols = self.num_data_cols();
        html! {
            <tr class={"tablefooter"}>
                <td></td>  // skipping the "left" column
                {for (0..num_data_cols).map(|cn| {
                    html!{<td><small>{num_data_cols - cn + 2}</small></td>}
                })}
                <td></td>  // skipping the "right" column
            </tr>
        }
    }

    fn render_footer_cell(&self, _: usize) -> Html {
        todo!()
    }
}
