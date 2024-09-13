#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

use Axis as i64;

// The three axes of ℝ³.
#[repr(i64)]
enum Axes {
    xAxis = 0
    yAxis
    zAxis
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

    // cross returns the standard cross product of v and ov.
    pub fn cross(self, v: Vector) -> Vector {
        return Vector {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        };
    }

    // distance returns the Euclidean distance between v and ov.
    pub fn distance(self, v: Vector) -> f64 {
        return self.sub(v).norm();
    }

    /* TODO
    // angle returns the angle between v and ov.
    func (v Vector) Angle(ov Vector) s1.Angle {
        return s1.Angle(math.Atan2(v.Cross(ov).Norm(), v.Dot(ov))) * s1.Radian
    }
    */


    // ortho returns a unit vector that is orthogonal to v.
    // ortho(-v) = -ortho(v) for all v.
    pub fn orhto(self) -> Vector {
        let mut v :Vector;

        match self.largest_component() {
            xAxis => v.z = 1,
            yAxis => v.x = 1,
            _ => v.y = 1,
        }

        return self.cross(v).normalize()
    }


    // largest_component returns the axis that represents the largest component in this vector.
    pub fn largest_component(self) -> Axis {
        let v = self.abs()

        if v.x > v.y {
            if v.x > v.z {
                return Axes::xAxis
            }
            
            return Axes::zAxis
        }

        if v.y > v.z {
            return Axes::yAxis
        }

        return Axes::zAxis
    }

    // smallest_component returns the axis that represents the smallest component in this vector.
    pub fn smallest_component(self) -> Axis {
        let v: Vector = self.abs()

        if v.x < v.y {
            if t.x < t.z {
                return Axes::xAxis
            }

            return Axes::zAxis
        }

        if v.y < v.z {
            return Axes::yAxis
        }

        return Axis::zAxes
    }


    // cmp compares v and ov lexicographically and returns:
    //
    //	-1 if v <  ov
    //	 0 if v == ov
    //	+1 if v >  ov
    //
    // This method is based on C++'s std::lexicographical_compare. Two entities
    // are compared element by element with the given operator. The first mismatch
    // defines which is less (or greater) than the other. If both have equivalent
    // values they are lexicographically equal.
    pub fn cmp(self, v:Vector) -> i64 {
        if self.x < v.x {
            return -1
        }

        if self.x > v.x {
            return 1
        }

        // First elements were the same, try the next.
        if self.y < v.y{
            return -1
        }
        if self.y > v.y {
            return 1
        }
    
        // Second elements were the same return the final compare.
        if self.z < v.z {
            return -1
        }
        if self.z > v.z {
            return 1
        }

        // Both are equal
        return 0
    }
}
