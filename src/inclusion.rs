use std::marker::PhantomData;

use crate::traits::{Boundary, BoundaryOf, Flip, IntoGeneral};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Inclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Exclusive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoundType {
    Inclusive,
    Exclusive,
}

#[derive(Debug, Clone, Copy)]
pub struct Left;

#[derive(Debug, Clone, Copy)]
pub struct Right;

#[derive(Debug, Clone, Copy)]
pub struct SideInclusion<B, S>(B, PhantomData<S>);

mod ordering {
    use super::{Left, Right, SideInclusion};
    use crate::{BoundType, Exclusive, Inclusive};

    impl<B: PartialEq, S> PartialEq for SideInclusion<B, S> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
    impl<B: Eq, S> Eq for SideInclusion<B, S> {}

    macro_rules! impl_ord {
        (($lhs:ident, $rhs:ident): $type:ty => $body:expr) => {
            impl PartialOrd for $type {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    Some(self.cmp(other))
                }
            }
            impl Ord for $type {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    let $lhs = self;
                    let $rhs = other;
                    $body
                }
            }
        };
    }

    impl_ord!((_lhs, _rhs): SideInclusion<Inclusive, Left> => std::cmp::Ordering::Equal);
    impl_ord!((_lhs, _rhs): SideInclusion<Exclusive, Left> => std::cmp::Ordering::Equal);
    impl_ord!((_lhs, _rhs): SideInclusion<Inclusive, Right> => std::cmp::Ordering::Equal);
    impl_ord!((_lhs, _rhs): SideInclusion<Exclusive, Right> => std::cmp::Ordering::Equal);
    impl_ord!((lhs, rhs): SideInclusion<BoundType, Left> => match (lhs.0, rhs.0) {
        (BoundType::Inclusive, BoundType::Inclusive) => std::cmp::Ordering::Equal,
        (BoundType::Inclusive, BoundType::Exclusive) => std::cmp::Ordering::Less,
        (BoundType::Exclusive, BoundType::Inclusive) => std::cmp::Ordering::Greater,
        (BoundType::Exclusive, BoundType::Exclusive) => std::cmp::Ordering::Equal,
    });
    impl_ord!((lhs, rhs): SideInclusion<BoundType, Right> => match (lhs.0, rhs.0) {
        (BoundType::Inclusive, BoundType::Inclusive) => std::cmp::Ordering::Equal,
        (BoundType::Inclusive, BoundType::Exclusive) => std::cmp::Ordering::Greater,
        (BoundType::Exclusive, BoundType::Inclusive) => std::cmp::Ordering::Less,
        (BoundType::Exclusive, BoundType::Exclusive) => std::cmp::Ordering::Equal,
    });
}

impl IntoGeneral for Inclusive {
    type General = BoundType;
    fn into_general(self) -> Self::General {
        BoundType::Inclusive
    }
}
impl IntoGeneral for Exclusive {
    type General = BoundType;
    fn into_general(self) -> Self::General {
        BoundType::Exclusive
    }
}

impl Flip for Inclusive {
    type Flip = Exclusive;
    fn flip(self) -> Self::Flip {
        Exclusive
    }
}
impl Flip for Exclusive {
    type Flip = Inclusive;
    fn flip(self) -> Self::Flip {
        Inclusive
    }
}
impl Flip for BoundType {
    type Flip = Self;
    fn flip(self) -> Self {
        match self {
            Self::Inclusive => Self::Exclusive,
            Self::Exclusive => Self::Inclusive,
        }
    }
}
impl Flip for Left {
    type Flip = Right;
    fn flip(self) -> Self::Flip {
        Right
    }
}
impl Flip for Right {
    type Flip = Left;
    fn flip(self) -> Self::Flip {
        Left
    }
}

impl Boundary for Inclusive {
    fn less<T: PartialOrd>(&self, this: &T, t: &T) -> bool {
        this <= t
    }
}
impl Boundary for Exclusive {
    fn less<T: PartialOrd>(&self, this: &T, t: &T) -> bool {
        this < t
    }
}
impl Boundary for BoundType {
    fn less<T: PartialOrd>(&self, s: &T, t: &T) -> bool {
        match self {
            BoundType::Inclusive => s <= t,
            BoundType::Exclusive => s < t,
        }
    }
}

impl<LR> BoundaryOf<LR> for Inclusive
where
    SideInclusion<Self, LR>: Ord,
{
    type Ordered = SideInclusion<Self, LR>;
    fn into_ordered(self) -> Self::Ordered {
        SideInclusion(self, PhantomData)
    }
}
impl<LR> BoundaryOf<LR> for Exclusive
where
    SideInclusion<Self, LR>: Ord,
{
    type Ordered = SideInclusion<Self, LR>;
    fn into_ordered(self) -> Self::Ordered {
        SideInclusion(self, PhantomData)
    }
}
impl<LR> BoundaryOf<LR> for BoundType
where
    SideInclusion<Self, LR>: Ord,
{
    type Ordered = SideInclusion<Self, LR>;
    fn into_ordered(self) -> Self::Ordered {
        SideInclusion(self, PhantomData)
    }
}
