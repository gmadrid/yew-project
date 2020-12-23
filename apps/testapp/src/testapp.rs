use grids::{CellId, Color, GridId, GridTrait, InvertedGrid, MergedGrid, SimpleGrid};
use renderer::decorators::{
    BorderedCellDecorator, ColorDecorator, FlatLabels, MergedBorderDecorator, MergedFlatLabels,
    PrintableColorDecorator, RegularSizedTableDecorator, RoundLabels, SmallSizedTableDecorator,
    ThickBorders,
};
use renderer::TableRenderer;
use yew::html::Scope;
use yew::prelude::*;
use yew::prelude::{html, Html};

pub struct TestApp {
    link: ComponentLink<Self>,

    x_grid: SimpleGrid,
    y_grid: SimpleGrid,

    current_value: Option<Color>,
}

pub enum Message {
    Down(CellId),
    Up,
    Enter(CellId),
    Leave(CellId),
}

impl TestApp {
    fn new(link: ComponentLink<Self>) -> Self {
        let mut x_grid = SimpleGrid::new(GridId::LayerOne, 15, 15);
        for coord in 0..std::cmp::min(x_grid.num_cols(), x_grid.num_rows()) {
            x_grid.set_cell(coord, coord, Color::Red);
            x_grid.set_cell(14 - coord, coord, Color::Green);
        }

        let mut y_grid = SimpleGrid::new(GridId::LayerTwo, 15, 15);
        for row in (0..y_grid.num_rows()).step_by(4) {
            for col in 0..y_grid.num_cols() {
                y_grid.set_cell(row, col, Color::Gray);
            }
        }

        TestApp {
            link,
            x_grid,
            y_grid,
            current_value: None,
        }
    }

    fn grid_for_id(&self, id: GridId) -> &dyn GridTrait {
        match id {
            GridId::LayerOne => &self.x_grid,
            GridId::LayerTwo => &self.y_grid,
            _ => panic!("Bad mapping for grid id: {:?}", id),
        }
    }

    fn grid_for_id_mut(&mut self, id: GridId) -> &mut dyn GridTrait {
        match id {
            GridId::LayerOne => &mut self.x_grid,
            GridId::LayerTwo => &mut self.y_grid,
            _ => panic!("Bad mapping for grid id: {:?}", id),
        }
    }

    fn make_msg_callback(&self, f: impl Fn(CellId) -> Message + 'static) -> Callback<CellId> {
        make_app_callback(&self.link, move |link, cell_id| {
            link.send_message(f(cell_id));
        })
    }
}

fn make_app_callback<C: Component>(
    link: &ComponentLink<C>,
    f: impl Fn(&Scope<C>, CellId) + 'static,
) -> Callback<CellId> {
    let scope = link.clone();
    let callback = move |cell_id: CellId| {
        f(&scope, cell_id);
    };
    callback.into()
}

impl Component for TestApp {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TestApp::new(link)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let info = yew::services::ConsoleService::info;
        match msg {
            Message::Down(cell_id) => {
                let grid = self.grid_for_id_mut(cell_id.grid_id);
                let value = !grid.cell(cell_id.row, cell_id.col);
                grid.set_cell(cell_id.row, cell_id.col, value);
                self.current_value = Some(value.clone());
                return true;
            }
            Message::Up => self.current_value = None,
            Message::Enter(cell_id) => {
                if let Some(value) = self.current_value {
                    let grid = self.grid_for_id_mut(cell_id.grid_id);
                    grid.set_cell(cell_id.row, cell_id.col, value);
                    return true;
                }
            }
            Message::Leave(_cell_id) => {}
        }
        false
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let combined_grid = MergedGrid::new(GridId::Combined, &self.x_grid, &self.y_grid);
        let mut combined_renderer = TableRenderer::new(&combined_grid);
        combined_renderer.add_class_decorator(RegularSizedTableDecorator::default());
        combined_renderer.add_style_decorator(ColorDecorator::default());
        combined_renderer.add_class_decorator(BorderedCellDecorator::default());
        combined_renderer.add_class_decorator(MergedBorderDecorator::default());
        combined_renderer.set_label_decorator(MergedFlatLabels::default());
        combined_renderer.set_interactions(
            self.make_msg_callback(Message::Down),
            self.make_msg_callback(|_| Message::Up),
            self.make_msg_callback(Message::Enter),
            self.make_msg_callback(Message::Leave),
        );

        let mut renderer = TableRenderer::new(&self.x_grid);
        renderer.add_class_decorator(RegularSizedTableDecorator::default());
        renderer.add_style_decorator(ColorDecorator::default());
        renderer.add_class_decorator(PrintableColorDecorator::default());
        renderer.add_class_decorator(BorderedCellDecorator::default());
        renderer.add_class_decorator(ThickBorders::default());
        renderer.set_label_decorator(FlatLabels::default());
        renderer.set_interactions(
            self.make_msg_callback(Message::Down),
            self.make_msg_callback(|_| Message::Up),
            self.make_msg_callback(Message::Enter),
            self.make_msg_callback(Message::Leave),
        );

        let mut y_renderer = TableRenderer::new(&self.y_grid);
        y_renderer.add_class_decorator(RegularSizedTableDecorator::default());
        y_renderer.add_style_decorator(ColorDecorator::default());
        y_renderer.add_class_decorator(PrintableColorDecorator::default());
        y_renderer.add_class_decorator(BorderedCellDecorator::default());
        y_renderer.add_class_decorator(ThickBorders::default());
        y_renderer.set_label_decorator(RoundLabels::default());
        y_renderer.set_interactions(
            self.make_msg_callback(Message::Down),
            self.make_msg_callback(|_| Message::Up),
            self.make_msg_callback(Message::Enter),
            self.make_msg_callback(Message::Leave),
        );

        let mut small_x_renderer = TableRenderer::new(&self.x_grid);
        small_x_renderer.add_class_decorator(SmallSizedTableDecorator::default());
        small_x_renderer.add_style_decorator(ColorDecorator::default());
        small_x_renderer.add_class_decorator(PrintableColorDecorator::default());
        small_x_renderer.add_class_decorator(BorderedCellDecorator::default());

        let inverted_grid = InvertedGrid::new(GridId::SmallTwo, &self.y_grid);
        let mut inverted_renderer = TableRenderer::new(&inverted_grid);
        inverted_renderer.add_class_decorator(SmallSizedTableDecorator::default());
        inverted_renderer.add_style_decorator(ColorDecorator::default());
        inverted_renderer.add_class_decorator(PrintableColorDecorator::default());
        inverted_renderer.add_class_decorator(BorderedCellDecorator::default());

        html! {
            <main class="main container">
              <h1>{"X grid"}</h1>
              {renderer.render()}
              <h1>{"Y grid"}</h1>
              {y_renderer.render()}
              <h1>{"combined"}</h1>
              {combined_renderer.render()}
              <h1>{"X small"}</h1>
              {small_x_renderer.render()}
              <h1>{"Y inverted"}</h1>
              {inverted_renderer.render()}
            </main>
        }
    }
}
