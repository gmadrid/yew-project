use crate::app::GridId;
use crate::biggrid::BigGrid;
use crate::components::gridrow::GridRow;
use crate::gridtrait::GridTrait;
use std::rc::Rc;
use yew::prelude::*;

pub struct GridTable {
    link: ComponentLink<Self>,
    props: Props,
}

impl GridTable {}

pub enum Msg {}

#[derive(Properties, Clone)]
pub struct Props {
    pub grid: Rc<BigGrid<bool>>,
    pub onclick: Callback<(GridId, usize, usize)>,
}

impl Component for GridTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        GridTable { link, props }
    }

    fn update(&mut self, msg: Msg) -> bool {
        match msg {}
    }

    fn change(&mut self, _props: Props) -> bool {
        false
    }

    fn view(&self) -> Html {
        let make_row = |index: usize| {
            html! { <GridRow
            onclick=self.props.onclick.clone()
            grid=self.props.grid.clone() row=index /> }
        };
        html! { <table class={"user-select-none"}>
        { for (0..(self.props.grid.as_ref().num_rows())).map(make_row)}
        </table>}
    }
}
