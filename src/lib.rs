mod bound;
mod converters;
mod half;
mod inclusion;
mod interval;
mod ndim;
mod pow;
mod std_range;
mod tests;
mod traits;

use inclusion::{Left, Right};
use ordered_float::{FloatCore, NotNan};
use traits::BoundaryOf;

pub use bound::Bound;
pub use half::{LeftBounded, RightBounded};
pub use inclusion::{Bounding, Exclusive, Inclusive};
pub use interval::Interval;
pub use ndim::NDim;

impl Inclusive {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            val: t,
            bounding: self,
        }
    }
}
impl Exclusive {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            val: t,
            bounding: self,
        }
    }
}
impl Bounding {
    pub fn at<T>(self, t: T) -> Bound<T, Self> {
        Bound {
            val: t,
            bounding: self,
        }
    }
}

impl<T: Ord, B: BoundaryOf<Left>> Bound<T, B> {
    pub fn to<R: BoundaryOf<Right>>(
        self,
        right: Bound<T, R>,
    ) -> Result<Interval<T, B, R>, IntervalIsEmpty> {
        Interval::new(self, right)
    }
}

impl<T: FloatCore, B: BoundaryOf<Left>> Bound<T, B> {
    pub fn float_to<R: BoundaryOf<Right>>(
        self,
        right: Bound<T, R>,
    ) -> Result<Interval<NotNan<T>, B, R>, Error> {
        Interval::try_new(self, right)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("left boundary must be less than or equal to right boundary")]
pub struct IntervalIsEmpty;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("infallible")]
    Infallible(#[from] std::convert::Infallible),
    #[error("float is NaN")]
    FloatIsNan(#[from] ordered_float::FloatIsNan),
    #[error("left boundary must be less than or equal to right boundary")]
    IntervalIsEmpty(#[from] IntervalIsEmpty),
}

pub type ClosedInterval<T> = Interval<T, Inclusive>;
pub type OpenInterval<T> = Interval<T, Exclusive>;
pub type RightHalfOpenInterval<T> = Interval<T, Inclusive, Exclusive>;
pub type LeftHalfOpenInterval<T> = Interval<T, Exclusive, Inclusive>;

pub type IntervalF<T, L, R> = Interval<NotNan<T>, L, R>;
pub type ClosedIntervalF<T> = ClosedInterval<NotNan<T>>;
pub type OpenIntervalF<T> = OpenInterval<NotNan<T>>;
pub type RightHalfOpenIntervalF<T> = RightHalfOpenInterval<NotNan<T>>;
pub type LeftHalfOpenIntervalF<T> = LeftHalfOpenInterval<NotNan<T>>;

pub type ClosedIntervalF64 = ClosedIntervalF<f64>;
pub type OpenIntervalF64 = OpenIntervalF<f64>;
pub type RightHalfOpenIntervalF64 = RightHalfOpenIntervalF<f64>;
pub type LeftHalfOpenIntervalF64 = LeftHalfOpenIntervalF<f64>;

pub type IntervalN<const N: usize, T, L = Bounding, R = L> = NDim<N, Interval<T, L, R>>;
pub type Interval2<T, L = Bounding, R = L> = IntervalN<2, T, L, R>;
pub type Interval3<T, L = Bounding, R = L> = IntervalN<3, T, L, R>;
pub type Interval4<T, L = Bounding, R = L> = IntervalN<4, T, L, R>;
pub type BoxN<const N: usize, T> = IntervalN<N, NotNan<T>, Inclusive>;
