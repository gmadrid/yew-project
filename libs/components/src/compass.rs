use yew::prelude::*;

pub struct Compass {
    link: ComponentLink<Self>,
    props: CompassProperties,
}

impl Compass {}

#[derive(Clone, Copy, Debug)]
pub enum CompassDirection {
    Down,
    Left,
    Right,
    Up,
}

#[derive(Clone, Properties)]
pub struct CompassProperties {
    pub callback: Callback<CompassDirection>,

    #[prop_or(true)]
    pub horiz: bool,
    #[prop_or(true)]
    pub vert: bool,
}

impl Compass {
    fn display_button(&self, code_point: u32, direction: CompassDirection) -> Html {
        let cb = self.link.callback(move |_| direction);
        let str = std::char::from_u32(code_point).unwrap().to_string();

        html! {
            <button onclick=cb class="btn btn-outline-primary">{str}</button>
        }
    }

    fn display_north(&self) -> Html {
        if !self.props.vert {
            return html! {};
        }
        self.display_button(0x2191, CompassDirection::Up)
    }
    fn display_west(&self) -> Html {
        if !self.props.horiz {
            return html! {};
        }
        self.display_button(0x2190, CompassDirection::Left)
    }
    fn display_east(&self) -> Html {
        if !self.props.horiz {
            return html! {};
        }
        self.display_button(0x2192, CompassDirection::Right)
    }
    fn display_south(&self) -> Html {
        if !self.props.vert {
            return html! {};
        }
        self.display_button(0x2193, CompassDirection::Down)
    }
}

impl Component for Compass {
    type Message = CompassDirection;
    type Properties = CompassProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Compass { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.props.callback.emit(msg);
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <div class="btn_group" role="group">
            {self.display_west()}
            {self.display_north()}
            {self.display_south()}
            {self.display_east()}
          </div>
        }
    }
}
