pub trait ToSpiral<F, T: Iterator<Item = F>, I: Iterator<Item = T>> {
    fn spiral(self) -> Spiral<F, T, I>;
}

pub struct Spiral<F, T: Iterator<Item = F>, I: Iterator<Item = T>> {
    iter: I,
    subiter: Vec<T>,
    index: usize,
    fin: bool,
}

impl<F, T: Iterator<Item = F>, I: Iterator<Item = T>> ToSpiral<F, T, I> for I {
    fn spiral(self) -> Spiral<F, T, I> {
        Spiral {
            iter: self,
            subiter: vec![],
            index: 0,
            fin: false,
        }
    }
}

impl<F, T: Iterator<Item = F>, I: Iterator<Item = T>> Spiral<F, T, I> {
    fn step(&mut self) {
        self.index += 1;
        if self.index >= self.subiter.len() {
            self.index = 0;
        }
    }
}
impl<F, T: Iterator<Item = F>, I: Iterator<Item = T>> Iterator for Spiral<F, T, I> {
    type Item = F;

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(iter), false) = (self.iter.next(), self.fin) {
            self.subiter.push(iter);
            self.index = self.subiter.len() - 1;
        }
        if self.subiter.is_empty() {
            return None;
        }
        let i = self.index;
        loop {
            if let Some(val) = self.subiter[self.index].next() {
                self.step();
                return Some(val);
            }
            self.step();
            if self.index == i {
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spiral_test() {
        assert_eq!(
            "abc\ndef\nghi"
                .lines()
                .map(|s| s.chars())
                .spiral()
                .collect::<String>(),
            "adgbehcfi".to_owned()
        );
    }
}
