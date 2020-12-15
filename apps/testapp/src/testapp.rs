use grids::{Color, GridTrait, SimpleGrid};
use renderer::decorators::{
    BorderedCellDecorator, ColorDecorator, FlatLabels, PrintableColorDecorator,
    RegularSizedTableDecorator, ThickBorders,
};
use renderer::TableRenderer;
use yew::prelude::*;
use yew::prelude::{html, Html};

pub struct TestApp {
    x_grid: SimpleGrid,
}

impl TestApp {
    fn new() -> Self {
        let mut x_grid = SimpleGrid::new(22, 18);
        for coord in 0..14 {
            x_grid.set_cell(coord, coord, Color::Red);
            x_grid.set_cell(14 - coord, coord + 5, Color::Green);
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
        let mut renderer = TableRenderer::new(&self.x_grid);

        renderer.add_class_decorator(RegularSizedTableDecorator::default());
        renderer.add_style_decorator(ColorDecorator::default());
        renderer.add_class_decorator(PrintableColorDecorator::default());
        renderer.add_class_decorator(BorderedCellDecorator::default());
        renderer.add_class_decorator(ThickBorders::default());
        renderer.set_label_decorator(FlatLabels::default());

        html! {
            <main class="main container">
              <h1>{"X grid"}</h1>
              {renderer.render()}
            </main>
        }
    }
}
