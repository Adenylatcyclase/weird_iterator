use num::PrimInt;

pub struct Fib<T: PrimInt> {
    n1: T,
    n2: T,
    count: usize,
}

impl<T: PrimInt> Fib<T> {
    pub fn new() -> Fib<T> {
        Fib {
            n1: T::zero(),
            n2: T::one(),
            count: 0,
        }
    }

    pub fn start_at(mut self, n1: T, n2: T) -> Self {
        self.n1 = n1;
        self.n2 = n2;
        self
    }
}
impl<T: PrimInt> Default for Fib<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PrimInt> Iterator for Fib<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.count;
        self.count += 1;
        match c {
            0 => Some(self.n1),
            1 => Some(self.n2),
            _ => {
                if let Some(t) = self.n1.checked_add(&self.n2) {
                    self.n1 = self.n2;
                    self.n2 = t;
                    Some(t)
                } else {
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_test1() {
        let mut f: Fib<i32> = Fib::new();
        assert_eq!(f.n1, 0);
        assert_eq!(f.n2, 1);
        assert_eq!(f.next(), Some(0));
        assert_eq!(f.next(), Some(1));
        assert_eq!(f.next(), Some(1));
        assert_eq!(f.next(), Some(2));
        assert_eq!(f.next(), Some(3));
        let f2: Fib<u8> = Fib::new();
        assert_eq!(
            f2.collect::<Vec<u8>>(),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233]
        );

        assert_eq!(
            Fib::new().start_at(2, 5).collect::<Vec<u8>>(),
            vec![2, 5, 7, 12, 19, 31, 50, 81, 131, 212]
        )
    }
}
