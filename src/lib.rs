#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn seq0() {
        assert_eq!(Seq::new(0, 1).collect::<Vec<u32>>(), vec![0, 1]);
    }

    #[test]
    fn seq1() {
        assert_eq!(Seq::new(3, 8).collect::<Vec<u32>>(), vec![3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn seq2() {
        assert_eq!(Seq::new(1, 0).collect::<Vec<u32>>(), vec![]);
    }

    #[test]
    fn seq3() {
        assert_eq!(
            Seq::new(u32::MAX, u32::MAX).collect::<Vec<u32>>(),
            vec![u32::MAX]
        );
    }

    #[test]
    fn seq4() {
        assert_eq!(
            Seq {
                counter: u32::MAX,
                remaining: 2
            }
            .collect::<Vec<u32>>(),
            vec![u32::MAX, u32::MAX]
        );
    }

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
                vec![1, 1, 3],
                vec![1, 2, 2],
                vec![1, 3, 1],
                vec![2, 1, 2],
                vec![2, 2, 1],
                vec![3, 1, 1],
            ],
        );
    }
}

/// An iterator that simply counts.
pub struct Seq {
    counter: u32,
    remaining: u32,
}

impl Seq {
    /// Constructs an iterator that counts from a given value up to another given value.
    pub fn new(from: u32, until: u32) -> Seq {
        if until >= from {
            Seq {
                counter: from,
                remaining: (until - from).checked_add(1).unwrap(),
            }
        } else {
            Seq {
                counter: from,
                remaining: 0,
            }
        }
    }
}

impl Iterator for Seq {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            None
        } else {
            let answer = self.counter;
            self.counter = self.counter.saturating_add(1);
            self.remaining -= 1;
            Some(answer)
        }
    }
}

/// An iterator that produces all combinations
/// of integer lists of a given size,
/// with a given sum of elements,
/// and where elements are no lesser than one.
pub struct Hfill {
    sum: u32,
    len: u32,
    heads: Seq,
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
            heads: Seq::new(1, sum.saturating_sub(len.saturating_sub(1))),
            head: 0,
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
            match self.heads.next() {
                Some(head) => {
                    self.head = head;
                    self.tails = Some(Box::new(Hfill::new(self.sum - self.head, self.len - 1)));
                    self.next()
                }
                None => None,
            }
        } else if let Some(mut tail) = self.tails.as_mut().unwrap().next() {
            tail.insert(0, self.head);
            Some(tail)
        } else {
            self.tails = None;
            self.next()
        }
    }
}
