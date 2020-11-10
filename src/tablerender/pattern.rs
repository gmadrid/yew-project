use super::{no_dot, TableRenderer};
use crate::app::{self, GridId};
use crate::bootstrap;
use crate::gridtrait::GridTrait;
use yew::prelude::*;

use super::render_table;

fn purl_dot() -> Html {
    html! {
        <svg width="1em" height="1em" viewBox="0 0 15 15" class="bi bi-circle-fill" fill="black" xmlns="http://www.w3.org/2000/svg">
          <circle cx="7.5" cy="7.5" r="4"/>
        </svg>
    }
}

pub struct PatternRenderer<'a, G1, G2>
where
    G1: GridTrait<bool>,
    G2: GridTrait<bool>,
{
    front_id: GridId,
    front: &'a G1,
    back_id: GridId,
    back: &'a G2,
    link: &'a ComponentLink<app::App>,
}

impl<'a, G1, G2> PatternRenderer<'a, G1, G2>
where
    G1: GridTrait<bool>,
    G2: GridTrait<bool>,
{
    pub fn render_table(
        link: &'a ComponentLink<app::App>,
        front_id: GridId,
        front: &'a impl GridTrait<bool>,
        back_id: GridId,
        back: &'a impl GridTrait<bool>,
    ) -> Html {
        // TODO: add consistency checks for the two tables.
        render_table(PatternRenderer::<'a> {
            front_id,
            front,
            back_id,
            back,
            link,
        })
    }
}

impl<'a, G1, G2> TableRenderer for PatternRenderer<'a, G1, G2>
where
    G1: GridTrait<bool>,
    G2: GridTrait<bool>,
{
    fn num_data_rows(&self) -> usize {
        self.front.num_rows()
    }

    fn num_data_cols(&self) -> usize {
        2 * self.front.num_cols()
    }

    fn render_data_row(&self, row: usize) -> Html {
        html! {
            <>
            {for (0..self.num_data_cols()).map(|cn| self.render_data_cell(row, cn))}
            </>
        }
    }

    fn render_data_cell(&self, row: usize, col: usize) -> Html // ???
    {
        let mut classes = vec![];
        if col % 2 == 0 {
            classes.push("purl");
            if col != 0 {
                classes.push("vfiver");
            }
        }
        let content = if col % 2 == 0 { purl_dot() } else { no_dot() };

        let real_col = col / 2;
        let (grid_id, value) = if col % 2 == 0 {
            (self.back_id, self.back.cell(row, real_col))
        } else {
            (self.front_id, self.front.cell(row, real_col))
        };
        if value {
            classes.push("on");
        } else {
            classes.push("off");
        }

        let displayed_row_num = self.num_data_rows() - row;
        if displayed_row_num % 5 == 0 && displayed_row_num != self.front.num_rows() {
            classes.push("hfiver");
        }

        type Message = <app::App as Component>::Message;
        let down_callback = self
            .link
            .callback(move |_| Message::Down(grid_id, row, real_col));
        let up_callback = self.link.callback(|_| Message::Up);
        let enter_callback = self
            .link
            .callback(move |_| Message::Enter(grid_id, row, real_col));
        let exit_callback = self.link.callback(|_| Message::Exit);

        html! {
            <td class=classes
                onmousedown=down_callback
                onmouseenter=enter_callback
                onmouseup=up_callback
                onmouseleave=exit_callback
            >{content}</td>
        }
    }

    fn render_left_cell(&self, row: usize) -> Html {
        let displayed_row_num = self.num_data_rows() - row;
        html! {
            <td class="left">
              { if displayed_row_num % 2 == 0 {
                  // +2 is a weird wrinkle specifically for Alasdair's class.
                  html!{<small>{displayed_row_num + 2}</small>}
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
                  html!{<small>{displayed_row_num + 2}</small>}
              } else {
                  bootstrap::empty()
              }}
            </td>
        }
    }

    fn render_footer_row(&self) -> Html {
        html! {
            <tr>
              <td class="left"></td>
              {for (0..self.num_data_cols()).map(|cn| {
                  let display_num = (self.num_data_cols() - cn) / 2;
                  if cn % 2 == 0 {
                    html!{<td class="patternfootercell purl" colspan=2><small>{display_num}</small></td>}
                  } else {
                    bootstrap::empty()
                  }
              })}
              <td class="right"></td>
            </tr>
        }
    }

    fn render_footer_cell(&self, _: usize) -> Html // ???
    {
        bootstrap::empty()
    }
}
