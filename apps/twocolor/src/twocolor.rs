use yew::prelude::*;

#[derive(Default)]
pub struct TwoColorApp;

impl Component for TwoColorApp {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        TwoColorApp::default()
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {}
    }
}
