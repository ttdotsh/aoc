pub struct MaybeChain<A, B> {
    a: Option<A>,
    b: Option<B>,
}

impl<A, B> Iterator for MaybeChain<A, B>
where
    A: Iterator,
    B: Iterator<Item = A::Item>,
{
    type Item = A::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.a {
            Some(it) => it.next().or_else(|| {
                self.a = None;
                self.next()
            }),
            None => self.b.as_mut()?.next(),
        }
    }
}

pub trait ChainOption: Iterator + Sized {
    fn maybe_chain<I>(self, other: Option<I>) -> MaybeChain<Self, I>
    where
        I: Iterator<Item = Self::Item>,
    {
        MaybeChain {
            a: Some(self),
            b: other,
        }
    }
}

impl<I: Iterator> ChainOption for I {}

#[cfg(test)]
mod test {
    use std::vec::IntoIter;

    use super::*;

    #[test]
    fn test_maybe_chain_with_some() {
        let nums = vec![1, 2];
        let more_nums = vec![3, 4];

        let mut all_nums = nums.into_iter().maybe_chain(Some(more_nums.into_iter()));

        assert_eq!(all_nums.next(), Some(1));
        assert_eq!(all_nums.next(), Some(2));
        assert_eq!(all_nums.next(), Some(3));
        assert_eq!(all_nums.next(), Some(4));
        assert_eq!(all_nums.next(), None);
    }

    #[test]
    fn test_maybe_chain_with_none() {
        let nums = vec![1, 2];

        let mut all_nums = nums.into_iter().maybe_chain::<IntoIter<i32>>(None);

        assert_eq!(all_nums.next(), Some(1));
        assert_eq!(all_nums.next(), Some(2));
        assert_eq!(all_nums.next(), None);
    }

    #[test]
    fn test_maybe_chain_with_some_then_none() {
        let nums = vec![1, 2];
        let more_nums = vec![3, 4];

        let mut all_nums = nums
            .into_iter()
            .maybe_chain(Some(more_nums.into_iter()))
            .maybe_chain::<MaybeChain<IntoIter<i32>, IntoIter<i32>>>(None);

        assert_eq!(all_nums.next(), Some(1));
        assert_eq!(all_nums.next(), Some(2));
        assert_eq!(all_nums.next(), Some(3));
        assert_eq!(all_nums.next(), Some(4));
        assert_eq!(all_nums.next(), None);
    }

    #[test]
    fn test_maybe_chain_with_none_then_some() {
        let nums = vec![1, 2];
        let more_nums = vec![3, 4];

        let mut all_nums = nums
            .into_iter()
            .maybe_chain::<IntoIter<i32>>(None)
            .maybe_chain(Some(more_nums.into_iter()));

        assert_eq!(all_nums.next(), Some(1));
        assert_eq!(all_nums.next(), Some(2));
        assert_eq!(all_nums.next(), Some(3));
        assert_eq!(all_nums.next(), Some(4));
        assert_eq!(all_nums.next(), None);
    }
}
