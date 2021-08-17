use crate::untyped_component::UntypedComponent;
use force_derive::ForceDefault;
use gen_id_allocator::Id;
use iter_context::{ContextualIterator, FromContextualIterator, Iter, IterMut};
use ref_cast::RefCast;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

#[repr(transparent)]
#[derive(Debug, ForceDefault, RefCast)]
pub struct RawComponent<Arena, T> {
    values: UntypedComponent<T>,
    marker: PhantomData<*const Arena>,
}

impl<Arena, T: Clone> Clone for RawComponent<Arena, T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            marker: PhantomData,
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        self.values.clone_from(&source.values);
    }
}

impl<Arena, T> From<Vec<T>> for RawComponent<Arena, T> {
    #[inline]
    fn from(values: Vec<T>) -> Self {
        RawComponent {
            values: values.into(),
            marker: PhantomData,
        }
    }
}

impl<Arena, T> RawComponent<Arena, T> {
    #[inline]
    pub fn insert(&mut self, id: Id<Arena>, value: T) {
        self.values.insert_with(id.untyped, value, || panic!());
    }

    #[inline]
    pub fn get(&self, id: Id<Arena>) -> Option<&T> {
        self.values.get(id.untyped)
    }

    #[inline]
    pub fn get_mut(&mut self, id: Id<Arena>) -> Option<&mut T> {
        self.values.get_mut(id.untyped)
    }

    #[inline]
    pub fn swap(&mut self, a: Id<Arena>, b: Id<Arena>) {
        self.values.swap(a.untyped, b.untyped);
    }

    #[inline]
    pub fn fill_with<F: FnMut() -> T>(&mut self, f: F) {
        self.values.fill_with(f);
    }

    #[inline]
    pub fn iter(&self) -> Iter<Arena, T> {
        Iter::new(self.values.iter())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<Arena, T> {
        IterMut::new(self.values.iter_mut())
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

impl<Arena, T> RawComponent<Arena, Option<T>> {
    #[inline]
    pub fn remove(&mut self, id: Id<Arena>) -> Option<T> {
        let value = self.values.get_mut(id.untyped)?;
        std::mem::take(value)
    }
}

impl<'a, Arena, T: Copy + 'a> RawComponent<Arena, T> {
    #[inline]
    pub fn assign<Rhs: ContextualIterator<Context = Arena, Item = T>>(&mut self, rhs: Rhs) {
        self.zip(rhs).for_each(|(lhs, rhs)| *lhs = rhs);
    }

    #[inline]
    pub fn assign_ref<Rhs: ContextualIterator<Context = Arena, Item = &'a T>>(&mut self, rhs: Rhs) {
        self.zip(rhs).for_each(|(lhs, rhs)| *lhs = *rhs);
    }
}

impl<Arena, T> Index<Id<Arena>> for RawComponent<Arena, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: Id<Arena>) -> &Self::Output {
        self.values.index(index.untyped)
    }
}

impl<Arena, T> IndexMut<Id<Arena>> for RawComponent<Arena, T> {
    #[inline]
    fn index_mut(&mut self, index: Id<Arena>) -> &mut Self::Output {
        self.values.index_mut(index.untyped)
    }
}

impl<Arena, T> Index<&Id<Arena>> for RawComponent<Arena, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: &Id<Arena>) -> &Self::Output {
        self.values.index(index.untyped)
    }
}

impl<Arena, T> IndexMut<&Id<Arena>> for RawComponent<Arena, T> {
    #[inline]
    fn index_mut(&mut self, index: &Id<Arena>) -> &mut Self::Output {
        self.values.index_mut(index.untyped)
    }
}

impl<'a, Arena, T> IntoIterator for &'a RawComponent<Arena, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<'a, Arena, T> IntoIterator for &'a mut RawComponent<Arena, T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}

impl<'a, Arena, T> ContextualIterator for &'a RawComponent<Arena, T> {
    type Context = Arena;
}

impl<'a, Arena, T> ContextualIterator for &'a mut RawComponent<Arena, T> {
    type Context = Arena;
}

impl<Arena, T> FromContextualIterator<T> for RawComponent<Arena, T> {
    type Context = Arena;

    #[inline]
    fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: ContextualIterator<Context = Self::Context, Item = T>,
    {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl<'a, Arena, T: 'a + Copy> FromContextualIterator<&'a T> for RawComponent<Arena, T> {
    type Context = Arena;

    #[inline]
    fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: ContextualIterator<Context = Self::Context, Item = &'a T>,
    {
        iter.into_iter().copied().collect::<Vec<_>>().into()
    }
}
