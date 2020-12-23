use crate::decorators::{
    BorderedCellDecorator, ClassDecorator, ColorDecorator, CssMunger, EmptyLabels, LabelDecorator,
    PrintableColorDecorator, RegularSizedTableDecorator, SmallSizedTableDecorator, StyleDecorator,
};

use grids::{CellId, GridTrait};

use yew::prelude::*;

static TABLE_ONCE: std::sync::Once = std::sync::Once::new();

pub struct TableRenderer<'a, GRID: GridTrait> {
    grid: &'a GRID,

    mdown: Callback<CellId>,
    mup: Callback<CellId>,
    menter: Callback<CellId>,
    mleave: Callback<CellId>,

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
            mdown: Callback::noop(),
            mup: Callback::noop(),
            menter: Callback::noop(),
            mleave: Callback::noop(),
            class_decorators: Default::default(),
            style_decorators: Default::default(),
            label_decorator: Box::from(EmptyLabels::default()),
        }
    }

    /// Creates a new renderer with the basic decorations: colored cells, printable, with borders.
    /// NOTE: this renderer will not display properly because it has no size.
    /// See small_renderer(), and regular_renderer().
    pub fn base_renderer(grid: &'a GRID) -> Self {
        let mut renderer = TableRenderer::new(grid);
        renderer.add_style_decorator(ColorDecorator::default());
        renderer.add_class_decorator(PrintableColorDecorator::default());
        renderer.add_class_decorator(BorderedCellDecorator::default());
        renderer
    }

    pub fn regular_renderer(grid: &'a GRID) -> Self {
        let mut renderer = Self::base_renderer(grid);
        renderer.add_class_decorator(RegularSizedTableDecorator::default());
        renderer
    }

    pub fn small_renderer(grid: &'a GRID) -> Self {
        let mut renderer = Self::base_renderer(grid);
        renderer.add_class_decorator(SmallSizedTableDecorator::default());
        renderer
    }

    pub fn set_interactions(
        &mut self,
        mdown: Callback<CellId>,
        mup: Callback<CellId>,
        menter: Callback<CellId>,
        mleave: Callback<CellId>,
    ) {
        self.mdown = mdown;
        self.mup = mup;
        self.menter = menter;
        self.mleave = mleave;
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
        label_decorator.register(&CssMunger::new());
        let boxx: Box<dyn LabelDecorator> = Box::from(label_decorator);
        self.label_decorator = boxx;
    }

    fn render_left_label(&self, row: usize) -> Html {
        if let Some((content, mut classes)) = self.label_decorator.left(self.grid, row) {
            for decorator in &self.class_decorators {
                classes.append(&mut decorator.label_class(self.grid, row));
            }
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

    fn th_with_colspan(&self, colspan: usize, classes: &[&str], content: impl Into<Html>) -> Html {
        if colspan == 1 {
            html! {<th class=classes>{content}</th>}
        } else {
            html! {<th colspan=colspan class=classes>{content}</th>}
        }
    }

    fn render_footer_cell(&self, col: usize) -> Html {
        if let Some((content, colspan, mut classes)) = self.label_decorator.bot(self.grid, col) {
            for decorator in &self.class_decorators {
                classes.append(&mut decorator.label_class(self.grid, col));
            }
            self.th_with_colspan(colspan, &classes, html! {<small>{content}</small>})
        } else {
            html! {}
        }
    }

    fn render_footer_row(&self) -> Html {
        if !self.label_decorator.has_bot() {
            return html! {};
        }

        html! {
          <tr>
            <th></th>
            { for (0..self.grid.num_cols()).map(|cn| {
                html!{{self.render_footer_cell(cn)}}
            })}
            <th></th>
          </tr>
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

        let cell_id = self.grid.cell_id(row, col);

        let cloned_cell_id = cell_id;
        let mdown_callback: Callback<_> = self.mdown.clone().reform(move |_| cloned_cell_id);

        let cloned_cell_id = cell_id;
        let mup_callback: Callback<_> = self.mup.clone().reform(move |_| cloned_cell_id);

        let cloned_cell_id = cell_id;
        let menter_callback: Callback<_> = self.menter.clone().reform(move |_| cloned_cell_id);

        let cloned_cell_id = cell_id;
        let mleave_callback: Callback<_> = self.mleave.clone().reform(move |_| cloned_cell_id);

        html! {
            <td
            onmousedown=mdown_callback
            onmouseup=mup_callback
            onmouseenter=menter_callback
            onmouseleave=mleave_callback
            class=classes style=style_string></td>
        }
    }

    pub fn render(&self) -> Html {
        html! {
          <table class="tblrdr user-select-none">
            {for (0..self.grid.num_rows()).map(|rn| {
                {self.render_full_row(rn)}
            })}
            {self.render_footer_row()}
          </table>
        }
    }
}
