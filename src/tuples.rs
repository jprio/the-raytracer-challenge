extern crate alloc;
use alloc::slice::Join;
use core::borrow::Borrow;
use float_eq::float_eq;
//use std::slice::Join;
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TupleKind {
    Vector = 0,
    Point = 1,
    Color = 2,
}

impl std::ops::Add for TupleKind {
    type Output = TupleKind;
    fn add(self, other: TupleKind) -> TupleKind {
        match self {
            TupleKind::Point => TupleKind::Point,
            TupleKind::Vector => match other {
                TupleKind::Vector => TupleKind::Vector,
                TupleKind::Point => TupleKind::Point,
                TupleKind::Color => panic!("cannot add"),
            },
            TupleKind::Color => match other {
                TupleKind::Color => TupleKind::Color,
                _ => panic!("cannot add"),
            },
        }
    }
}
impl std::ops::Sub for TupleKind {
    type Output = TupleKind;
    fn sub(self, other: TupleKind) -> TupleKind {
        match self {
            TupleKind::Point => TupleKind::Point,
            TupleKind::Vector => match other {
                TupleKind::Vector => TupleKind::Vector,
                TupleKind::Point => TupleKind::Point,
                TupleKind::Color => panic!("cannot add"),
            },
            TupleKind::Color => match other {
                TupleKind::Color => TupleKind::Color,
                _ => panic!("cannot add"),
            },
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Projectile {
    pub position: Tuple,
    pub velocity: Tuple,
}

#[derive(Debug, Copy, Clone)]
pub struct Environment {
    pub gravity: Tuple,
    pub wind: Tuple,
}
pub fn tick(env: Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;
    //let position = proj.position;
    //let velocity = proj.velocity;
    return Projectile {
        position: position,
        velocity: velocity,
    };
}

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: TupleKind,
}
// TODO : pk ça marche pas !?!?
//impl<S: Borrow<Tuple>> alloc::slice::Join<&Tuple> for [S] {
impl Join<&str> for Tuple {
    type Output = String;
    fn join(slice: &Self, sep: &str) -> Self::Output {
        let mut joined = "".to_string();
        joined.push_str(&slice.x.to_string());
        joined.push_str(&sep);
        joined.push_str(&slice.y.to_string());
        joined.push_str(&sep);
        joined.push_str(&slice.z.to_string());
        joined.push_str(&sep);
        joined
    }
}
impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        //*self == other.format
        float_eq!(self.x, other.x, abs <= 0.1)
            && float_eq!(self.y, other.y, abs <= 0.1)
            && float_eq!(self.z, other.z, abs <= 0.1)
            && self.w == other.w
    }
}
impl std::ops::Add for Tuple {
    type Output = Tuple;
    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}
impl std::ops::Sub for Tuple {
    type Output = Tuple;
    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}
impl std::ops::Mul<f64> for Tuple {
    type Output = Tuple;
    fn mul(self, other: f64) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w,
        }
    }
}
impl std::ops::Div<f64> for Tuple {
    type Output = Tuple;
    fn div(self, other: f64) -> Tuple {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w,
        }
    }
}

impl Tuple {
    pub fn magnitude(&self) -> f64 {
        if self.w == TupleKind::Point {
            panic!("Cannot get the magnitude of a point :(")
        }
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    pub fn normalize(&self) -> Self {
        if self.w == TupleKind::Point {
            panic!("Cannot normalize a point :(")
        }
        *self / self.magnitude()
    }
    pub fn dot(&self, other: Tuple) -> f64 {
        if self.w == TupleKind::Point || other.w == TupleKind::Point {
            panic!("Cannot dot with a point :(")
        }

        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: Tuple) -> Tuple {
        if self.w == TupleKind::Point || other.w == TupleKind::Point {
            panic!("Cannot cross with a point :(")
        }
        Tuple {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: TupleKind::Vector,
        }
    }
    pub fn clamp(&self, max: f64, min: f64, scale: f64) -> Tuple {
        Tuple {
            x: (self.x * scale).clamp(min, max).round(),
            y: (self.y * scale).clamp(min, max).round(),
            z: (self.z * scale).clamp(min, max).round(),
            w: self.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuples::*;
    #[test]
    fn can_instantiate_point() {
        let p = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: TupleKind::Point,
        };
        assert_eq!(p.x, 4.3);
        assert_eq!(p.y, -4.2);
        assert_eq!(p.z, 3.1);
    }
    #[test]
    fn can_compare() {
        let p1 = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: TupleKind::Point,
        };
        let p2 = Tuple {
            x: 4.35,
            y: -4.2,
            z: 3.1,
            w: TupleKind::Point,
        };
        let p3 = Tuple {
            x: 4.45,
            y: -4.2,
            z: 3.1,
            w: TupleKind::Point,
        };
        assert_eq!(p1, p2);
        assert_ne!(p1, p3);
    }
    #[test]
    fn can_add_sub() {
        let p1 = Tuple {
            x: 4.3,
            y: -4.2,
            z: 3.1,
            w: TupleKind::Point,
        };
        let p2 = Tuple {
            x: 4.35,
            y: -4.2,
            z: 3.1,
            w: TupleKind::Point,
        };
        let p3 = p1 + p2;
        assert_eq!(
            p3,
            Tuple {
                x: 8.65,
                y: -8.4,
                z: 6.2,
                w: TupleKind::Point,
            }
        );

        let p4 = p1
            + Tuple {
                x: 8.65,
                y: -8.4,
                z: 6.2,
                w: TupleKind::Vector,
            };
        assert_eq!(p4.w, TupleKind::Point);
        assert_eq!((p1 - p2).w, TupleKind::Point);
    }
    #[test]
    fn can_mul_div() {
        let p2 = Tuple {
            x: 4.35,
            y: -4.2,
            z: 3.1,
            w: TupleKind::Vector,
        };

        assert_eq!(
            p2 * 2.0,
            Tuple {
                x: 8.6,
                y: -8.4,
                z: 6.2,
                w: TupleKind::Vector
            }
        )
    }
    #[test]
    fn magnitude() {
        let p2 = Tuple {
            x: 4.35,
            y: -4.2,
            z: 3.1,
            w: TupleKind::Vector,
        };
        let mag = p2.magnitude();
        println!("Magnitude : {}", mag);
    }
    #[test]
    fn normalize() {
        let p2 = Tuple {
            x: 4.35,
            y: -4.2,
            z: 3.1,
            w: TupleKind::Vector,
        };
        let normalized = p2.normalize();
        println!("Normalized : {:#?}", normalized);
        println!("Normalized magnitude : {:#?}", normalized.magnitude());
    }
    #[test]
    fn dot() {
        let p2 = Tuple {
            x: 4.35,
            y: -4.2,
            z: 3.1,
            w: TupleKind::Vector,
        };
        let doted = p2.dot(p2);
        println!("Doted : {:#?}", doted);
    }
    #[test]
    fn cross() {
        let p1 = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: TupleKind::Vector,
        };
        let p2 = Tuple {
            x: 2.0,
            y: 3.0,
            z: 4.0,
            w: TupleKind::Vector,
        };
        let crossed = p1.cross(p2);
        println!("Doted : {:#?}", crossed);
        assert_eq!(crossed.x, -1.0);
        assert_eq!(crossed.y, 2.0);
        assert_eq!(crossed.z, -1.0);
    }
    #[test]
    fn ticking() {
        let mut projectile = Projectile {
            position: Tuple {
                x: 0.,
                y: 0.,
                z: 0.,
                w: TupleKind::Point,
            },
            velocity: Tuple {
                x: 1.,
                y: 1.,
                z: 0.,
                w: TupleKind::Vector,
            },
        };
        let mut i = 0;
        let env = Environment {
            gravity: Tuple {
                x: 0.,
                y: 0.,
                z: 1.,
                w: TupleKind::Vector,
            },
            wind: Tuple {
                x: 0.,
                y: 0.,
                z: 0.,
                w: TupleKind::Vector,
            },
        };
        //let mut proj = projectile;
        while i < 10 {
            projectile = tick(env, projectile);
            println!("{:#?}", projectile);
            i += 1;
        }
    }
}
