use crate::{Component, ContextualIterator};

pub trait FromContextualIterator: ContextualIterator {
    #[inline]
    fn collect(self) -> Component<Self::Context, Self::Item> {
        self.into_iter().collect::<Vec<_>>().into()
    }
}

impl<Iter> FromContextualIterator for Iter where Iter: ContextualIterator {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn collect_contextual_iterator() {
        let primes = Component::<(), u32>::from(vec![2, 3, 5, 7, 11]);

        let primes_squared = primes.iter().map(|v| v * v).collect();

        assert_eq!(vec![4, 9, 25, 49, 121], primes_squared.values.values);
    }
}
