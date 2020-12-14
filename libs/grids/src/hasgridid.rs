// A trait for grids that have a Grid id. Not essential for most grids.
pub trait HasGridId {
    fn grid_id(&self) -> &str;
}
