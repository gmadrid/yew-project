use yew::prelude::*;

pub struct TestApp();

impl Component for TestApp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        TestApp()
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
false
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main class="main container">
            </main>
        }
    }
}
