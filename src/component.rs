use crate::untyped_component::UntypedComponent;
use force_derive::ForceDefault;
use gen_id_allocator::ValidId;
use iter_context::{ContextualIterator, FromContextualIterator, Iter, IterMut};
use ref_cast::RefCast;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut, Neg, Not};

#[repr(transparent)]
#[derive(Debug, ForceDefault, RefCast)]
pub struct Component<Arena, T> {
    values: UntypedComponent<T>,
    marker: PhantomData<Arena>,
}

impl<Arena, T: Clone> Clone for Component<Arena, T> {
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

impl<Arena, T> From<Vec<T>> for Component<Arena, T> {
    #[inline]
    fn from(values: Vec<T>) -> Self {
        Component {
            values: values.into(),
            marker: PhantomData,
        }
    }
}

impl<Arena, T> Component<Arena, T> {
    #[inline]
    pub fn insert<Id: ValidId<Arena = Arena>>(&mut self, id: Id, value: T) {
        self.values.insert_with(id.id().untyped, value, || panic!());
    }

    #[inline]
    pub fn get<Id: ValidId<Arena = Arena>>(&self, id: Id) -> Option<&T> {
        self.values.get(id.id().untyped)
    }

    #[inline]
    pub fn get_mut<Id: ValidId<Arena = Arena>>(&mut self, id: Id) -> Option<&mut T> {
        self.values.get_mut(id.id().untyped)
    }

    #[inline]
    pub fn swap<IdA, IdB>(&mut self, a: IdA, b: IdB)
    where
        IdA: ValidId<Arena = Arena>,
        IdB: ValidId<Arena = Arena>,
    {
        self.values.swap(a.id().untyped, b.id().untyped);
    }

    #[inline]
    pub fn fill_with<F: FnMut() -> T>(&mut self, f: F) {
        self.values.fill_with(f);
    }

    #[inline]
    pub fn iter(&self) -> Iter<Arena, T> {
        Iter::new(&self.values)
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<Arena, T> {
        IterMut::new(&mut self.values)
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

impl<Arena, T> Component<Arena, Option<T>> {
    #[inline]
    pub fn remove<Id: ValidId<Arena = Arena>>(&mut self, id: Id) -> Option<T> {
        let value = self.values.get_mut(id.id().untyped)?;
        std::mem::take(value)
    }
}

impl<'a, Arena, T: Copy + 'a> Component<Arena, T> {
    #[inline]
    pub fn assign<Rhs: ContextualIterator<Context = Arena, Item = T>>(&mut self, rhs: Rhs) {
        self.zip(rhs).for_each(|(lhs, rhs)| *lhs = rhs);
    }

    #[inline]
    pub fn assign_ref<Rhs: ContextualIterator<Context = Arena, Item = &'a T>>(&mut self, rhs: Rhs) {
        self.zip(rhs).for_each(|(lhs, rhs)| *lhs = *rhs);
    }
}

impl<Arena, T, V: ValidId<Arena = Arena>> Index<V> for Component<Arena, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: V) -> &Self::Output {
        self.values.index(index.id().untyped)
    }
}

impl<Arena, T, V: ValidId<Arena = Arena>> IndexMut<V> for Component<Arena, T> {
    #[inline]
    fn index_mut(&mut self, index: V) -> &mut Self::Output {
        self.values.index_mut(index.id().untyped)
    }
}

impl<'a, Arena, T> IntoIterator for &'a Component<Arena, T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}

impl<'a, Arena, T> IntoIterator for &'a mut Component<Arena, T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}

impl<'a, Arena, T> ContextualIterator for &'a Component<Arena, T> {
    type Context = Arena;
}

impl<'a, Arena, T> ContextualIterator for &'a mut Component<Arena, T> {
    type Context = Arena;
}

impl<Arena, T> FromContextualIterator<T> for Component<Arena, T> {
    type Context = Arena;

    #[inline]
    fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: ContextualIterator<Context = Self::Context, Item = T>,
    {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl<'a, Arena, T: 'a + Copy> FromContextualIterator<&'a T> for Component<Arena, T> {
    type Context = Arena;

    #[inline]
    fn from_iter<Iter>(iter: Iter) -> Self
    where
        Iter: ContextualIterator<Context = Self::Context, Item = &'a T>,
    {
        iter.into_iter().copied().collect::<Vec<_>>().into()
    }
}

macro_rules! impl_op_assign {
    ($op_trait:ident, $op_fn:ident) => {
        impl<Arena, T, Rhs> std::ops::$op_trait<Rhs> for $crate::Component<Arena, T>
        where
            Rhs: iter_context::ContextualIterator<Context = Arena>,
            T: std::ops::$op_trait<Rhs::Item>,
        {
            #[inline]
            fn $op_fn(&mut self, rhs: Rhs) {
                self.zip(rhs).for_each(|(lhs, rhs)| lhs.$op_fn(rhs));
            }
        }
    };
}

impl_op_assign!(AddAssign, add_assign);
impl_op_assign!(SubAssign, sub_assign);
impl_op_assign!(MulAssign, mul_assign);
impl_op_assign!(DivAssign, div_assign);
impl_op_assign!(RemAssign, rem_assign);
impl_op_assign!(BitAndAssign, bitand_assign);
impl_op_assign!(BitOrAssign, bitor_assign);
impl_op_assign!(BitXorAssign, bitxor_assign);
impl_op_assign!(ShlAssign, shl_assign);
impl_op_assign!(ShrAssign, shr_assign);

macro_rules! impl_op {
    ($op_trait:ident, $op_fn:ident) => {
        impl<'a, Arena, T, Rhs, Output> std::ops::$op_trait<Rhs> for &'a $crate::Component<Arena, T>
        where
            &'a T: std::ops::$op_trait<Rhs::Item, Output = Output>,
            Rhs: iter_context::ContextualIterator<Context = Arena>,
        {
            type Output = iter_context::Map<
                Arena,
                iter_context::Zip<Arena, Self, Rhs>,
                fn((&'a T, Rhs::Item)) -> Output,
            >;

            #[inline]
            fn $op_fn(self, rhs: Rhs) -> Self::Output {
                self.zip(rhs).map(|(lhs, rhs)| lhs.$op_fn(rhs))
            }
        }
    };
}

impl_op!(Add, add);
impl_op!(Sub, sub);
impl_op!(Mul, mul);
impl_op!(Div, div);
impl_op!(Rem, rem);
impl_op!(Shl, shl);
impl_op!(Shr, shr);
impl_op!(BitAnd, bitand);
impl_op!(BitOr, bitor);
impl_op!(BitXor, bitxor);

impl<'a, Arena, T> Not for &'a Component<Arena, T>
where
    &'a T: Not,
{
    type Output = iter_context::Map<Arena, Self, fn(&'a T) -> <&'a T as Not>::Output>;

    #[inline]
    fn not(self) -> Self::Output {
        self.map(Not::not)
    }
}

impl<'a, Arena, T> Neg for &'a Component<Arena, T>
where
    &'a T: Neg,
{
    type Output = iter_context::Map<Arena, Self, fn(&'a T) -> <&'a T as Neg>::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        self.map(Neg::neg)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use gen_id_allocator::{Id, Valid};

    fn get_id(i: usize) -> Valid<'static, Id<()>> {
        Valid::assert(Id::first(i))
    }

    #[test]
    #[should_panic]
    fn insert_beyond_end_panics() {
        let mut comp = Component::<(), ()>::default();
        let id = get_id(1);

        comp.insert(id, ());
    }

    #[allow(dead_code)]
    fn assignment_ops(comp: &Component<(), u32>, target: &mut Component<(), u32>) {
        *target += comp + comp;
        *target += comp;
        *target -= comp;
        *target *= comp;
        *target /= comp;
        *target %= comp;
        *target &= comp;
        *target |= comp;
        *target ^= comp;
        *target <<= comp;
        *target >>= comp;
    }

    #[test]
    fn add_then_add_assign() {
        let mut ones = Component::<(), u32>::default();
        let mut target = Component::<(), u32>::default();
        let id = get_id(0);
        ones.insert(id, 1);
        target.insert(id, 0);

        target += &ones + &ones;

        assert_eq!(2, target[id]);
    }

    #[test]
    fn assign() {
        let mut ones = Component::<(), u32>::default();
        let mut target = Component::<(), u32>::default();
        let id = get_id(0);
        ones.insert(id, 1);
        target.insert(id, 0);

        target.assign(&ones + &ones);

        assert_eq!(2, target[id]);
    }

    #[test]
    fn assign_ref() {
        let mut ones = Component::<(), u32>::default();
        let mut target = Component::<(), u32>::default();
        let id = get_id(0);
        ones.insert(id, 1);
        target.insert(id, 0);

        target.assign_ref(&ones);

        assert_eq!(1, target[id]);
    }

    #[test]
    fn lots_of_math() {
        let mut target = Component::<(), u32>::from(vec![0, 0, 0]);
        let ints = Component::<(), u32>::from(vec![1, 2, 3]);
        let primes = Component::<(), u32>::from(vec![2, 3, 5]);

        target.assign((&primes + &ints) * &primes);

        assert_eq!(Vec::from(target.values), vec![6, 15, 40]);
    }

    #[test]
    fn remove() {
        let id = get_id(0);
        let mut comp = Component::<(), Option<u32>>::default();

        assert_eq!(None, comp.remove(id));

        comp.insert(id, Some(5));

        assert_eq!(Some(5), comp.remove(id));
        assert_eq!(None, comp.remove(id));
    }
}
