use force_derive::ForceDefault;
use ref_cast::RefCast;
use std::ops::{Index, IndexMut};

#[repr(transparent)]
#[derive(Debug, ForceDefault, RefCast)]
pub struct IndexVec<T> {
    values: Vec<T>,
}

impl<T: Clone> Clone for IndexVec<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.values.clone_from(&source.values);
    }
}

impl<T> IndexVec<T> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn insert_with<F: Fn() -> T>(&mut self, index: usize, value: T, fill: F) {
        if let Some(current) = self.values.get_mut(index) {
            *current = value;
        } else {
            if self.len() != index {
                let extend_by = index - self.values.len();
                let iter = std::iter::repeat_with(fill).take(extend_by);
                self.values.extend(iter);
            }
            self.values.push(value);
        }
    }

    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        self.values.get(index)
    }

    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.values.get_mut(index)
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[inline]
    pub fn swap(&mut self, a: usize, b: usize) {
        self.values.swap(a, b);
    }

    #[inline]
    pub fn fill_with<F: FnMut() -> T>(&mut self, f: F) {
        self.values.fill_with(f);
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.values.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
        self.values.iter_mut()
    }
}

impl<T: Default> IndexVec<T> {
    #[inline]
    pub fn insert(&mut self, index: usize, value: T) {
        self.insert_with(index, value, Default::default)
    }
}

impl<T> Index<usize> for IndexVec<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.values.index(index)
    }
}

impl<T> IndexMut<usize> for IndexVec<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.values.index_mut(index)
    }
}

impl<T> From<Vec<T>> for IndexVec<T> {
    #[inline]
    fn from(values: Vec<T>) -> Self {
        Self { values }
    }
}

impl<T> From<IndexVec<T>> for Vec<T> {
    #[inline]
    fn from(values: IndexVec<T>) -> Self {
        values.values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_or_push_at_zero() {
        let mut vec = IndexVec::<u32>::default();

        vec.insert(0, 1);

        assert_eq!(vec![1u32], vec.values);
    }

    #[test]
    fn set_or_push_at_one() {
        let mut vec = IndexVec::<u32>::default();

        vec.insert(1, 1);

        assert_eq!(vec![0u32, 1], vec.values);
    }

    #[test]
    fn set_or_push_twice() {
        let mut vec = IndexVec::<u32>::default();

        vec.insert(1, 1);
        vec.insert(0, 2);

        assert_eq!(vec![2u32, 1], vec.values);
    }
}
