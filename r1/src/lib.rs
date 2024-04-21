use std::f64::INFINITY;

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

// epsilon is a small number that represents a reasonable level of noise between two
// values that can be considered to be equal.
const EPSILON: f64 = 1e-15;
// dblEpsilon is a smaller number for values that require more precision.
// This is the C++ DBL_EPSILON equivalent.
const DBL_EPSILON: f64 = 2.220446049250313e-16;

impl Interval {
    pub fn is_empty(self) -> bool {
        self.lo > self.hi
    }

    pub fn equal(self, oi: Interval) -> bool {
        return self == oi || self.is_empty() && oi.is_empty();
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
            return true;
        }

        self.lo <= oi.lo && oi.hi <= self.hi
    }

    pub fn interior_contains(self, p: f64) -> bool {
        self.lo < p && p < self.hi
    }

    pub fn interior_contains_interval(self, oi: Interval) -> bool {
        println!("oi.is_empty() = {}", oi.is_empty());

        if oi.is_empty() {
            return true;
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
        return oi.lo < self.hi && self.lo < oi.hi && self.lo < self.hi && oi.lo <= oi.hi;
    }

    pub fn intersection(self, oi: Interval) -> Interval {
        Interval {
            lo: self.lo.max(oi.lo),
            hi: self.hi.min(oi.hi),
        }
    }

    pub fn add_point(self, p: f64) -> Interval {
        if self.is_empty() {
            Interval { lo: p, hi: p };
        }

        if p < self.lo {
            Interval { lo: p, hi: self.hi };
        }

        if p > self.hi {
            Interval { lo: self.lo, hi: p };
        }

        return self;
    }

    pub fn clamp_point(self, p: f64) -> f64 {
        return self.lo.max(self.hi.min(p));
    }

    pub fn expanded(self, margin: f64) -> Interval {
        if self.is_empty() {
            return self;
        }

        Interval {
            lo: self.lo - margin,
            hi: self.hi + margin,
        }
    }

    pub fn union(self, other: Interval) -> Interval {
        if self.is_empty() {
            return other;
        }

        if other.is_empty() {
            return self;
        }

        return Interval {
            lo: self.lo.min(other.lo),
            hi: self.hi.max(other.hi),
        };
    }

    pub fn string(self) -> String {
        format!("[{:.7}, {:.7}]", self.lo, self.hi)
    }

    pub fn approx_equal(self, other: Interval) -> bool {
        if self.is_empty() {
            return other.length() <= 2.0 * EPSILON;
        }

        if other.is_empty() {
            return self.length() <= 2.0 * EPSILON;
        }

        return (other.lo - self.lo).abs() <= EPSILON && (other.hi - self.hi).abs() <= EPSILON;
    }

    pub fn directed_hausdorff_distance(self, other: Interval) -> f64 {
        if self.is_empty() {
            return 0.0;
        }

        if other.is_empty() {
            return INFINITY;
        }

        let zero: f64 = 0.0;
        return zero.max((self.hi - other.hi).max(other.lo - self.lo));
    }
}

#[cfg(test)]
mod interval {
    use super::*;

    fn setup_interval() -> (Interval, Interval, Interval, Interval) {
        let empty: Interval = empty_interval();
        let unit: Interval = Interval { lo: 0.0, hi: 1.0 };
        let negunit: Interval = Interval { lo: -1.0, hi: 0.0 };
        let half: Interval = Interval { lo: 0.5, hi: 0.5 };

        (empty, unit, negunit, half)
    }

    #[test]
    fn empty() {
        let (empty, unit, negunit, half) = setup_interval();

        let mut res = empty.is_empty();
        assert_eq!(true, res);

        res = unit.is_empty();
        assert_eq!(false, res);

        res = negunit.is_empty();
        assert_eq!(false, res);

        res = half.is_empty();
        assert_eq!(false, res);
    }

    #[test]
    fn center() {
        let (_, unit, negunit, half) = setup_interval();

        let mut expected = 0.5;
        let mut res = unit.center();
        assert_eq!(expected, res);

        expected = -0.5;
        res = negunit.center();
        assert_eq!(expected, res);

        expected = 0.5;
        res = half.center();
        assert_eq!(expected, res);
    }

    #[test]
    fn length() {
        let (_, unit, negunit, half) = setup_interval();

        let mut expected = 1.0;
        let mut res = unit.length();
        assert_eq!(expected, res);

        expected = 1.0;
        res = negunit.length();
        assert_eq!(expected, res);

        expected = 0.0;
        res = half.length();
        assert_eq!(expected, res);
    }

    #[test]
    fn interval_contains() {
        let (_, unit, _, _) = setup_interval();

        let mut p: f64 = 0.5;
        let mut expected_contains: bool = true;
        let mut expected_interior_contains: bool = true;
        let mut res: bool = unit.contains(p);
        let mut res_interior: bool = unit.interior_contains(p);
        assert_eq!(expected_contains, res);
        assert_eq!(expected_interior_contains, res_interior);

        p = 0.0;
        expected_contains = true;
        expected_interior_contains = false;
        res = unit.contains(p);
        res_interior = unit.interior_contains(p);
        assert_eq!(expected_contains, res);
        assert_eq!(expected_interior_contains, res_interior);

        p = 1.0;
        expected_contains = true;
        expected_interior_contains = false;
        res = unit.contains(p);
        res_interior = unit.interior_contains(p);
        assert_eq!(expected_contains, res);
        assert_eq!(expected_interior_contains, res_interior);
    }

    struct OperationTest {
        label: String,
        have: Interval,
        other: Interval,
        contains: bool,
        interior_contains: bool,
        intersects: bool,
        interior_intersects: bool,
    }

    #[test]
    fn interval_operations() {
        let (empty, unit, negunit, half) = setup_interval();

        let tests_array: [OperationTest; 8] = [
            OperationTest {
                label: String::from("testing empty empty"),
                have: empty,
                other: empty,
                contains: true,
                interior_contains: true,
                intersects: false,
                interior_intersects: false,
            },
            OperationTest {
                label: String::from("testing empty unit"),
                have: empty,
                other: unit,
                contains: false,
                interior_contains: false,
                intersects: false,
                interior_intersects: false,
            },
            OperationTest {
                label: String::from("testing unit half"),
                have: unit,
                other: half,
                contains: true,
                interior_contains: true,
                intersects: true,
                interior_intersects: true,
            },
            OperationTest {
                label: String::from("testing unit unit"),
                have: unit,
                other: unit,
                contains: true,
                interior_contains: false,
                intersects: true,
                interior_intersects: true,
            },
            OperationTest {
                label: String::from("testing unit empty"),
                have: unit,
                other: empty,
                contains: true,
                interior_contains: true,
                intersects: false,
                interior_intersects: false,
            },
            OperationTest {
                label: String::from("testing unit negunit"),
                have: unit,
                other: negunit,
                contains: false,
                interior_contains: false,
                intersects: true,
                interior_intersects: false,
            },
            OperationTest {
                label: String::from("testing unit Interval { lo: 0.0, hi: 0.5 }"),
                have: unit,
                other: Interval { lo: 0.0, hi: 0.5 },
                contains: true,
                interior_contains: false,
                intersects: true,
                interior_intersects: true,
            },
            OperationTest {
                label: String::from("testing half Interval { lo: 0.0, hi: 0.5 }"),
                have: half,
                other: Interval { lo: 0.0, hi: 0.5 },
                contains: false,
                interior_contains: false,
                intersects: true,
                interior_intersects: false,
            },
        ];

        for test in tests_array {
            println!("\n{}", test.label);
            let mut res: bool;
            res = test.have.contains_interval(test.other);
            assert_eq!(res, test.contains);

            res = test.have.interior_contains_interval(test.other);
            assert_eq!(res, test.interior_contains);

            res = test.have.intersects(test.other);
            assert_eq!(res, test.intersects);

            res = test.have.interior_intersects(test.other);
            assert_eq!(res, test.interior_intersects)
        }
    }

    struct TestCase {
        x: Interval,
        y: Interval,
        want: Interval,
    }

    #[test]
    fn intersections() {
        let (empty, unit, negunit, half) = setup_interval();

        let tests_array: [TestCase; 5] = [
            TestCase {
                x: unit,
                y: half,
                want: half,
            },
            TestCase {
                x: unit,
                y: negunit,
                want: Interval { lo: 0.0, hi: 0.0 },
            },
            TestCase {
                x: negunit,
                y: half,
                want: empty,
            },
            TestCase {
                x: unit,
                y: empty,
                want: empty,
            },
            TestCase {
                x: empty,
                y: unit,
                want: empty,
            },
        ];

        for test in tests_array {
            let res: Interval;
            res = test.x.intersection(test.y);
            assert_eq!(true, test.want.equal(res))
        }
    }

    #[test]
    fn union() {
        let (empty, unit, negunit, half) = setup_interval();

        let tests_array: [TestCase; 8] = [
            TestCase {
                x: Interval {
                    lo: 99.0,
                    hi: 100.0,
                },
                y: empty,
                want: Interval {
                    lo: 99.0,
                    hi: 100.0,
                },
            },
            TestCase {
                x: empty,
                y: Interval {
                    lo: 99.0,
                    hi: 100.0,
                },
                want: Interval {
                    lo: 99.0,
                    hi: 100.0,
                },
            },
            TestCase {
                x: Interval { lo: 5.0, hi: 3.0 },
                y: Interval { lo: 0.0, hi: -2.0 },
                want: empty,
            },
            TestCase {
                x: Interval { lo: 0.0, hi: -2.0 },
                y: Interval { lo: 5.0, hi: 3.0 },
                want: empty,
            },
            TestCase {
                x: unit,
                y: unit,
                want: unit,
            },
            TestCase {
                x: unit,
                y: negunit,
                want: Interval { lo: -1.0, hi: 1.0 },
            },
            TestCase {
                x: negunit,
                y: unit,
                want: Interval { lo: -1.0, hi: 1.0 },
            },
            TestCase {
                x: half,
                y: unit,
                want: unit,
            },
        ];

        for test in tests_array {
            let res: Interval;
            res = test.x.union(test.y);
            assert_eq!(true, test.want.equal(res))
        }
    }

    struct PointTest {
        interval: Interval,
        point: f64,
        want: Interval,
    }

    #[test]
    fn add_point() {
        let (empty, _, _, _) = setup_interval();

        let tests_array: [PointTest; 4] = [
            PointTest {
                interval: empty,
                point: 5.0,
                want: Interval { lo: 5.0, hi: 5.0 },
            },
            PointTest {
                interval: Interval { lo: 5.0, hi: 5.0 },
                point: -1.0,
                want: Interval { lo: -1.0, hi: 5.0 },
            },
            PointTest {
                interval: Interval { lo: -1.0, hi: 5.0 },
                point: 0.0,
                want: Interval { lo: -1.0, hi: 5.0 },
            },
            PointTest {
                interval: Interval { lo: -1.0, hi: 5.0 },
                point: 6.0,
                want: Interval { lo: -1.0, hi: 6.0 },
            },
        ];

        for test in tests_array {
            let res: Interval;

            res = test.interval.add_point(test.point);
            assert_eq!(true, test.want.equal(res))
        }
    }

    struct ClampPointTest {
        interval: Interval,
        clamp: f64,
        want: f64,
    }

    #[test]
    fn clamp_point() {
        let tests_array: [ClampPointTest; 3] = [
            ClampPointTest {
                interval: Interval { lo: 0.1, hi: 0.4 },
                clamp: 0.3,
                want: 0.3,
            },
            ClampPointTest {
                interval: Interval { lo: 0.1, hi: 0.4 },
                clamp: -7.0,
                want: 0.1,
            },
            ClampPointTest {
                interval: Interval { lo: 0.1, hi: 0.4 },
                clamp: 0.6,
                want: 0.4,
            },
        ];

        for test in tests_array {
            let res: f64;

            res = test.interval.clamp_point(test.clamp);
            assert_eq!(test.want, res);
        }
    }

    struct ExpendedTest {
        interval: Interval,
        margin: f64,
        want: Interval,
    }

    #[test]
    fn expended() {
        let (empty, unit, _, _) = setup_interval();

        let tests_array: [ExpendedTest; 4] = [
            ExpendedTest {
                interval: empty,
                margin: 0.45,
                want: empty,
            },
            ExpendedTest {
                interval: unit,
                margin: 0.5,
                want: Interval { lo: -0.5, hi: 1.5 },
            },
            ExpendedTest {
                interval: unit,
                margin: -0.5,
                want: Interval { lo: 0.5, hi: 0.5 },
            },
            ExpendedTest {
                interval: unit,
                margin: 0.51,
                want: empty,
            },
        ];

        for test in tests_array {
            let res: Interval;

            res = test.interval.expanded(test.margin);
            assert_eq!(true, test.want.equal(res));
        }
    }

    #[test]
    fn interval_string() {
        let i: Interval = Interval { lo: 2.0, hi: 4.5 };
        let res: String;

        res = i.string();

        assert_eq!("[2.0000000, 4.5000000]", res);
    }

    struct ApproxEqualTest {
        interval: Interval,
        other: Interval,
        want: bool,
    }
    #[test]
    fn approx_equal() {
        let (empty, _, _, _) = setup_interval();

        let lo = 4.0 * DBL_EPSILON; // < max_error default
        let hi = 6.0 * DBL_EPSILON; // > max_error default

        let tests_array: [ApproxEqualTest; 21] = [
            ApproxEqualTest {
                interval: empty,
                other: empty,
                want: true,
            },
            ApproxEqualTest {
                interval: Interval { lo: 0.0, hi: 0.0 },
                other: empty,
                want: true,
            },
            ApproxEqualTest {
                interval: empty,
                other: Interval { lo: 0.0, hi: 0.0 },
                want: true,
            },
            ApproxEqualTest {
                interval: Interval { lo: 1.0, hi: 1.0 },
                other: empty,
                want: true,
            },
            ApproxEqualTest {
                interval: empty,
                other: Interval { lo: 1.0, hi: 1.0 },
                want: true,
            },
            ApproxEqualTest {
                interval: empty,
                other: Interval { lo: 0.0, hi: 1.0 },
                want: false,
            },
            ApproxEqualTest {
                interval: empty,
                other: Interval {
                    lo: 1.0,
                    hi: 1.0 + 2.0 * lo,
                },
                want: true,
            },
            ApproxEqualTest {
                interval: empty,
                other: Interval {
                    lo: 1.0,
                    hi: 1.0 + 2.0 * hi,
                },
                want: false,
            },
            ApproxEqualTest {
                interval: Interval { lo: 1.0, hi: 1.0 },
                other: Interval { lo: 1.0, hi: 1.0 },
                want: true,
            },
            ApproxEqualTest {
                interval: Interval { lo: 1.0, hi: 1.0 },
                other: Interval {
                    lo: 1.0 - lo,
                    hi: 1.0 - lo,
                },
                want: true,
            },
            ApproxEqualTest {
                interval: Interval { lo: 1.0, hi: 1.0 },
                other: Interval {
                    lo: 1.0 + lo,
                    hi: 1.0 + lo,
                },
                want: true,
            },
            ApproxEqualTest {
                interval: Interval { lo: 1.0, hi: 1.0 },
                other: Interval {
                    lo: 1.0 - hi,
                    hi: 1.0,
                },
                want: false,
            },
            ApproxEqualTest {
                interval: Interval { lo: 1.0, hi: 1.0 },
                other: Interval {
                    lo: 1.0,
                    hi: 1.0 + hi,
                },
                want: false,
            },
            ApproxEqualTest {
                interval: Interval { lo: 1.0, hi: 1.0 },
                other: Interval {
                    lo: 1.0 - lo,
                    hi: 1.0 + lo,
                },
                want: true,
            },
            ApproxEqualTest {
                interval: Interval { lo: 0.0, hi: 0.0 },
                other: Interval { lo: 1.0, hi: 1.0 },
                want: false,
            },
            ApproxEqualTest {
                interval: Interval {
                    lo: 1.0 - lo,
                    hi: 2.0 + lo,
                },
                other: Interval { lo: 1.0, hi: 2.0 },
                want: true,
            },
            ApproxEqualTest {
                interval: Interval {
                    lo: 1.0 + lo,
                    hi: 2.0 - lo,
                },
                other: Interval { lo: 1.0, hi: 2.0 },
                want: true,
            },
            ApproxEqualTest {
                interval: Interval {
                    lo: 1.0 - hi,
                    hi: 2.0 + lo,
                },
                other: Interval { lo: 1.0, hi: 2.0 },
                want: false,
            },
            ApproxEqualTest {
                interval: Interval {
                    lo: 1.0 + hi,
                    hi: 2.0 - lo,
                },
                other: Interval { lo: 1.0, hi: 2.0 },
                want: false,
            },
            ApproxEqualTest {
                interval: Interval {
                    lo: 1.0 - lo,
                    hi: 2.0 + hi,
                },
                other: Interval { lo: 1.0, hi: 2.0 },
                want: false,
            },
            ApproxEqualTest {
                interval: Interval {
                    lo: 1.0 + lo,
                    hi: 2.0 - hi,
                },
                other: Interval { lo: 1.0, hi: 2.0 },
                want: false,
            },
        ];

        for test in tests_array {
            let res: bool;

            res = test.interval.approx_equal(test.other);
            assert_eq!(test.want, res)
        }
    }
}
