use super::{Interactions, Interactor};
use grids::{CellId, Color, GridTrait};
use std::convert::TryInto;

pub struct OneColorInteractor {
    current_color: Option<Color>,
}

impl OneColorInteractor {
    pub fn new() -> OneColorInteractor {
        OneColorInteractor {
            current_color: None,
        }
    }

    fn mouse_down(&mut self, grid: &mut dyn GridTrait, cell_id: CellId) -> bool {
        let color = grid.cell(cell_id.row, cell_id.col);
        self.current_color = Some(!color);
        grid.set_cell(cell_id.row, cell_id.col, !color);
        true
    }

    fn mouse_enter(&mut self, grid: &mut dyn GridTrait, cell_id: CellId) -> bool {
        if let Some(current_color) = self.current_color {
            let old_color = grid.cell(cell_id.row, cell_id.col);
            if current_color != old_color {
                grid.set_cell(cell_id.row, cell_id.col, current_color);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn mouse_exit(&mut self, _: &mut dyn GridTrait, _: CellId) -> bool {
        false
    }

    fn mouse_up(&mut self, _: &mut dyn GridTrait, _: CellId) -> bool {
        self.current_color = None;
        false
    }
}

impl Interactor for OneColorInteractor {
    fn update(
        &mut self,
        grid: &mut dyn GridTrait,
        msg: impl TryInto<Interactions>,
    ) -> Option<bool> {
        let interaction = msg.try_into();
        if let Ok(interaction) = interaction {
            match interaction {
                Interactions::MouseDown(cell_id) => Some(self.mouse_down(grid, cell_id)),
                Interactions::MouseUp(cell_id) => Some(self.mouse_up(grid, cell_id)),
                Interactions::MouseEnter(cell_id) => Some(self.mouse_enter(grid, cell_id)),
                Interactions::MouseExit(cell_id) => Some(self.mouse_exit(grid, cell_id)),
            }
        } else {
            None
        }
    }
}
