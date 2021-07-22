use crate::index_vec::IndexVec;
use gen_id_allocator::UntypedId;
use ref_cast::RefCast;
use std::ops::{Index, IndexMut};

#[repr(transparent)]
#[derive(Debug, RefCast)]
pub struct UntypedComponent<T> {
    values: IndexVec<T>,
}

impl<T> Default for UntypedComponent<T> {
    #[inline]
    fn default() -> Self {
        Self {
            values: Default::default(),
        }
    }
}

impl<T: Clone> Clone for UntypedComponent<T> {
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

impl<T: Default> UntypedComponent<T> {
    #[inline]
    pub fn insert(&mut self, id: UntypedId, value: T) {
        self.insert_with(id, value, Default::default);
    }
}

impl<T> UntypedComponent<T> {
    #[inline]
    pub fn insert_with<F: Fn() -> T>(&mut self, id: UntypedId, value: T, fill: F) {
        self.values.insert_with(id.index(), value, fill);
    }

    #[inline]
    pub fn get(&self, id: UntypedId) -> Option<&T> {
        self.values.get(id.index())
    }

    #[inline]
    pub fn get_mut(&mut self, id: UntypedId) -> Option<&mut T> {
        self.values.get_mut(id.index())
    }

    #[inline]
    pub fn swap(&mut self, a: UntypedId, b: UntypedId) {
        self.values.swap(a.index(), b.index());
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

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> From<Vec<T>> for UntypedComponent<T> {
    #[inline]
    fn from(values: Vec<T>) -> Self {
        UntypedComponent {
            values: values.into(),
        }
    }
}

impl<T> From<UntypedComponent<T>> for Vec<T> {
    #[inline]
    fn from(component: UntypedComponent<T>) -> Self {
        component.values.into()
    }
}

impl<'a, T> IntoIterator for &'a UntypedComponent<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut UntypedComponent<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}

impl<T> Index<UntypedId> for UntypedComponent<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: UntypedId) -> &Self::Output {
        self.values.index(index.index())
    }
}

impl<T> IndexMut<UntypedId> for UntypedComponent<T> {
    #[inline]
    fn index_mut(&mut self, index: UntypedId) -> &mut Self::Output {
        self.values.index_mut(index.index())
    }
}

impl<T> Index<&UntypedId> for UntypedComponent<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: &UntypedId) -> &Self::Output {
        self.values.index(index.index())
    }
}

impl<T> IndexMut<&UntypedId> for UntypedComponent<T> {
    #[inline]
    fn index_mut(&mut self, index: &UntypedId) -> &mut Self::Output {
        self.values.index_mut(index.index())
    }
}
