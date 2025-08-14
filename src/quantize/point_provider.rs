use crate::util::color::ARGB;

pub type Point = [f64; 3];

/// A trait allowing quantizers to use different color spaces.
pub trait PointProvider {
    fn to_int(&self, point: Point) -> ARGB;
    #[allow(clippy::wrong_self_convention)]
    fn from_int(&self, argb: ARGB) -> Point;
    fn distance(&self, from: Point, to: Point) -> f64;
}
