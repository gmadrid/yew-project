use crate::point::Point;
use yew::prelude::*;

pub struct CrownApp {}

impl CrownApp {
    pub fn new() -> Self {
        CrownApp {}
    }

    pub fn center(&self) -> (f64, f64) {
        (150.0, 100.0)
    }
}

pub fn arc_string(radius: f64, direction: u8, end: Point) -> String {
    format!("A {} {} 0 0 {} {}", radius, radius, direction, end)
}

pub fn arc(start: Point, end: Point, center: Point, width: f64) -> String {
    let info = yew::services::ConsoleService::info;

    let start_vec = start - center;
    let end_vec = end - center;
    info(&format!(
        "start_vec: {:?}\nend_vec: {:?}",
        start_vec, end_vec
    ));

    let direction = if  { 1 } else { 0 };
    info(&format!("direction: {}", direction));

    let radius = start_vec.r();
    let start_outer_vec = Point::polar(radius + width, start_vec.theta());
    let start_outer = start_outer_vec + center;
    let end_outer_vec = Point::polar(radius + width, end_vec.theta());
    let end_outer = end_outer_vec + center;
    info(&format!("radius: {:.3}", radius));
    format!(
        "M {} {} L {} {} Z",
        start,
        arc_string(radius, direction, end),
        end_outer,
        arc_string(radius + width, 1 - direction, start_outer)
    )
    // format!(
    //     "M {} A {} {} 0 0 {} {}",
    //     start, radius, radius, direction, end
    // )

    // let start_p = start;
    // let end_p = end;
    // let mut start_outer = start_p + Point::polar(width, start_p.theta());
    // info("END OUTER=================");
    // let mut end_outer = end_p + Point::polar(width, end_p.theta());
    // info("==========================");
    //
    // info(&format!("start_p: {:?}\n end_p: {:?}", start_p, end_p));
    // info(&format!(
    //     "start_outer_rel: {:?}\n end_outer_rel: {:?}",
    //     start_p + Point::polar(width, start_p.theta()),
    //     end_p + Point::polar(width, end_p.theta())
    // ));
    //
    // let angle = (end_p - start_p).theta();
    // info(&format!("angle: {} ({})", angle, end_p - start_p));
    // if angle < 0.0 {
    //     info("Swapping");
    //     std::mem::swap(&mut start, &mut end);
    //     std::mem::swap(&mut start_outer, &mut end_outer);
    // }
    // info(&format!("FOO: {}   {}", start, end));
    //
    // format!(
    //     "M {} A 50 50 0 0 0 {} L {} A 65 65 0 0 1 {} Z",
    //     start, end, end_outer, start_outer
    // )
}

pub fn new_d() -> String {
    let center = Point::coord(150.0, 100.0);
    let v1 = Point::polar_deg(50.0, 0.0) + center;
    let v2 = Point::polar_deg(50.0, 80.0) + center;

    let v3 = Point::polar_deg(70.0, 200.0) + center;
    let v4 = Point::polar_deg(70.0, 250.0) + center;

    format!(
        "{} {}",
        arc(v1, v2, center, 15.0),
        arc(v3, v4, center, 25.0)
    )
}

// pub fn make_d() -> String {
//     let info = yew::services::ConsoleService::info;
//     let center = Point::coord(0.0, 0.0);
//     let v1 = Point::polar_deg(50.0, 0.0);
//     info(&format!("v1: {:?}", v1));
//     let v2 = Point::polar_deg(65.0, 0.0);
//     let v3 = Point::polar_deg(65.0, 80.0);
//     info("V4 =====================");
//     let v4 = Point::polar_deg(50.0, 80.0);
//     info(&format!("v4: {:?}", v4));
//     info("========================");
//     let vx = Point::polar_deg(50.0, 80.0);
//
//     let arc1 = format!("A 65 65 0 0 1 {}", v3);
//     let arc2 = format!("A 50 50 0 0 0 {}", v1);
//     let d = format!("M {} L {} {} L {} {}", v1, v2, arc1, v4, arc2);
//
//     format!(
//         " {}",
//         //arc(v1, v4, center, 15.0),
//         arc(v1, v4, 15.0)
//     )
// }

impl Component for CrownApp {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        CrownApp::new()
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let d = new_d();
        html! {
          <svg version="1.1"
            baseProfile="full"
            width="300" height="200"
            xmlns="http://www.w3.org/2000/svg">

            <rect width=300 height=200 style="fill:#ddd"/>

            <path d={d}/>

          </svg>
        }
    }
}
