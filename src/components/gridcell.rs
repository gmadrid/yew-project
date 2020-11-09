use yew::prelude::*;

pub struct GridCell {
    link: ComponentLink<Self>,
    props: Props,
}

pub enum Msg {}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub val: usize,
}

impl GridCell {}

impl Component for GridCell {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        GridCell { link, props }
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
        html! { <td>{self.props.val}</td>}
    }
}
