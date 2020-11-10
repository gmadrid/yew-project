use crate::app::GridId;
use yew::prelude::*;

pub struct GridCell {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {
    Click,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub val: bool,
    pub row: usize,
    pub col: usize,
    pub onclick: Callback<(GridId, usize, usize)>,
}

impl GridCell {}

impl Component for GridCell {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        yew::services::ConsoleService::log("MAKE A CELL");
        GridCell { link, props }
    }

    fn update(&mut self, msg: Msg) -> bool {
        match msg {
            Click => {
                self.props
                    .onclick
                    .emit((GridId::Front, self.props.row, self.props.col));
                false
            }
        }
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
        let cls = if self.props.val { "on" } else { "off" };

        let row = self.props.row;
        let col = self.props.col;
        let onclick_callback = self.props.onclick.reform(move |_| {
            //yew::services::ConsoleService::log("THIS IS A LOG");
            (GridId::Front, row, col)
        });
        html! { <td onclick=onclick_callback class=cls></td>}
    }
}
