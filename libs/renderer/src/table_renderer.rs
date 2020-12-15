use crate::decorators::{ClassDecorator, CssMunger, EmptyLabels, LabelDecorator, StyleDecorator};

use grids::GridTrait;

use yew::prelude::*;

static TABLE_ONCE: std::sync::Once = std::sync::Once::new();

pub struct TableRenderer<'a, GRID: GridTrait> {
    grid: &'a GRID,

    class_decorators: Vec<Box<dyn ClassDecorator>>,
    style_decorators: Vec<Box<dyn StyleDecorator>>,
    label_decorator: Box<dyn LabelDecorator>,
}

impl<'a, GRID: GridTrait> TableRenderer<'a, GRID> {
    pub fn new(grid: &'a GRID) -> Self {
        TABLE_ONCE.call_once(|| {
            let munger = CssMunger::new();
            munger.insert_rule(".tblrdr td, .tblrdr th {line-height:1.0; vertical-align: middle; text-align: center }")
        });
        TableRenderer {
            grid,
            class_decorators: Default::default(),
            style_decorators: Default::default(),
            label_decorator: Box::from(EmptyLabels::default()),
        }
    }

    pub fn add_class_decorator(&mut self, class_decorator: impl ClassDecorator + 'static) {
        let munger = CssMunger::new();
        class_decorator.register(&munger);

        let boxx: Box<dyn ClassDecorator> = Box::from(class_decorator);
        self.class_decorators.push(boxx);
    }

    pub fn add_style_decorator(&mut self, style_decorator: impl StyleDecorator + 'static) {
        let boxx: Box<dyn StyleDecorator> = Box::from(style_decorator);
        self.style_decorators.push(boxx);
    }

    pub fn set_label_decorator(&mut self, label_decorator: impl LabelDecorator + 'static) {
        let boxx: Box<dyn LabelDecorator> = Box::from(label_decorator);
        self.label_decorator = boxx;
    }

    fn render_left_label(&self, row: usize) -> Html {
        if let Some((content, classes)) = self.label_decorator.left(self.grid, row) {
            html! {<th class=classes><small>{content}</small></th>}
        } else {
            html! {}
        }
    }

    fn render_right_label(&self, row: usize) -> Html {
        if let Some((content, mut classes)) = self.label_decorator.right(self.grid, row) {
            for decorator in &self.class_decorators {
                classes.append(&mut decorator.label_class(self.grid, row));
            }
            html! {<th class=classes><small>{content}</small></th>}
        } else {
            html! {}
        }
    }

    fn render_full_row(&self, row: usize) -> Html {
        html! {
            <tr>
              {self.render_left_label(row)}
              {for (0..self.grid.num_cols()).map(|cn| {
                  {self.render_cell(row, cn)}
              })}
              {self.render_right_label(row)}
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
            classes.append(&mut decorator.cell_class(self.grid, row, col, contents))
        }

        html! {
            <td class=classes style=style_string></td>
        }
    }

    pub fn render(&self) -> Html {
        html! {
          <table class="tblrdr">
            {for (0..self.grid.num_rows()).map(|rn| {
                {self.render_full_row(rn)}
            })}
          </table>
        }
    }
}
