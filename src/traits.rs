use ordered_float::{FloatCore, FloatIsNan, NotNan};

pub trait Flip {
    type Flip: Flip<Flip = Self>;
    fn flip(self) -> Self::Flip;
}

pub trait Boundary: Flip + Eq + Copy {
    fn less<T: Ord>(&self, this: &T, t: &T) -> bool;
}

pub trait BoundaryOf<LR>: Boundary {
    type Ordered: Ord;
    fn into_ordered(self) -> Self::Ordered;
}

pub trait Minimum<T> {
    fn minimum(&self) -> T;
}

pub trait Maximum<T> {
    fn maximum(&self) -> T;
}

pub(crate) trait IntoGeneral {
    type General;
    fn into_general(self) -> Self::General;
}

pub trait OrdFrom<T>: Ord + Sized {
    type Error;
    fn ord_from(t: T) -> Result<Self, Self::Error>;
}
impl<T: Ord> OrdFrom<T> for T {
    type Error = std::convert::Infallible;
    fn ord_from(t: T) -> Result<Self, Self::Error> {
        Ok(t)
    }
}
impl<T: FloatCore> OrdFrom<T> for NotNan<T> {
    type Error = FloatIsNan;
    fn ord_from(t: T) -> Result<Self, Self::Error> {
        NotNan::new(t)
    }
}
