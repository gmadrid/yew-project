use yew::prelude::*;

const VERSION_NUMBER: &str = env!("CARGO_PKG_VERSION");

pub struct PROJECTApp {}

impl Component for PROJECTApp {
    type Properties = ();
    type Message = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        PROJECTApp {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            {format!("Version: {}", VERSION_NUMBER)}
        }
    }
}
