use yew::prelude::*;

pub struct Compass {
    link: ComponentLink<Self>,
    props: CompassProperties,
}

impl Compass {}

#[derive(Debug)]
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
    fn display_north(&self) -> Html {
        let cb = self.link.callback(|_| CompassDirection::Up);
        if self.props.vert {
            html! {
                <button onclick=cb>{"NORTH"}</button>
            }
        } else {
            html! {}
        }
    }
    fn display_west(&self) -> Html {
        let cb = self.link.callback(|_| CompassDirection::Left);
        if self.props.horiz {
            html! {
                <button onclick=cb>{"WEST"}</button>
            }
        } else {
            html! {}
        }
    }
    fn display_east(&self) -> Html {
        let cb = self.link.callback(|_| CompassDirection::Right);
        if self.props.horiz {
            html! {
                <button onclick=cb>{"EAST"}</button>
            }
        } else {
            html! {}
        }
    }
    fn display_south(&self) -> Html {
        let cb = self.link.callback(|_| CompassDirection::Down);
        if self.props.vert {
            html! {
                <button onclick=cb>{"SOUTH"}</button>
            }
        } else {
            html! {}
        }
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
          <>
            {self.display_north()}
            {self.display_west()}
            {self.display_east()}
            {self.display_south()}
          </>
        }
    }
}
