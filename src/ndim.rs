#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NDim<const N: usize, T>(pub [T; N]);

impl<const N: usize, T: PartialEq> PartialEq<[T; N]> for NDim<N, T> {
    fn eq(&self, other: &[T; N]) -> bool {
        self.0 == *other
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Xy<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Xyz<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Xyzw<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> std::ops::Deref for NDim<2, T> {
    type Target = Xy<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T> std::ops::DerefMut for NDim<2, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T> std::ops::Deref for NDim<3, T> {
    type Target = Xyz<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T> std::ops::DerefMut for NDim<3, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T> std::ops::Deref for NDim<4, T> {
    type Target = Xyzw<T>;
    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}
impl<T> std::ops::DerefMut for NDim<4, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<const N: usize, T> NDim<N, T> {
    pub fn as_array(&self) -> &[T; N] {
        &self.0
    }
    pub fn as_array_mut(&mut self) -> &mut [T; N] {
        &mut self.0
    }
    pub fn into_array(self) -> [T; N] {
        self.0
    }
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.0.iter()
    }
}
impl<T> NDim<2, T> {
    pub fn new(x: T, y: T) -> Self {
        Self([x, y])
    }
}
impl<T> NDim<3, T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self([x, y, z])
    }
}
impl<T> NDim<4, T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self([x, y, z, w])
    }
}
impl<const N: usize, T> std::ops::Index<usize> for NDim<N, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl<const N: usize, T> std::ops::IndexMut<usize> for NDim<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
impl<const N: usize, T> From<[T; N]> for NDim<N, T> {
    fn from(array: [T; N]) -> Self {
        Self(array)
    }
}
impl<const N: usize, T> From<NDim<N, T>> for [T; N] {
    fn from(ndim: NDim<N, T>) -> Self {
        ndim.0
    }
}
impl<const N: usize, T> IntoIterator for NDim<N, T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<'a, const N: usize, T> IntoIterator for &'a NDim<N, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
