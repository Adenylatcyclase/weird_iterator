
trait ToFinite<I: Iterator> {
    fn finite(self, limit: usize) -> FiniteIterator<I>;
}
struct FiniteIterator<I: Iterator> {
    iter: I,
    count: usize,
    limit: usize,
}

impl<T, I: Iterator<Item = T>> Iterator for FiniteIterator<I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.count;
        self.count += 1;
        if c >= self.limit {
            None
        } else {
            self.iter.next()
        }
    }
}

impl<T, I: Iterator<Item = T>> ToFinite<I> for I {
    fn finite(self, limit: usize) -> FiniteIterator<I> {
        FiniteIterator {
            iter: self,
            count: 0,
            limit,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finite_test1() {
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8, 9].into_iter().finite(3).collect::<Vec<u32>>(), vec![1, 2, 3]);
    }
}