use bootstrap::main_container;
use grids::{Color, GridId, GridTrait, MetaGrid, SimpleGrid};
use renderer::decorators::{
    BorderedCellDecorator, ColorDecorator, PrintableColorDecorator, RegularSizedTableDecorator,
};
use renderer::TableRenderer;
use yew::prelude::*;

pub struct MetapixelApp {
    link: ComponentLink<Self>,

    base_grid: SimpleGrid,
    row_grid_cols: Vec<u8>,
    col_grid_cols: Vec<u8>,
}

impl MetapixelApp {
    fn render_controls(&self) -> Html {
        bootstrap::empty()
    }

    fn render_base_grid(&self) -> Html {
        let mut renderer = TableRenderer::new(&self.base_grid);
        renderer.add_class_decorator(RegularSizedTableDecorator::default());
        renderer.add_style_decorator(ColorDecorator::default());
        renderer.add_class_decorator(BorderedCellDecorator::default());
        renderer.add_class_decorator(PrintableColorDecorator::default());
        renderer.render()
    }

    fn render_meta_grid(&self) -> Html {
        let metagrid = MetaGrid::new(
            GridId::SmallOne,
            &self.base_grid,
            &self.row_grid_cols,
            &self.col_grid_cols,
        );
        let mut renderer = TableRenderer::new(&metagrid);
        renderer.add_class_decorator(RegularSizedTableDecorator::default());
        renderer.add_style_decorator(ColorDecorator::default());
        renderer.add_class_decorator(BorderedCellDecorator::default());
        renderer.add_class_decorator(PrintableColorDecorator::default());
        renderer.render()
    }
}

impl Component for MetapixelApp {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut grid = SimpleGrid::new(GridId::Main, 5, 5);
        for row in 0..grid.num_rows() {
            for col in 0..grid.num_cols() {
                let color = if row % 2 == col % 2 {
                    Color::Orange
                } else {
                    Color::White
                };
                grid.set_cell(row, col, color);
            }
        }
        MetapixelApp {
            link,
            base_grid: grid,
            row_grid_cols: vec![3, 2, 1, 2, 3],
            col_grid_cols: vec![1, 2, 3, 2, 1],
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        main_container(html! {
            <>
                {self.render_controls()}
                {bootstrap::spacer()}
                {self.render_base_grid()}
                {bootstrap::spacer()}
                {self.render_meta_grid()}
            </>
        })
    }
}
//
// impl MetapixelApp {
//     fn new() -> Self {
//         let mut x_grid = SimpleGrid::new("left", 15, 15);
//         for coord in 0..std::cmp::min(x_grid.num_cols(), x_grid.num_rows()) {
//             x_grid.set_cell(coord, coord, Color::Red);
//             x_grid.set_cell(14 - coord, coord, Color::Green);
//         }
//
//         let mut y_grid = SimpleGrid::new("right", 15, 15);
//         for row in (0..y_grid.num_rows()).step_by(4) {
//             for col in 0..y_grid.num_cols() {
//                 y_grid.set_cell(row, col, Color::Gray);
//             }
//         }
//
//         MetapixelApp { x_grid, y_grid }
//     }
// }
//
// impl Component for MetapixelApp {
//     type Message = ();
//     type Properties = ();
//
//     fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
//         MetapixelApp::new()
//     }
//
//     fn update(&mut self, _msg: Self::Message) -> ShouldRender {
//         false
//     }
//
//     fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
//         false
//     }
//
//     fn view(&self) -> Html {
//         let combined_grid = MergedGrid::new("merged", &self.x_grid, &self.y_grid);
//         let mut combined_renderer = TableRenderer::new(&combined_grid);
//         combined_renderer.add_class_decorator(RegularSizedTableDecorator::default());
//         combined_renderer.add_style_decorator(ColorDecorator::default());
//         combined_renderer.add_class_decorator(BorderedCellDecorator::default());
//         combined_renderer.add_class_decorator(MergedBorderDecorator::default());
//         combined_renderer.set_label_decorator(MergedFlatLabels::default());
//
//         let mut renderer = TableRenderer::new(&self.x_grid);
//         renderer.add_class_decorator(RegularSizedTableDecorator::default());
//         renderer.add_style_decorator(ColorDecorator::default());
//         renderer.add_class_decorator(PrintableColorDecorator::default());
//         renderer.add_class_decorator(BorderedCellDecorator::default());
//         renderer.add_class_decorator(ThickBorders::default());
//         renderer.set_label_decorator(FlatLabels::default());
//
//         let mut y_renderer = TableRenderer::new(&self.y_grid);
//         y_renderer.add_class_decorator(RegularSizedTableDecorator::default());
//         y_renderer.add_style_decorator(ColorDecorator::default());
//         y_renderer.add_class_decorator(PrintableColorDecorator::default());
//         y_renderer.add_class_decorator(BorderedCellDecorator::default());
//         y_renderer.add_class_decorator(ThickBorders::default());
//         y_renderer.set_label_decorator(RoundLabels::default());
//
//         let mut small_x_renderer = TableRenderer::new(&self.x_grid);
//         small_x_renderer.add_class_decorator(SmallSizedTableDecorator::default());
//         small_x_renderer.add_style_decorator(ColorDecorator::default());
//         small_x_renderer.add_class_decorator(PrintableColorDecorator::default());
//         small_x_renderer.add_class_decorator(BorderedCellDecorator::default());
//
//         let inverted_grid = InvertedGrid::new("inverted", &self.y_grid);
//         let mut inverted_renderer = TableRenderer::new(&inverted_grid);
//         inverted_renderer.add_class_decorator(SmallSizedTableDecorator::default());
//         inverted_renderer.add_style_decorator(ColorDecorator::default());
//         inverted_renderer.add_class_decorator(PrintableColorDecorator::default());
//         inverted_renderer.add_class_decorator(BorderedCellDecorator::default());
//
//         html! {
//             <main class="main container">
//               {bootstrap::div!("small", "JOIJO", "boop", "boopie")}
//               {bootstrap::col!("foo", "bar", "baz", "quux")}
//               <h1>{"X grid"}</h1>
//               {renderer.render()}
//               <h1>{"Y grid"}</h1>
//               {y_renderer.render()}
//               <h1>{"combined"}</h1>
//               {combined_renderer.render()}
//               <h1>{"X small"}</h1>
//               {small_x_renderer.render()}
//               <h1>{"Y inverted"}</h1>
//               {inverted_renderer.render()}
//             </main>
//         }
//     }
// }
