use crate::point::Point;
use r1::*;

extern crate r1;

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: r1::Interval,
    pub y: r1::Interval,
}

pub fn rect_from_points(points: &[Point]) -> Rect {
    if points.len() == 0 {
        return Rect {
            x: Interval { lo: 0.0, hi: 0.0 },
            y: Interval { lo: 0.0, hi: 0.0 },
        };
    }

    let mut r = Rect {
        x: Interval {
            lo: points[0].x,
            hi: points[0].x,
        },
        y: Interval {
            lo: points[0].y,
            hi: points[0].y,
        },
    };

    for point in points {
        r = r.add_point(*point)
    }

    return r;
}

// rect_from_center_size(constructs a rectangle with the given center and size.
// Both dimensions of size must be non-negative.
pub fn rect_from_center_size(center: Point, size: Point) -> Rect {
    let ix = Interval {
        lo: center.x - size.x / 2.0,
        hi: center.x + size.x / 2.0,
    };
    let iy = Interval {
        lo: center.y - size.y / 2.0,
        hi: center.y + size.y / 2.0,
    };
    return Rect { x: ix, y: iy };
}

// empty_rect constructs the canonical empty rectangle. Use IsEmpty() to test
// for empty rectangles, since they have more than one representation. A Rect{}
// is not the same as the EmptyRect.
pub fn empty_rect() -> Rect {
    return Rect {
        x: empty_interval(),
        y: empty_interval(),
    };
}

impl Rect {
    pub fn is_valid(self) -> bool {
        return self.x.is_empty() == self.y.is_empty();
    }

    // vertices returns all four vertices of the rectangle. Vertices are returned in
    // CCW direction starting with the lower left corner.
    pub fn vertices(self) -> [Point; 4] {
        [
            Point {
                x: self.x.lo,
                y: self.y.lo,
            },
            Point {
                x: self.x.hi,
                y: self.y.lo,
            },
            Point {
                x: self.x.hi,
                y: self.y.hi,
            },
            Point {
                x: self.x.lo,
                y: self.y.hi,
            },
        ]
    }

    pub fn vertex_i_j(self, i: i64, j: i64) -> Point {
        let mut x = self.x.lo;
        if i == 1 {
            x = self.x.hi
        }

        let mut y = self.y.lo;
        if j == 1 {
            y = self.y.hi
        }

        return Point { x, y };
    }

    pub fn lo(self) -> Point {
        Point {
            x: self.x.lo,
            y: self.y.lo,
        }
    }

    pub fn hi(self) -> Point {
        Point {
            x: self.x.hi,
            y: self.y.hi,
        }
    }

    pub fn center(self) -> Point {
        Point {
            x: self.x.center(),
            y: self.y.center(),
        }
    }

    pub fn size(self) -> Point {
        Point {
            x: self.x.length(),
            y: self.y.length(),
        }
    }

    // contains_point reports whether the rectangle contains the given point.
    // Rectangles are closed regions, i.e. they contain their boundary.
    pub fn contains_point(self, p: Point) -> bool {
        return self.x.contains(p.x) && self.y.contains(p.y);
    }

    // interior_contains_point returns true iff the given point is contained in the interior
    // of the region (i.e. the region excluding its boundary).
    pub fn interior_contains_point(self, p: Point) -> bool {
        return self.x.interior_contains(p.x) && self.y.interior_contains(p.y);
    }

    pub fn contains(self, r: Rect) -> bool {
        return self.x.contains_interval(r.x) && self.y.contains_interval(r.y);
    }

    pub fn interior_contains(self, r: Rect) -> bool {
        return self.x.interior_contains_interval(r.x) && self.y.interior_contains_interval(r.y);
    }

    pub fn intersects(self, r: Rect) -> bool {
        return self.x.intersects(r.x) && self.y.intersects(r.y);
    }

    pub fn interior_intersects(self, r: Rect) -> bool {
        return self.x.interior_intersects(r.x) && self.y.interior_intersects(r.y);
    }

    pub fn add_point(self, p: Point) -> Rect {
        return Rect {
            x: self.x.add_point(p.x),
            y: self.y.add_point(p.y),
        };
    }

    pub fn add_rect(self, r: Rect) -> Rect {
        return Rect {
            x: self.x.union(r.x),
            y: self.y.union(r.y),
        };
    }

    // clamp_point returns the closest point in the rectangle to the given point.
    // The rectangle must be non-empty.
    pub fn clamp_point(self, p: Point) -> Point {
        return Point {
            x: self.x.clamp_point(p.x),
            y: self.y.clamp_point(p.y),
        };
    }

    // expanded returns a rectangle that has been expanded in the x-direction
    // by margin.X, and in y-direction by margin.Y. If either margin is empty,
    // then shrink the interval on the corresponding sides instead. The resulting
    // rectangle may be empty. Any expansion of an empty rectangle remains empty.
    pub fn expanded(self, margin: Point) -> Rect {
        let xx: Interval = self.x.expanded(margin.x);
        let yy: Interval = self.y.expanded(margin.y);

        if xx.is_empty() || yy.is_empty() {
            return empty_rect();
        }

        return Rect { x: xx, y: yy };
    }

    // expanded_by_margin returns a Rect that has been expanded by the amount on all sides.
    pub fn expanded_by_margin(self, margin: f64) -> Rect {
        return self.expanded(Point {
            x: margin,
            y: margin,
        });
    }

    // union returns the smallest rectangle containing the union of this rectangle and
    // the given rectangle.
    pub fn union(self, r: Rect) -> Rect {
        return Rect {
            x: self.x.union(r.x),
            y: self.y.union(r.y),
        };
    }

    // Intersection returns the smallest rectangle containing the intersection of this
    // rectangle and the given rectangle.
    pub fn intersection(self, r: Rect) -> Rect {
        let xx = self.x.intersection(r.x);
        let yy = self.y.intersection(r.y);

        if xx.is_empty() || yy.is_empty() {
            return empty_rect();
        }

        return Rect { x: xx, y: yy };
    }

    // approx_equal returns true if the x- and y-intervals of the two rectangles are
    // the same up to the given tolerance.
    pub fn approx_equal(self, r: Rect) -> bool {
        return self.x.approx_equal(r.x) && self.y.approx_equal(r.y);
    }

    pub fn string(self) -> String {
        return format!("[lo{:?}, hi{:?}]", self.lo(), self.hi());
    }
}
