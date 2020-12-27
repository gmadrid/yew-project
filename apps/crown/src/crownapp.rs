use crate::arc::arc;
use crate::point::Point;
use yew::prelude::*;

#[derive(Default)]
pub struct CrownApp {}

impl CrownApp {
    pub fn new() -> Self {
        CrownApp::default()
    }

    pub fn center(&self) -> (f64, f64) {
        (150.0, 100.0)
    }
}

pub fn new_d() -> String {
    let center = Point::coord(150.0, 100.0);
    let v1 = Point::polar_deg(50.0, 0.0) + center;
    let v2 = Point::polar_deg(50.0, 80.0) + center;

    let v3 = Point::polar_deg(70.0, 160.0) + center;
    let v4 = Point::polar_deg(70.0, -160.0) + center;

    format!(
        "{} {}",
        arc(v1, v2, center, 15.0),
        arc(v3, v4, center, 25.0)
    )
}

impl Component for CrownApp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        CrownApp::new()
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let d = new_d();
        html! {
          <svg version="1.1"
            baseProfile="full"
            width="300" height="200"
            xmlns="http://www.w3.org/2000/svg">

            <rect width=300 height=200 style="fill:#ddd"/>

            <path d={d}/>

          </svg>
        }
    }
}
