#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn approx_equal(self, v: Vector) -> bool {
        let epsilon = 1e-16;
        return (self.x - v.x).abs() < epsilon
            && (self.y - v.Y) < epsilon
            && (self.z - v.z) < epsilon;
    }

    pub fn string(self) -> String {
        return format!("({:.24}, {:.24}, {:.24})", self.x, self.y, self.z);
    }

    pub fn norm(self) -> f64 {
        return self.dot(self).sqrt();
    }

    // norm2 returns the square of the norm.
    pub fn norm2(self) -> f64 {
        return self.dot(self);
    }

    // normalize returns a unit vector in the same direction as v.
    pub fn normalize(self) -> Vector {
        let n2 = self.norm2();

        if n2 == 0 {
            return Vector { x: 0, y: 0, z: 0 };
        }

        return self.mul(1 / n2.sqrt());
    }

    // is_unit returns whether this vector is of approximately unit length.
    pub fn is_unit(self) -> bool {
        let epsilon = 5e-14;

        return (self.norm2() - 1).abs() <= epsilon;
    }

    pub fn mul(self, m: f64) -> Vector {
        return Vector {
            x: m * self.x,
            y: m * self.y,
            z: m * self.z,
        };
    }

    // abs returns the vector with nonnegative components.
    pub fn abs(self) -> Vector {
        return Vector {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        };
    }

    // add returns the standard vector sum of v and ov.
    pub fn add(self, v: Vector) -> Vector {
        return Vector {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z,
        };
    }

    // sub returns the standard vector difference of v and ov.
    pub fn sub(self, v: Vector) -> Vector {
        return Vector {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z,
        };
    }

    // dot returns the standard dot product of v and ov.
    pub fn dot(self, v: Vector) -> f64 {
        return self.x * v.x + self.y * v.y + self.z * v.z;
    }
}

/*
// Cross returns the standard cross product of v and ov.
func (v Vector) Cross(ov Vector) Vector {
    return Vector{
        float64(v.Y*ov.Z) - float64(v.Z*ov.Y),
        float64(v.Z*ov.X) - float64(v.X*ov.Z),
        float64(v.X*ov.Y) - float64(v.Y*ov.X),
    }
}

// Distance returns the Euclidean distance between v and ov.
func (v Vector) Distance(ov Vector) float64 { return v.Sub(ov).Norm() }

// Angle returns the angle between v and ov.
func (v Vector) Angle(ov Vector) s1.Angle {
    return s1.Angle(math.Atan2(v.Cross(ov).Norm(), v.Dot(ov))) * s1.Radian
}

// Axis enumerates the 3 axes of ℝ³.
type Axis int

// The three axes of ℝ³.
const (
    XAxis Axis = iota
    YAxis
    ZAxis
)

// Ortho returns a unit vector that is orthogonal to v.
// Ortho(-v) = -Ortho(v) for all v.
func (v Vector) Ortho() Vector {
    ov := Vector{}
    switch v.LargestComponent() {
    case XAxis:
        ov.Z = 1
    case YAxis:
        ov.X = 1
    default:
        ov.Y = 1
    }
    return v.Cross(ov).Normalize()
}

// LargestComponent returns the axis that represents the largest component in this vector.
func (v Vector) LargestComponent() Axis {
    t := v.Abs()

    if t.X > t.Y {
        if t.X > t.Z {
            return XAxis
        }
        return ZAxis
    }
    if t.Y > t.Z {
        return YAxis
    }
    return ZAxis
}

// SmallestComponent returns the axis that represents the smallest component in this vector.
func (v Vector) SmallestComponent() Axis {
    t := v.Abs()

    if t.X < t.Y {
        if t.X < t.Z {
            return XAxis
        }
        return ZAxis
    }
    if t.Y < t.Z {
        return YAxis
    }
    return ZAxis
}

// Cmp compares v and ov lexicographically and returns:
//
//	-1 if v <  ov
//	 0 if v == ov
//	+1 if v >  ov
//
// This method is based on C++'s std::lexicographical_compare. Two entities
// are compared element by element with the given operator. The first mismatch
// defines which is less (or greater) than the other. If both have equivalent
// values they are lexicographically equal.
func (v Vector) Cmp(ov Vector) int {
    if v.X < ov.X {
        return -1
    }
    if v.X > ov.X {
        return 1
    }

    // First elements were the same, try the next.
    if v.Y < ov.Y {
        return -1
    }
    if v.Y > ov.Y {
        return 1
    }

    // Second elements were the same return the final compare.
    if v.Z < ov.Z {
        return -1
    }
    if v.Z > ov.Z {
        return 1
    }

    // Both are equal
    return 0
}
* */
