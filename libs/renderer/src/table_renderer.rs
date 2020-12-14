use crate::decorators::{ClassDecoratorTrait, CssMunger, StyleDecorator};

use grids::GridTrait;

use yew::prelude::*;

pub struct TableRenderer<'a, GRID: GridTrait> {
    grid: &'a GRID,

    class_decorators: Vec<Box<dyn ClassDecoratorTrait>>,
    style_decorators: Vec<Box<dyn StyleDecorator>>,
}

impl<'a, GRID: GridTrait> TableRenderer<'a, GRID> {
    pub fn new(grid: &'a GRID) -> Self {
        TableRenderer {
            grid,
            class_decorators: Default::default(),
            style_decorators: Default::default(),
        }
    }

    pub fn add_class_decorator(&mut self, class_decorator: impl ClassDecoratorTrait + 'static) {
        let munger = CssMunger::new();
        class_decorator.register(&munger);

        let boxx: Box<dyn ClassDecoratorTrait> = Box::from(class_decorator);
        self.class_decorators.push(boxx);
    }

    pub fn add_style_decorator(&mut self, style_decorator: impl StyleDecorator + 'static) {
        let boxx: Box<dyn StyleDecorator> = Box::from(style_decorator);
        self.style_decorators.push(boxx);
    }

    fn render_full_row(&self, row: usize) -> Html {
        html! {
            <tr>
              {for (0..self.grid.num_cols()).map(|cn| {
                  {self.render_cell(row, cn)}
              })}
            </tr>
        }
    }

    fn render_cell(&self, row: usize, col: usize) -> Html {
        let mut style_strings = Vec::default();

        let contents = self.grid.cell(row, col);

        for decorator in &self.style_decorators {
            if let Some(mut ss) = decorator.cell_style(row, col, contents) {
                style_strings.append(&mut ss);
            }
        }

        let style_string = style_strings.join(";");

        let mut classes = vec![];
        for decorator in &self.class_decorators {
            classes.append(&mut decorator.cell_class(row, col, contents))
        }

        html! {
            <td class=classes style=style_string></td>
        }
    }

    pub fn render(&self) -> Html {
        html! {
          <table>
            {for (0..self.grid.num_rows()).map(|rn| {
                {self.render_full_row(rn)}
            })}
          </table>
        }
    }
}
