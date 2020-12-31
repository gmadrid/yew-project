use std::f64::consts::PI;

#[derive(Clone, Copy, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
    r: f64,
    theta: f64,
}

impl Point {
    pub fn coord(x: f64, y: f64) -> Point {
        let mut theta = (y / x).atan();
        if x < 0.0 {
            if y < 0.0 {
                theta -= PI;
            } else {
                theta += PI;
            }
        }
        Point {
            x,
            y,
            r: (x * x + y * y).sqrt(),
            theta,
        }
    }

    pub fn polar(r: f64, theta: f64) -> Point {
        Point {
            x: r * theta.cos(),
            y: r * theta.sin(),
            r,
            theta,
        }
    }

    pub fn polar_deg(r: f64, theta_deg: f64) -> Point {
        Point::polar(r, theta_deg.to_radians())
    }

    // pub fn x(&self) -> f64 {
    //     self.x
    // }
    //
    // pub fn y(&self) -> f64 {
    //     self.y
    // }
    //
    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn theta(&self) -> f64 {
        self.theta
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Point::coord(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Point::coord(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.5} {:.5}", self.x, self.y)
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("r", &self.r)
            .field("theta", &self.theta)
            .field("theta_deg", &(self.theta * 180.0 / PI))
            .finish()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn close_enough(f1: f64, f2: f64) {
        let abs_diff = (f1 - f2).abs();
        if !(abs_diff < 1.0e-10) {
            assert_eq!(f1, f2);
        }
    }

    #[test]
    fn test_quad1() {
        let point = Point::coord(3.0f64.sqrt(), 1.0);
        close_enough(2.0, point.r());
        close_enough(PI / 6.0, point.theta());

        let point = Point::polar(2.0, 30.0f64.to_radians());
        close_enough(3.0f64.sqrt(), point.x());
        close_enough(1.0, point.y());
    }

    #[test]
    fn test_quad2() {
        let point = Point::coord(-2.0, 2.0 * 3.0f64.sqrt());
        close_enough(4.0, point.r());
        close_enough(PI * 2.0 / 3.0, point.theta());

        let point = Point::polar(4.0, 120.0f64.to_radians());
        close_enough(-2.0, point.x());
        close_enough(2.0 * 3.0f64.sqrt(), point.y());
    }

    #[test]
    fn test_quad3() {
        let point = Point::coord(-1.0, -3.0f64.sqrt());
        close_enough(2.0, point.r());
        close_enough(-120f64.to_radians(), point.theta());

        let point = Point::polar(4.0, -120.0f64.to_radians());
        close_enough(-2.0, point.x());
        close_enough(-2.0 * 3f64.sqrt(), point.y());
    }

    #[test]
    fn test_quad4() {
        let point = Point::coord(3f64.sqrt(), -1.0);
        close_enough(2.0, point.r());
        close_enough(-30f64.to_radians(), point.theta());

        let point = Point::polar(2.0, -60f64.to_radians());
        close_enough(1.0, point.x());
        close_enough(-3f64.sqrt(), point.y());
    }
}
