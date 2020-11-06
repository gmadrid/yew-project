use yew::prelude::{html, Html};

pub fn row(content_fn: impl FnOnce() -> Html) -> Html {
    html! {
        <div class="row">
            {content_fn()}
        </div>
    }
}

pub fn spacer() -> Html {
    row(|| {
        html! {
            <div class="col" style="min-height: 2em"></div>
        }
    })
}

pub fn maybe(txt: Option<&str>, content_fn: impl FnOnce(&str) -> Html) -> Html {
    if let Some(txt) = txt {
        html! {
            {content_fn(txt)}
        }
    } else {
        html! {}
    }
}

pub fn card(header: &str, subtitle: &str, body_fn: impl FnOnce() -> Html) -> Html {
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
              {body_fn()}
            </div>
        </div>
    }
    //              <h6 class="card-subtitle">{subtitle}</h6>
}

pub fn ul(items: &[&str]) -> Html {
    html! {
        <ul>
          { for items.iter().map(|s| { html! { <li>{s}</li> }})}
        </ul>
    }
}
