use crate::point::Point;

use std::f64::consts::PI;

/// Function for creating SVG arc strings to use in <path> elements.
pub fn arc(start: Point, end: Point, center: Point, width: f64) -> String {
    // Start and end relative to center.
    let start_vec = start - center;
    let end_vec = end - center;

    let direction = find_arc_direction(start_vec, end_vec);

    // Compute the radius of the arcs, then using that radius, compute the location of
    // the corners (endpoints) of the outer edge of the arc.
    let radius = start_vec.r();
    let start_outer_vec = Point::polar(radius + width, start_vec.theta());
    let end_outer_vec = Point::polar(radius + width, end_vec.theta());

    // Now compute the corners (endpoints) of the outer edge of the arc in absolute coords.
    let start_outer = start_outer_vec + center;
    let end_outer = end_outer_vec + center;

    format!(
        "M {} {} L {} {} Z",
        start,
        arc_string(radius, direction, end),
        end_outer,
        arc_string(radius + width, 1 - direction, start_outer)
    )
}

/// Determines the correct value of the "direction" parameter to an SVG `A` path command.
/// Inputs are vectors from the "center" to the start and end points respectively.
fn find_arc_direction(start_vec: Point, end_vec: Point) -> u8 {
    let start_theta = start_vec.theta();
    let mut end_theta = end_vec.theta();

    // Adjust end_theta, since end_theta will be < start_theta if the arc crosses 180 deg.
    while end_theta < start_theta {
        end_theta += 2.0 * PI;
    }

    // Now, choose the shorter arc.
    if end_theta - start_theta > PI {
        0
    } else {
        1
    }
}

/// Creates an "arc string" for use in an SVG `path` element.
/// Assumes an arc along a circle that is less than 180 deg.
pub fn arc_string(radius: f64, direction: u8, end: Point) -> String {
    format!("A {} {} 0 0 {} {}", radius, radius, direction, end)
}
