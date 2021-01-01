use grids::{CellId, GridId, GridTrait, HasGrids, InvertedGrid, MergedGrid, SimpleGrid};
use renderer::decorators::{
    FlatLabels, MergedBorderDecorator, MergedFlatLabels, RoundLabels, ThickBorders,
};
use renderer::interact::{Interactions, Interactor, OneColorInteractor};
use renderer::TableRenderer;
use std::convert::TryFrom;
use yew::prelude::*;
use yew::prelude::{html, Html};

pub struct TestApp {
    link: ComponentLink<Self>,

    interact: OneColorInteractor,

    x_grid: SimpleGrid,
    y_grid: SimpleGrid,
}

pub enum Message {
    Down(CellId),
    Up(CellId),
    Enter(CellId),
    Leave(CellId),
}

impl Message {
    fn cell_id(&self) -> CellId {
        match self {
            Message::Down(cell_id) => *cell_id,
            Message::Enter(cell_id) => *cell_id,
            Message::Leave(cell_id) => *cell_id,
            Message::Up(cell_id) => *cell_id,
        }
    }
}

impl From<Interactions> for Message {
    fn from(i: Interactions) -> Message {
        match i {
            Interactions::MouseDown(cell_id) => Message::Down(cell_id),
            Interactions::MouseUp(cell_id) => Message::Up(cell_id),
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
            Message::Up(cell_id) => Ok(Interactions::MouseUp(*cell_id)),
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
        }
    }

    // TODO: can we get rid of this uglyness?
    fn grid_with_interact(&mut self, id: GridId) -> (&mut dyn GridTrait, &mut OneColorInteractor) {
        (
            match id {
                GridId::LayerOne => &mut self.x_grid,
                GridId::LayerTwo => &mut self.y_grid,
                _ => panic!("Bad mapping for grid id: {:?}", id),
            },
            &mut self.interact,
        )
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

impl Component for TestApp {
    type Message = Message;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TestApp::new(link)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let (grid, interact) = self.grid_with_interact(msg.cell_id().grid_id);
        if let Some(should_render) = interact.update(grid, &msg) {
            return should_render;
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
        self.interact.install(&self.link, &mut combined_renderer);

        let mut renderer = TableRenderer::regular_renderer(&self.x_grid);
        renderer.add_class_decorator(ThickBorders::default());
        renderer.set_label_decorator(FlatLabels::default());
        self.interact.install(&self.link, &mut renderer);

        let mut y_renderer = TableRenderer::regular_renderer(&self.y_grid);
        y_renderer.add_class_decorator(ThickBorders::default());
        y_renderer.set_label_decorator(RoundLabels::default());
        self.interact.install(&self.link, &mut y_renderer);

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
