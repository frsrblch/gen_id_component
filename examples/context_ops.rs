#![allow(dead_code)]

use iter_context::ContextualIterator;

fn main() {}

struct Enum;

struct EnumArray<T>([T; 4]);

impl<'a, T> IntoIterator for &'a EnumArray<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut EnumArray<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<'a, T> ContextualIterator for &'a EnumArray<T> {
    type Context = Enum;
}

impl<'a, T> ContextualIterator for &'a mut EnumArray<T> {
    type Context = Enum;
}

// fn add_assign_enum_arrays<Source, Target: Fixed, T>(
//     sv: &EnumArray<Component<Source, T>>,
//     st: &EnumArray<Component<Source, Option<Id<Target>>>>,
//     tv: &mut EnumArray<Component<Target, T>>,
// ) where
//     for<'a> T: AddAssign<&'a T>,
// {
//     for ((sv, st), tv) in sv.zip(st).zip(tv) {
//         *tv += sv.map_to(st);
//     }
// }
