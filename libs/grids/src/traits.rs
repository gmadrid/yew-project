use crate::{GridId, GridTrait};

// A trait for grids that have a Grid id. Not essential for most grids.
pub trait HasGridId {
    fn grid_id(&self) -> &str;
}

pub trait HasGrids {
    fn grid_for_id(&self, grid_id: GridId) -> &dyn GridTrait;
    fn grid_for_id_mut(&mut self, grid_id: GridId) -> &mut dyn GridTrait;
}
