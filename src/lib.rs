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

    #[test]
    #[should_panic(expected = "assertion failed: len >= 2")]
    fn xfill0() {
        Xfill::new(5,1).for_each(drop);
    }

    #[test]
    #[should_panic(expected = "assertion failed: sum >= len - 2")]
    fn xfill1() {
        Xfill::new(1,4).for_each(drop);
    }

    #[test]
    fn xfill2() {
        assert_eq!(
            Xfill::new(5, 3).collect::<Vec<Vec<u32>>>(),
            vec![
                vec![0, 5, 0],
                vec![0, 4, 1],
                vec![0, 3, 2],
                vec![0, 2, 3],
                vec![0, 1, 4],
                vec![1, 4, 0],
                vec![1, 3, 1],
                vec![1, 2, 2],
                vec![1, 1, 3],
                vec![2, 3, 0],
                vec![2, 2, 1],
                vec![2, 1, 2],
                vec![3, 2, 0],
                vec![3, 1, 1],
                vec![4, 1, 0],
            ]
        );
    }
}

/// An iterator that simply counts sequentially.
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

/// An iterator for producing special combinations of integer lists.
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
    pub fn new(sum: u32, len: u32) -> Hfill {
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

/// An iterator for producing special combinations of integer lists.
pub struct Xfill {
    sum: u32,
    len: u32,
    heads: Seq,
    head: u32,
    lasts: Option<Seq>,
    last: u32,
    middle: Option<Hfill>,
}

impl Xfill {
    /// Constructs an iterator that produces all combinations of integer lists where the number of
    /// elements is 'len', the sum of elements is 'sum', the first and last elements are equal or
    /// greater than 0, and the remaining elements are equal or greater than 1.
    pub fn new(sum: u32, len: u32) -> Xfill {
        assert!(len >= 2);
        assert!(sum >= len - 2);
        let heads = if len == 2 {
            Seq::new(0, sum)
        } else {
            Seq::new(0, sum - (len - 2))
        };
        Xfill {
            sum,
            len,
            heads,
            head: 0,
            lasts: None,
            last: 0,
            middle: None,
        }
    }
}

impl Iterator for Xfill {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == 2 {
            match self.heads.next() {
                Some(head) => Some(vec![head, self.sum.checked_sub(head).unwrap()]),
                None => None,
            }
        } else if self.lasts.is_none() {
            match self.heads.next() {
                Some(head) => {
                    self.head = head;
                    self.lasts = Some(Seq::new(0, self.sum - (self.len - 2) - self.head));
                    self.middle = None;
                    self.next()
                }
                None => None,
            }
        } else if self.middle.is_none() {
            match self.lasts.as_mut().unwrap().next() {
                Some(last) => {
                    self.last = last;
                    self.middle = Some(Hfill::new(self.sum - self.head - self.last, self.len - 2));
                    self.next()
                }
                None => {
                    self.lasts = None;
                    self.next()
                }
            }
        } else {
            match self.middle.as_mut().unwrap().next() {
                Some(mut middle) => {
                    middle.insert(0, self.head);
                    middle.push(self.last);
                    Some(middle)
                }
                None => {
                    self.middle = None;
                    self.next()
                }
            }
        }
    }
}
