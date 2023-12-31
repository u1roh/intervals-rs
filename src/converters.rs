use crate::{traits::IntoGeneral, Bound, BoundType, Exclusive, Inclusive, Interval};

impl<T> From<T> for Bound<T, Inclusive> {
    fn from(t: T) -> Self {
        Self {
            limit: t,
            bound_type: Inclusive,
        }
    }
}
impl<T> From<T> for Bound<T, Exclusive> {
    fn from(t: T) -> Self {
        Self {
            limit: t,
            bound_type: Exclusive,
        }
    }
}

/// ```
/// use inter_val::{BoundType, Inclusive, Interval};
/// let src: Interval<i32, Inclusive> = Inclusive.at(0).to(Inclusive.at(10));
/// let dst: Interval<i32, BoundType> = src.into();
/// assert_eq!(dst.left().bound_type, BoundType::Inclusive);
/// assert_eq!(dst.right().bound_type, BoundType::Inclusive);
/// ```
impl<T> From<Interval<T, Inclusive>> for Interval<T, BoundType> {
    fn from(i: Interval<T, Inclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use inter_val::{BoundType, Exclusive, Interval};
/// let src: Interval<i32, Exclusive> = Exclusive.at(0).to(Exclusive.at(10));
/// let dst: Interval<i32, BoundType> = src.into();
/// assert_eq!(dst.left().bound_type, BoundType::Exclusive);
/// assert_eq!(dst.right().bound_type, BoundType::Exclusive);
/// ```
impl<T> From<Interval<T, Exclusive>> for Interval<T, BoundType> {
    fn from(i: Interval<T, Exclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use inter_val::{BoundType, Inclusive, Exclusive, Interval};
/// let src: Interval<i32, Inclusive, Exclusive> = Inclusive.at(0).to(Exclusive.at(10));
/// let dst: Interval<i32, BoundType> = src.into();
/// assert_eq!(dst.left().bound_type, BoundType::Inclusive);
/// assert_eq!(dst.right().bound_type, BoundType::Exclusive);
/// ```
impl<T> From<Interval<T, Inclusive, Exclusive>> for Interval<T, BoundType> {
    fn from(i: Interval<T, Inclusive, Exclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use inter_val::{BoundType, Inclusive, Exclusive, Interval};
/// let src: Interval<i32, Exclusive, Inclusive> = Exclusive.at(0).to(Inclusive.at(10));
/// let dst: Interval<i32, BoundType> = src.into();
/// assert_eq!(dst.left().bound_type, BoundType::Exclusive);
/// assert_eq!(dst.right().bound_type, BoundType::Inclusive);
/// ```
impl<T> From<Interval<T, Exclusive, Inclusive>> for Interval<T, BoundType> {
    fn from(i: Interval<T, Exclusive, Inclusive>) -> Self {
        i.into_general()
    }
}

/// ```
/// use std::any::{Any, TypeId};
/// use inter_val::{Inclusive, Interval};
/// let a: Interval<_, _, _> = 3.into();
/// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Inclusive, Inclusive>>());
/// assert_eq!(a.left().limit, 3);
/// assert_eq!(a.right().limit, 3);
impl<T: PartialOrd + Clone> From<T> for Interval<T, Inclusive> {
    fn from(t: T) -> Self {
        Self::new(t.clone().into(), t.into())
    }
}
