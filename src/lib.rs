#[cfg(test)]
mod tests {

    use super::*;

    fn hfill_check(sum: u32, len: u32, expected: Vec<Vec<u32>>) {
        assert_eq!(Hfill::new(sum, len).collect::<Vec<Vec<u32>>>(), expected);
    }

    #[test]
    fn hfill0() {
        hfill_check(0, 0, vec![]);
        hfill_check(0, 1, vec![]);
        hfill_check(1, 0, vec![]);
    }

    #[test]
    fn hfill1() {
        hfill_check(1, 1, vec![vec![1]]);
    }

    #[test]
    fn hfill2() {
        hfill_check(
            5,
            3,
            vec![
                vec![3, 1, 1],
                vec![2, 2, 1],
                vec![2, 1, 2],
                vec![1, 3, 1],
                vec![1, 2, 2],
                vec![1, 1, 3],
            ],
        );
    }
}

/// An iterator that produces all combinations
/// of integer lists of a given size,
/// with a given sum of elements,
/// and where elements are no lesser than one.
struct Hfill {
    sum: u32,
    len: u32,
    head: u32,
    tails: Option<Box<Hfill>>,
}

impl Hfill {
    /// Constructs an iterator that produces all combinations
    /// of integer lists of fixed length 'len',
    /// whose sum of elements is 'sum'
    /// and each element is at least 1.
    fn new(sum: u32, len: u32) -> Hfill {
        Hfill {
            sum,
            len,
            head: sum - (len - 1),
            tails: None,
        }
    }
}

impl Iterator for Hfill {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            None
        } else if self.sum < self.len {
            None
        } else if self.len == 1 {
            self.len = 0;
            Some(vec![self.sum])
        } else if self.tails.is_none() {
            self.tails = Some(Box::new(Hfill::new(self.sum - self.head, self.len - 1)));
            self.next()
        } else if let Some(mut tail) = self.tails.as_mut().unwrap().next() {
            tail.insert(0, self.head);
            Some(tail)
        } else if self.head > 1 {
            self.head -= 1;
            self.tails = None;
            self.next()
        } else {
            None
        }
    }
}
