use yew::prelude::*;

pub fn main_container(contents: impl Into<Html>) -> Html {
    html! {
      <>
        {spacer()}
        {contents.into()}
      </>
    }
}

pub fn spacer() -> Html {
    row(html! {
        <div class="col" style="min-height: 2em"></div>
    })
}

pub fn row(content: impl Into<Html>) -> Html {
    div(&["row"], content)
}

pub fn div(classes: &[&str], content: impl Into<Html>) -> Html {
    html! {
        <div class=classes>
          {content.into()}
        </div>
    }
}

pub fn empty() -> Html {
    html! {}
}
