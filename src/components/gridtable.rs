use crate::components::gridrow::GridRow;
use yew::prelude::*;

pub struct GridTable {
    link: ComponentLink<Self>,
    props: Props,
    vals: [usize; 5],
}

impl GridTable {}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

impl Component for GridTable {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        GridTable {
            link,
            props,
            vals: [1, 2, 3, 4, 5],
        }
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
        let make_row = |val: &usize| {
            html! { <GridRow val=val /> }
        };
        html! { <table>
        { for self.vals.iter().map(make_row)}
        </table>}
    }
}
