use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

pub struct InputComponent {
    link: ComponentLink<Self>,
    callback: Callback<Vec<u8>>,

    input_ref: NodeRef,
    err_ref: NodeRef,

    meta_vec: Vec<u8>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<Vec<u8>>,
}

pub enum Msg {
    ChangeEvent,
}

impl InputComponent {}

fn parse_input(input: &str) -> Result<Vec<u8>, String> {
    let bad_chars = input.find(|ch: char| !ch.is_whitespace() && !ch.is_ascii_digit() && ch != ',');
    if bad_chars.is_some() {
        return Err("Input can only contain digits, commas, and white space.".to_owned());
    }

    let pieces = input.split(',').map(str::trim);
    let num_result_pieces = pieces.into_iter().map(str::parse::<u8>).collect::<Vec<_>>();
    if let Some(err) = num_result_pieces.iter().find(|res| res.is_err()) {
        return Err(err.clone().unwrap_err().to_string());
    }
    // unwrap: should be okay, because any errs were caught by above if statement.
    Ok(num_result_pieces
        .into_iter()
        .map(|result| result.unwrap())
        .collect::<Vec<_>>())
}

impl Component for InputComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        InputComponent {
            link,
            callback: props.callback,
            input_ref: Default::default(),
            err_ref: Default::default(),
            meta_vec: Default::default(),
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        yew::services::ConsoleService::log("in update()");
        let val = self
            .input_ref
            .cast::<HtmlInputElement>()
            .expect("no expect")
            .value();
        yew::services::ConsoleService::log(&format!("got value: {}", val));

        let result = parse_input(&val);
        let err_element = self.err_ref.cast::<HtmlElement>().expect("no err element");

        yew::services::ConsoleService::log(&format!("'{:?}' result", result));
        match result {
            Ok(vec) => {
                err_element.set_inner_text("");
                self.meta_vec = vec;
                self.callback.emit(self.meta_vec.clone());
            }
            Err(err_text) => {
                err_element.set_inner_text(&err_text);
                yew::services::ConsoleService::log("got an error");
            }
        }

        false
    }

    fn change(&mut self, props: <Self as yew::Component>::Properties) -> bool {
        if self.callback != props.callback {
            self.callback = props.callback;
        }

        // We work hard to never rerender this.
        false
    }

    fn view(&self) -> Html {
        yew::services::ConsoleService::log("RENDERING INPUT");

        let change_callback = self.link.callback(|_| {
            yew::services::ConsoleService::log("in change_callback");
            Msg::ChangeEvent
        });

        html! {
          <>
            <input ref=self.input_ref.clone() onchange=change_callback type="text"/>
            <span ref=self.err_ref.clone() class=".errspan"></span>
          </>
        }
    }
}
