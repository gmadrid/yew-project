use crate::components::gridcell::GridCell;
use yew::prelude::*;

pub struct GridRow {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub val: usize,
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

    fn change(&mut self, props: Props) -> bool {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let make_cell = |_: usize| {
            html! { <GridCell val=self.props.val /> }
        };
        html! { <tr>
        { for (0..10).map(make_cell)}
        </tr>}
    }
}
