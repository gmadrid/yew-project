use yew::prelude::{html, Html};

pub fn h4(content: impl Into<Html>) -> Html {
    html! {<h4>{content}</h4>}
}

pub fn h5(content: impl Into<Html>) -> Html {
    html! {<h5>{content}</h5>}
}

pub fn empty() -> Html {
    html! {}
}

pub fn concat(first_html: Html, second_html: Html) -> Html {
    html! {
        <>
          {first_html}
          {second_html}
        </>
    }
}

pub fn concat_spaced(first_html: Html, second_html: Html) -> Html {
    html! {
        <>
          {first_html}
          {spacer()}
          {second_html}
        </>
    }
}

pub fn row(content: Html) -> Html {
    div(&["row"], content)
}

pub fn col(content: Html) -> Html {
    div(&["col"], content)
}

pub fn div(classes: &[&str], content: Html) -> Html {
    html! {
        <div class=classes>
          {content}
        </div>
    }
}

pub fn spacer() -> Html {
    row(html! {
        <div class="col" style="min-height: 2em"></div>
    })
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

/*
pub fn ul(items: &[&str]) -> Html {
    html! {
        <ul>
          { for items.iter().map(|s| { html! { <li>{s}</li> }})}
        </ul>
    }
}
*/
