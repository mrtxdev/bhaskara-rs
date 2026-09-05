use std::fmt;

pub struct Squared {
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub x1: f64,
    pub x2: f64,
    pub xv: f64,
    pub yv: f64,
}

impl Squared {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        let delta = Self::calculate_delta(a, b, c);

        if delta > 0.0 {
            Self::create_valid_results(a, b, c, delta)
        } else {
            Self::create_empty_results()
        }
    }

    fn calculate_delta(a: f64, b: f64, c: f64) -> f64 {
        (b * b) - 4.0 * a * c
    }

    fn calculate_x1(b: f64, delta_sqrt: f64, a: f64) -> f64 {
        (-b + delta_sqrt) / (2.0 * a)
    }

    fn calculate_x2(b: f64, delta_sqrt: f64, a: f64) -> f64 {
        (-b - delta_sqrt) / (2.0 * a)
    }

    fn calculate_xv(b: f64, a: f64) -> f64 {
        -b / (2.0 * a)
    }

    fn calculate_yv(delta: f64, a: f64) -> f64 {
        -delta / (4.0 * a)
    }

    fn create_valid_results(a: f64, b: f64, c: f64, delta: f64) -> Self {
        let delta_sqrt = delta.sqrt();
        Self {
            a,
            b,
            c,
            x1: Self::calculate_x1(b, delta_sqrt, a),
            x2: Self::calculate_x2(b, delta_sqrt, a),
            xv: Self::calculate_xv(b, a),
            yv: Self::calculate_yv(delta, a),
        }
    }

    fn create_empty_results() -> Self {
        Self {
            a: 0.0,
            b: 0.0,
            c: 0.0,
            x1: 0.0,
            x2: 0.0,
            xv: 0.0,
            yv: 0.0,
        }
    }
}

impl fmt::Display for Squared {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sb = if self.b >= 0.0 { "+" } else { "" };
        let sc = if self.c >= 0.0 { "+" } else { "" };

        write!(f, "{}x² {}{}x {}{}= 0", self.a, sb, self.b, sc, self.c)
    }
}
