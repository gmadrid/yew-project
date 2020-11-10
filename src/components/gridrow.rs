use crate::app::GridId;
use crate::biggrid::BigGrid;
use crate::components::gridcell::GridCell;
use crate::gridtrait::GridTrait;
use std::rc::Rc;
use yew::prelude::*;

pub struct GridRow {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    pub grid: Rc<BigGrid<bool>>,
    pub row: usize,
    pub onclick: Callback<(GridId, usize, usize)>,
}

impl GridRow {}

impl Component for GridRow {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        GridRow { link, props }
    }

    fn update(&mut self, msg: Msg) -> bool {
        match msg {}
    }

    fn change(&mut self, _props: Props) -> bool {
        // todo: make this better/efficient.
        false
    }

    fn view(&self) -> Html {
        let make_cell = |col: usize| {
            html! { <GridCell
                row=self.props.row
                col=col
            onclick=self.props.onclick.clone()
            val=self.props.grid.as_ref().cell(self.props.row, col) /> }
        };
        html! { <tr>
        { for (0..self.props.grid.as_ref().num_cols()).map(make_cell)}
        </tr>}
    }
}
