use super::{Interactions, Interactor};
use grids::{CellId, Color, GridTrait, HasGrids};
use std::convert::TryInto;
use std::marker::PhantomData;
use yew::prelude::*;

static info: fn(&str) = yew::services::ConsoleService::info;

pub struct OneColorInteractor<COMP, GRID>
where
    COMP: Component + HasGrids,
    GRID: GridTrait,
{
    current_color: Option<Color>,
    grid: PhantomData<GRID>,
    phantom: PhantomData<COMP>,
}

impl<COMP, GRID> OneColorInteractor<COMP, GRID>
where
    COMP: Component + HasGrids,
    GRID: GridTrait,
{
    pub fn new() -> OneColorInteractor<COMP, GRID> {
        OneColorInteractor {
            current_color: None,
            grid: PhantomData::default(),
            phantom: PhantomData::default(),
        }
    }

    fn mouse_down(&mut self, comp: &mut COMP, interaction: Interactions) -> bool {
        let cell_id = interaction.cell_id();
        let mut grid = comp.grid_for_id_mut(cell_id.grid_id);
        let color = grid.cell(cell_id.row, cell_id.col);
        self.current_color = Some(!color);
        grid.set_cell(cell_id.row, cell_id.col, !color);
        info("WHEEEEEEEEEEEE");
        true
    }
}

impl<COMP, GRID> Interactor<COMP> for OneColorInteractor<COMP, GRID>
where
    COMP: Component + HasGrids,
    COMP::Message: From<Interactions>,
    GRID: GridTrait,
{
    type State = GRID;

    fn update(&mut self, comp: &mut COMP, msg: impl TryInto<Interactions>) -> Option<bool> {
        let interaction = msg.try_into();
        if let Ok(interaction) = interaction {
            match interaction {
                Interactions::MouseUp => Some(self.mouse_down(comp, interaction)),
                Interactions::MouseDown(cell_id) => Some(false),
                Interactions::MouseEnter(cell_id) => Some(false),
                Interactions::MouseExit(cell_id) => Some(false),
            }
        } else {
            None
        }
    }
}
