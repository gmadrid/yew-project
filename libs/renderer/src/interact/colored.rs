use super::{Interactions, Interactor};
use grids::{CellId, Color, GridTrait};
use std::convert::TryInto;

#[derive(Default)]
pub struct ColoredInteractor {
    active: bool,
    current_color: Color,
}

impl ColoredInteractor {
    pub fn new() -> ColoredInteractor {
        Default::default()
    }

    pub fn current_color(&self) -> Color {
        self.current_color
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color
    }

    fn set_cell_at_cell_id(&self, grid: &mut dyn GridTrait, cell_id: CellId) -> bool {
        if grid.cell(cell_id.row, cell_id.col) == self.current_color {
            false
        } else {
            grid.set_cell(cell_id.row, cell_id.col, self.current_color);
            true
        }
    }

    fn mouse_down(&mut self, grid: &mut dyn GridTrait, cell_id: CellId) -> bool {
        self.active = true;
        self.set_cell_at_cell_id(grid, cell_id)
    }

    fn mouse_enter(&mut self, grid: &mut dyn GridTrait, cell_id: CellId) -> bool {
        if self.active {
            self.set_cell_at_cell_id(grid, cell_id)
        } else {
            false
        }
    }

    fn mouse_exit(&mut self, _: &mut dyn GridTrait, _: CellId) -> bool {
        false
    }

    fn mouse_up(&mut self, _: &mut dyn GridTrait, _: CellId) -> bool {
        self.active = false;
        false
    }
}

impl Interactor for ColoredInteractor {
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
