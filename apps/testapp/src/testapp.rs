use grids::{CellId, Color, GridId, GridTrait, HasGrids, InvertedGrid, MergedGrid, SimpleGrid};
use renderer::decorators::{
    FlatLabels, MergedBorderDecorator, MergedFlatLabels, RoundLabels, ThickBorders,
};
use renderer::interact::{Interactions, Interactor, OneColorInteractor};
use renderer::TableRenderer;
use std::convert::TryFrom;
use yew::html::Scope;
use yew::prelude::*;
use yew::prelude::{html, Html};

pub struct TestApp {
    link: ComponentLink<Self>,

    interact: OneColorInteractor<Self, SimpleGrid>,

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

impl From<Interactions> for Message {
    fn from(i: Interactions) -> Message {
        match i {
            Interactions::MouseDown(cell_id) => Message::Down(cell_id),
            Interactions::MouseUp => Message::Up,
            Interactions::MouseEnter(cell_id) => Message::Enter(cell_id),
            Interactions::MouseExit(cell_id) => Message::Leave(cell_id),
        }
    }
}

impl TryFrom<&Message> for Interactions {
    type Error = ();

    fn try_from(msg: &Message) -> Result<Self, Self::Error> {
        match msg {
            Message::Down(cell_id) => Ok(Interactions::MouseDown(*cell_id)),
            Message::Up => Ok(Interactions::MouseUp),
            Message::Enter(cell_id) => Ok(Interactions::MouseEnter(*cell_id)),
            Message::Leave(cell_id) => Ok(Interactions::MouseExit(*cell_id)),
            // _ => Err(())
        }
    }
}

impl TestApp {
    fn new(link: ComponentLink<Self>) -> Self {
        let x_grid = SimpleGrid::new(GridId::LayerOne, 15, 15);
        let y_grid = SimpleGrid::new(GridId::LayerTwo, 15, 15);

        TestApp {
            link,
            interact: OneColorInteractor::new(),
            x_grid,
            y_grid,
            current_value: None,
        }
    }

    fn make_msg_callback(&self, f: impl Fn(CellId) -> Message + 'static) -> Callback<CellId> {
        make_app_callback(&self.link, move |link, cell_id| {
            link.send_message(f(cell_id));
        })
    }
}

impl HasGrids for TestApp {
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
        let interact = &mut self.interact;
        if let Some(render) = interact.update(self, &msg) {
            yew::services::ConsoleService::info("WHAAAAAAAA");
            return render;
        }

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
        let mut combined_renderer = TableRenderer::regular_renderer(&combined_grid);
        combined_renderer.add_class_decorator(MergedBorderDecorator::default());
        combined_renderer.set_label_decorator(MergedFlatLabels::default());
        combined_renderer.set_interactions(
            self.make_msg_callback(Message::Down),
            self.make_msg_callback(|_| Message::Up),
            self.make_msg_callback(Message::Enter),
            self.make_msg_callback(Message::Leave),
        );

        let mut renderer = TableRenderer::regular_renderer(&self.x_grid);
        renderer.add_class_decorator(ThickBorders::default());
        renderer.set_label_decorator(FlatLabels::default());
        self.interact.install(&self.link, &mut renderer);

        //OneColorInteractor::install(&mut renderer);
        // renderer.set_interactions(
        //     self.make_msg_callback(Message::Down),
        //     self.make_msg_callback(|_| Message::Up),
        //     self.make_msg_callback(Message::Enter),
        //     self.make_msg_callback(Message::Leave),
        // );

        let mut y_renderer = TableRenderer::regular_renderer(&self.y_grid);
        y_renderer.add_class_decorator(ThickBorders::default());
        y_renderer.set_label_decorator(RoundLabels::default());
        y_renderer.set_interactions(
            self.make_msg_callback(Message::Down),
            self.make_msg_callback(|_| Message::Up),
            self.make_msg_callback(Message::Enter),
            self.make_msg_callback(Message::Leave),
        );

        let small_x_renderer = TableRenderer::small_renderer(&self.x_grid);

        let inverted_grid = InvertedGrid::new(GridId::SmallTwo, &self.y_grid);
        let inverted_renderer = TableRenderer::small_renderer(&inverted_grid);

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
