use ordered_float::{FloatCore, FloatIsNan, NotNan};

use crate::bounding::{Left, Right};
use crate::traits::{BoundaryOf, Flip, IntoGeneral, Maximum, Minimum, Scalar};
use crate::{Bound, Exclusive, Inclusive, LeftBounded, RightBounded};

/// Return type of `Interval::union()`.
pub struct IntervalUnion<T, L: Flip, R: Flip> {
    pub enclosure: Interval<T, L, R>,
    pub gap: Option<Interval<T, R::Flip, L::Flip>>,
}
impl<T, L: Flip, R: Flip> IntoIterator for IntervalUnion<T, L, R> {
    type Item = Interval<T, L, R>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        if let Some(gap) = self.gap {
            let first = Interval {
                left: self.enclosure.left,
                right: gap.left.flip(),
            };
            let second = Interval {
                left: gap.right.flip(),
                right: self.enclosure.right,
            };
            vec![first, second].into_iter()
        } else {
            vec![self.enclosure].into_iter()
        }
    }
}

fn is_valid_interval<T, L, R>(left: &LeftBounded<T, L>, right: &RightBounded<T, R>) -> bool
where
    T: Ord,
    L: BoundaryOf<Left>,
    R: BoundaryOf<Right>,
{
    left.contains(&right.val) && right.contains(&left.val)
}

/// Interval like *[a, b]*, *(a, b)*, *[a, b)*, and *(a, b]* for any `Ord` type.
/// * `T`: Scalar type. `T` should implements `Ord`. Use `ordered_float::NotNan<T>` for floating point numbers.
/// * `L`: Left boundary type. One of `Bounding`, `Inclusive` or `Exclusive`.
/// * `R`: Right boundary type. One of `Bounding`, `Inclusive` or `Exclusive`.
/// * `Interval<T, Inclusive>` represents a closed interval, i.e., *[a, b]*.
/// * `Interval<T, Exclusive>` represents a open interval, i.e., *(a, b)*.
/// * `Interval<T, Inclusive, Exclusive>` represents a right half-open interval, i.e., *[a, b)*.
/// * `Interval<T, Exclusive, Inclusive>` represents a left half-open interval, i.e., *(a, b]*.
/// * `Interval<T>` (= `Interval<T, Bounding, Bounding>`) represents any of the above.
#[derive(Debug, Clone, Copy, Eq)]
pub struct Interval<T, L = crate::Bounding, R = L> {
    pub(crate) left: LeftBounded<T, L>,
    pub(crate) right: RightBounded<T, R>,
}
impl<T: Eq, L: Eq, R: Eq> PartialEq for Interval<T, L, R> {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.right == other.right
    }
}
impl<T, L, R> Interval<T, L, R> {
    pub fn left(&self) -> &LeftBounded<T, L> {
        &self.left
    }
    pub fn right(&self) -> &RightBounded<T, R> {
        &self.right
    }
}
impl<T: Ord, L: BoundaryOf<Left>, R: BoundaryOf<Right>> Interval<T, L, R> {
    fn new_(left: LeftBounded<T, L>, right: RightBounded<T, R>) -> Option<Self> {
        is_valid_interval(&left, &right).then_some(Self { left, right })
    }

    /// Create a new interval.
    /// ```
    /// use std::any::{Any, TypeId};
    /// use intervals::{Interval, Bounding, Exclusive, Inclusive};
    ///
    /// let a: Interval<i32, Inclusive, Exclusive> = Interval::new(0.into(), 3.into()).unwrap();
    /// assert!(a.contains(&0));
    /// assert!(a.contains(&2));
    /// assert!(!a.contains(&3));
    ///
    /// let a = Interval::new(Exclusive.at(0), Inclusive.at(3)).unwrap();
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Exclusive, Inclusive>>());
    ///
    /// let a = Interval::new(Bounding::Exclusive.at(0), Bounding::Exclusive.at(3)).unwrap();
    /// assert_eq!(a.type_id(), TypeId::of::<Interval<i32, Bounding, Bounding>>());
    ///
    /// assert!(Interval::new(Inclusive.at(3), Exclusive.at(0)).is_none());
    /// assert!(Interval::new(Inclusive.at(3), Exclusive.at(3)).is_none());
    /// assert!(Interval::new(Inclusive.at(3), Inclusive.at(3)).is_some());
    /// ```
    pub fn new(left: Bound<T, L>, right: Bound<T, R>) -> Option<Self> {
        Self::new_(left.into(), right.into())
    }

    /// ```
    /// use intervals::{IntervalF, Exclusive, Inclusive};
    /// let a = IntervalF::try_new(Inclusive.at(-1.0), Exclusive.at(1.0)).unwrap().unwrap();
    /// assert!(a.contains(&-1.0));
    /// assert!(!a.contains(&1.0));
    ///
    /// let a = IntervalF::<_, Exclusive, Inclusive>::try_new(1.23.into(), 4.56.into())
    ///     .unwrap()
    ///     .unwrap();
    /// assert!(!a.contains(&1.23));
    /// assert!(a.contains(&1.23000000000001));
    /// assert!(a.contains(&4.56));
    /// ```
    pub fn try_new<T2>(left: Bound<T2, L>, right: Bound<T2, R>) -> Result<Option<Self>, T::Error>
    where
        T: Scalar<T2>,
    {
        let left = Bound {
            val: T::scalar_try_from(left.val)?,
            bounding: left.bounding,
        };
        let right = Bound {
            val: T::scalar_try_from(right.val)?,
            bounding: right.bounding,
        };
        Ok(Self::new(left, right))
    }

    /// ```
    /// use intervals::{Interval, Exclusive, Inclusive};
    /// let a: Interval<i32, Inclusive, Exclusive> = Interval::between(-2, 5).unwrap();
    /// assert_eq!(a, Inclusive.at(-2).to(Exclusive.at(5)).unwrap());
    /// ```
    pub fn between(left: T, right: T) -> Option<Self>
    where
        T: Into<Bound<T, L>> + Into<Bound<T, R>>,
    {
        Self::new(left.into(), right.into())
    }

    /// ```
    /// use intervals::{IntervalF, Exclusive, Inclusive};
    /// let a: IntervalF<f64, Inclusive, Exclusive> = IntervalF::try_between(-1.0, 1.0).unwrap().unwrap();
    /// assert_eq!(a, Inclusive.at(-1.0).float_to(Exclusive.at(1.0)).unwrap());
    /// ```
    pub fn try_between<T2>(left: T2, right: T2) -> Result<Option<Self>, T::Error>
    where
        T: Scalar<T2> + Into<Bound<T, L>> + Into<Bound<T, R>>,
    {
        Ok(Self::new(
            T::scalar_try_from(left)?.into(),
            T::scalar_try_from(right)?.into(),
        ))
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Inclusive.at(7)).unwrap();
    /// let b = Exclusive.at(4).to(Inclusive.at(7)).unwrap();
    /// let c = Inclusive.at(1.23).float_to(Inclusive.at(4.56)).unwrap();
    /// assert_eq!(a.min(), 4);
    /// assert_eq!(b.min(), 5);
    /// assert_eq!(c.min(), 1.23);
    /// ```
    pub fn min(&self) -> T
    where
        LeftBounded<T, L>: Minimum<T>,
    {
        self.left.minimum()
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Inclusive.at(7)).unwrap();
    /// let b = Inclusive.at(4).to(Exclusive.at(7)).unwrap();
    /// let c = Inclusive.at(1.23).float_to(Inclusive.at(4.56)).unwrap();
    /// assert_eq!(a.max(), 7);
    /// assert_eq!(b.max(), 6);
    /// assert_eq!(c.max(), 4.56);
    /// ```
    pub fn max(&self) -> T
    where
        RightBounded<T, R>: Maximum<T>,
    {
        self.right.maximum()
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(4).to(Exclusive.at(7)).unwrap();
    /// let b = Exclusive.at(1.23).float_to(Inclusive.at(4.56)).unwrap();
    /// assert!(a.contains(&4));
    /// assert!(!a.contains(&7));
    /// assert!(!b.contains(&1.23));
    /// assert!(b.contains(&1.230000000001));
    /// assert!(b.contains(&4.56));
    /// ```
    pub fn contains<T2>(&self, t: &T2) -> bool
    where
        T: Scalar<T2>,
    {
        self.left.contains(t) && self.right.contains(t)
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(0).to(Exclusive.at(4)).unwrap();
    /// let c = Inclusive.at(1).to(Exclusive.at(4)).unwrap();
    /// assert!(a.includes(&a));
    /// assert!(!a.includes(&b) && b.includes(&a));
    /// assert!(!a.includes(&c) && !c.includes(&a));
    /// ```
    pub fn includes(&self, other: &Self) -> bool {
        self.left.includes(&other.left) && self.right.includes(&other.right)
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(1).to(Exclusive.at(4)).unwrap();
    /// let c = Inclusive.at(3).to(Exclusive.at(5)).unwrap();
    /// assert!(a.overlaps(&a));
    /// assert!(a.overlaps(&b) && b.overlaps(&a));
    /// assert!(!a.overlaps(&c) && !c.overlaps(&a));
    /// ```
    pub fn overlaps(&self, other: &Self) -> bool {
        let left = std::cmp::max(&self.left, &other.left);
        let right = std::cmp::min(&self.right, &other.right);
        is_valid_interval(left, right)
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(1).to(Exclusive.at(4)).unwrap();
    /// let c = Inclusive.at(3).to(Exclusive.at(5)).unwrap();
    /// assert_eq!(a.intersection(a), Some(a));
    /// assert_eq!(a.intersection(b), Inclusive.at(1).to(Exclusive.at(3)));
    /// assert_eq!(a.intersection(c), None);
    /// ```
    pub fn intersection(self, other: Self) -> Option<Self> {
        Self::new_(
            self.left.intersection(other.left),
            self.right.intersection(other.right),
        )
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(5).to(Exclusive.at(8)).unwrap();
    /// assert_eq!(a.enclosure(b), Inclusive.at(0).to(Exclusive.at(8)).unwrap());
    /// ```
    pub fn enclosure(self, other: Self) -> Self {
        Self {
            left: self.left.union(other.left),
            right: self.right.union(other.right),
        }
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(5).to(Exclusive.at(8)).unwrap();
    /// assert_eq!(a.gap(b), Inclusive.at(3).to(Exclusive.at(5)));
    /// ```
    pub fn gap(self, other: Self) -> Option<Interval<T, R::Flip, L::Flip>>
    where
        L::Flip: BoundaryOf<Right>,
        R::Flip: BoundaryOf<Left>,
    {
        Interval::new_(self.right.flip(), other.left.flip())
            .or(Interval::new_(other.right.flip(), self.left.flip()))
    }

    /// ```
    /// use intervals::{Interval, Inclusive, Exclusive};
    /// let a = Inclusive.at(0).to(Exclusive.at(3)).unwrap();
    /// let b = Inclusive.at(5).to(Exclusive.at(8)).unwrap();
    /// let union = a.union(b);
    /// assert_eq!(union.enclosure, a.enclosure(b));
    /// assert_eq!(union.gap, a.gap(b));
    /// let union_ints: Vec<Interval<_, _, _>> = union.into_iter().collect();
    /// assert_eq!(union_ints.len(), 2);
    /// assert_eq!(union_ints[0], a);
    /// assert_eq!(union_ints[1], b);
    /// ```
    pub fn union(self, other: Self) -> IntervalUnion<T, L, R>
    where
        T: Clone,
        L::Flip: BoundaryOf<Right>,
        R::Flip: BoundaryOf<Left>,
    {
        IntervalUnion {
            gap: self.clone().gap(other.clone()),
            enclosure: self.enclosure(other),
        }
    }

    pub fn lower_bound(&self) -> RightBounded<T, L::Flip>
    where
        T: Clone,
    {
        self.left.clone().flip()
    }

    pub fn upper_bound(&self) -> LeftBounded<T, R::Flip>
    where
        T: Clone,
    {
        self.right.clone().flip()
    }

    /// ```
    /// use intervals::Interval;
    /// let span = Interval::enclosure_of_items(vec![3, 9, 2, 5]).unwrap(); // [2, 9]
    /// assert_eq!(span.min(), 2);
    /// assert_eq!(span.max(), 9);
    /// ```
    pub fn enclosure_of_items<A: Into<Self>>(items: impl IntoIterator<Item = A>) -> Option<Self> {
        let mut items = items.into_iter();
        let first = items.next()?.into();
        Some(items.fold(first, |acc, item| acc.enclosure(item.into())))
    }
}

impl<T: FloatCore, L: BoundaryOf<Left>, R: BoundaryOf<Right>> Interval<NotNan<T>, L, R> {
    /// ```
    /// use intervals::{Interval, Exclusive, Inclusive};
    /// let a = Interval::float_new(Inclusive.at(-1.0), Exclusive.at(1.0)).unwrap().unwrap();
    /// assert!(a.contains(&-1.0));
    /// assert!(!a.contains(&1.0));
    /// ```
    pub fn float_new(left: Bound<T, L>, right: Bound<T, R>) -> Result<Option<Self>, FloatIsNan> {
        Self::try_new(left, right)
    }

    /// ```
    /// use intervals::{Interval, Exclusive, Inclusive};
    /// let a: Interval<_, Inclusive, Exclusive> = Interval::float_between(-1.0, 1.0).unwrap().unwrap();
    /// assert!(a.contains(&-1.0));
    /// assert!(!a.contains(&1.0));
    /// ```
    pub fn float_between(left: T, right: T) -> Result<Option<Self>, FloatIsNan>
    where
        T: Into<Bound<T, L>> + Into<Bound<T, R>>,
    {
        Self::float_new(left.into(), right.into())
    }

    /// ```
    /// use intervals::{Interval, Exclusive, Inclusive};
    /// let a = Interval::float_new(Inclusive.at(-1.0), Inclusive.at(1.0)).unwrap().unwrap();
    /// assert_eq!(a.inf(), -1.0);
    /// assert!(a.contains(&-1.0));
    ///
    /// let b = Interval::float_new(Exclusive.at(-1.0), Inclusive.at(1.0)).unwrap().unwrap();
    /// assert_eq!(b.inf(), -1.0);
    /// assert!(!b.contains(&-1.0));
    /// ```
    pub fn inf(&self) -> NotNan<T> {
        self.left.inf()
    }

    /// ```
    /// use intervals::{Interval, Exclusive, Inclusive};
    /// let a = Interval::float_new(Inclusive.at(-1.0), Inclusive.at(1.0)).unwrap().unwrap();
    /// assert_eq!(a.sup(), 1.0);
    /// assert!(a.contains(&1.0));
    ///
    /// let b = Interval::float_new(Inclusive.at(-1.0), Exclusive.at(1.0)).unwrap().unwrap();
    /// assert_eq!(b.sup(), 1.0);
    /// assert!(!b.contains(&1.0));
    /// ```
    pub fn sup(&self) -> NotNan<T> {
        self.right.sup()
    }

    /// ```
    /// use intervals::{Interval, Inclusive};
    /// let a = Inclusive.at(2.1).float_to(Inclusive.at(5.3)).unwrap();
    /// assert_eq!(a.measure(), 5.3 - 2.1);
    ///
    /// let a = Inclusive.at(std::f64::INFINITY).float_to(Inclusive.at(std::f64::INFINITY)).unwrap();
    /// assert!(a.measure().is_nan());
    /// ```
    pub fn measure(&self) -> T {
        *self.right.val - *self.left.val
    }

    /// ```
    /// use intervals::{Interval, Inclusive};
    /// let a = Inclusive.at(2.1).float_to(Inclusive.at(5.3)).unwrap();
    /// assert_eq!(a.center(), (2.1 + 5.3) / 2.0);
    ///
    /// let a = Inclusive.at(std::f64::NEG_INFINITY).float_to(Inclusive.at(std::f64::INFINITY)).unwrap();
    /// assert!(a.center().is_nan());
    /// ```
    pub fn center(&self) -> T {
        (*self.left.val + *self.right.val) / (T::one() + T::one())
    }

    pub fn closure(self) -> Interval<NotNan<T>, Inclusive> {
        Interval {
            left: self.left.closure(),
            right: self.right.closure(),
        }
    }
    pub fn interior(self) -> Option<Interval<NotNan<T>, Exclusive>> {
        Interval::<_, Exclusive>::new_(self.left.interior(), self.right.interior())
    }

    /// ```
    /// use intervals::{Interval, Inclusive};
    /// let a = Inclusive.at(0.0).float_to(Inclusive.at(1.0)).unwrap();
    /// let b = Inclusive.at(0.0).float_to(Inclusive.at(2.0)).unwrap();
    /// let c = Inclusive.at(1.0).float_to(Inclusive.at(2.0)).unwrap();
    /// assert_eq!(a.iou(a), 1.0);
    /// assert_eq!(a.iou(b), 0.5);
    /// assert_eq!(a.iou(c), 0.0);
    /// ```
    pub fn iou(self, other: Self) -> T {
        self.intersection(other)
            .map(|intersection| {
                let union = self.enclosure(other);
                intersection.measure() / union.measure()
            })
            .unwrap_or(T::zero())
    }
}

impl<T, L: IntoGeneral, R: IntoGeneral> IntoGeneral for Interval<T, L, R> {
    type General = Interval<T, L::General, R::General>;
    fn into_general(self) -> Self::General {
        Interval {
            left: self.left.into_general(),
            right: self.right.into_general(),
        }
    }
}

impl<T, L, R> Minimum<T> for Interval<T, L, R>
where
    LeftBounded<T, L>: Minimum<T>,
{
    fn minimum(&self) -> T {
        self.left.minimum()
    }
}

impl<T, L, R> Maximum<T> for Interval<T, L, R>
where
    RightBounded<T, R>: Maximum<T>,
{
    fn maximum(&self) -> T {
        self.right.maximum()
    }
}

/// ```
/// use intervals::{Interval, Exclusive, Inclusive, Bounding};
///
/// // Iterate from Interval<i32, Exclusive, Inclusive>
/// let items: Vec<_> = Exclusive.at(0).to(Inclusive.at(10)).unwrap().into_iter().collect();
/// assert_eq!(items.len(), 10);
/// assert_eq!(items[0], 1);
/// assert_eq!(items.last().unwrap(), &10);
///
/// // Iterate from Interval<i32>
/// let items: Vec<_> = (Bounding::Exclusive.at(0).to(Bounding::Inclusive.at(10)))
///     .unwrap()
///     .into_iter()
///     .collect();
/// assert_eq!(items.len(), 10);
/// assert_eq!(items[0], 1);
/// assert_eq!(items.last().unwrap(), &10);
/// ```
impl<T, L, R> IntoIterator for Interval<T, L, R>
where
    std::ops::RangeInclusive<T>: Iterator<Item = T>,
    Self: Minimum<T> + Maximum<T>,
{
    type Item = T;
    type IntoIter = std::ops::RangeInclusive<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.minimum()..=self.maximum()
    }
}
