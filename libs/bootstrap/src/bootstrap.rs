use yew::prelude::*;

pub fn main_container(contents: impl Into<Html>) -> Html {
    html! {
      <main class="main container">
        {spacer()}
        {contents.into()}
      </main>
    }
}

pub fn h5(content: impl Into<Html>) -> Html {
    html! {<h5>{content}</h5>}
}

pub fn concat(first_html: Html, second_html: Html) -> Html {
    html! {
        <>
          {first_html}
          {second_html}
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

pub fn col(content: impl Into<Html>) -> Html {
    div(&["col"], content)
}

pub fn div(classes: &[&str], content: impl Into<Html>) -> Html {
    html! {
        <div class=classes>
          {content.into()}
        </div>
    }
}

pub fn maybe(txt: Option<&str>, content_fn: impl FnOnce(&str) -> Html) -> Html {
    if let Some(txt) = txt {
        content_fn(txt)
    } else {
        empty()
    }
}

pub fn card(header: impl Into<Html>, subtitle: &str, body: Html) -> Html {
    let maybe_str = if subtitle.is_empty() {
        None
    } else {
        Some(subtitle)
    };
    html! {
        <div class="card">
            <h4 class="card-header">{header}</h4>
            <div class="card-body">
              {maybe(maybe_str, |s| {
                  html! {
                      <h6 class="card-subtitle">{s}</h6>
                  }
              })}
              {body}
            </div>
        </div>
    }
}
pub fn empty() -> Html {
    html! {}
}
