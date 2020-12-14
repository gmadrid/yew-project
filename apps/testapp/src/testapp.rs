use grids::{Color, GridTrait, SimpleGrid};
use renderer::TableRenderer;
use yew::prelude::*;
use yew::prelude::{html, Html};

pub struct TestApp {
    x_grid: SimpleGrid,
}

impl TestApp {
    fn new() -> Self {
        let mut x_grid = SimpleGrid::new(10, 15);
        for coord in 0..10 {
            x_grid.set_cell(coord, coord, Color::Red);
            x_grid.set_cell(9 - coord, coord + 5, Color::Green);
        }
        TestApp { x_grid }
    }
}

impl Component for TestApp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        TestApp::new()
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let renderer = TableRenderer::new(&self.x_grid);

        html! {
            <main class="main container">
              <h1>{"X grid"}</h1>
              {renderer.render()}
            </main>
        }
    }
}
