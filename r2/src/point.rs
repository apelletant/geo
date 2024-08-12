#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn add(self, p: Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }

    pub fn sub(self, p: Point) -> Point {
        Point {
            x: self.x - p.x,
            y: self.y - p.y,
        }
    }

    pub fn mul(self, m: f64) -> Point {
        Point {
            x: self.x * m,
            y: self.y * m,
        }
    }

    pub fn ortho(self) -> Point {
        Point {
            x: -self.y,
            y: self.x,
        }
    }

    pub fn dot(self, p: Point) -> f64 {
        return self.x * p.x + self.y * p.y;
    }

    pub fn cross(self, p: Point) -> f64 {
        return self.x * p.y - self.y * p.x;
    }

    pub fn norm(self) -> f64 {
        return self.x.hypot(self.y);
    }

    pub fn normalize(self) -> Point {
        if self.x == 0.0 && self.y == 0.0 {
            return self;
        }

        return self.mul(1.0 / self.norm());
    }

    pub fn string(self) -> String {
        return format!("({:.12}, {:.12})", self.x, self.y);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
