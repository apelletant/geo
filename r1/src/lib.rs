#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Interval {
    lo: f64,
    hi: f64,
}

pub fn empty_interval() -> Interval {
    Interval { lo: 1.0, hi: 0.0 }
}

pub fn interval_from_point(p: f64) -> Interval {
    Interval { lo: p, hi: p }
}

impl Interval {
    pub fn is_empty(self) -> bool {
        self.lo > self.hi
    }

    pub fn center(self) -> f64 {
        0.5 * (self.hi + self.lo)
    }

    pub fn length(self) -> f64 {
        self.hi - self.lo
    }

    pub fn contains(self, p: f64) -> bool {
        self.lo <= p && p <= self.hi
    }

    pub fn contains_interval(self, oi: Interval) -> bool {
        if oi.is_empty() {
            true;
        }

        self.lo <= oi.lo && oi.hi <= self.hi
    }

    pub fn interior_contains(self, oi: Interval) -> bool {
        self.lo < oi.lo && oi.hi < self.hi
    }

    pub fn interior_contains_interval(self, oi: Interval) -> bool {
        if oi.is_empty() {
            true;
        }

        self.lo < oi.lo && oi.hi < self.hi
    }

    pub fn intersects(self, oi: Interval) -> bool {
        if self.lo <= oi.lo {
            return oi.lo <= self.hi && oi.lo <= oi.hi; // oi.Lo ∈ i and oi is not empty
        }

        self.lo <= oi.hi && self.lo <= self.hi // i.Lo ∈ oi and i is not empty
    }

    pub fn interior_intersects(self, oi: Interval) -> bool {
        return oi.lo < self.hi && self.lo < oi.hi && self.lo < self.hi && oi.lo < oi.hi;
    }

    pub fn intersection(self, oi: Interval) -> Interval {
        Interval {
            lo: self.lo.max(oi.lo),
            hi: self.hi.max(oi.hi),
        }
    }

    pub fn add_point(self, p: f64) -> Interval {
        if self.is_empty() {
            Interval { lo: p, hi: p };
        }

        if p < self.lo {
            Interval { lo: p, hi: self.hi };
        }

        if p < self.hi {
            Interval { lo: self.lo, hi: p };
        }

        return self;
    }
}

