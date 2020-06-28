#[cfg(test)]
mod tests {

    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn seq0() {
        assert_eq!(Seq::new(0, 1).collect::<Vec<u32>>(), vec![0, 1]);
    }

    #[test]
    fn seq1() {
        assert_eq!(Seq::new(1, 0).collect::<Vec<u32>>(), vec![]);
    }

    #[test]
    fn seq2() {
        assert_eq!(
            Seq::new(u32::MAX, u32::MAX).collect::<Vec<u32>>(),
            vec![u32::MAX]
        );
    }

    #[test]
    fn seq3() {
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
    #[should_panic(expected = "assertion failed: len >= 2")]
    fn xfill0() {
        Xfill::new(5, 1).for_each(drop);
    }

    #[test]
    #[should_panic(expected = "assertion failed: sum >= len - 2")]
    fn xfill1() {
        Xfill::new(1, 4).for_each(drop);
    }

    #[test]
    fn xfill2() {
        assert_eq!(
            Xfill::new(2, 2).collect::<Vec<Vec<u32>>>(),
            vec![vec![0, 2], vec![1, 1], vec![2, 0]]
        );
    }

    #[test]
    fn blend0() {
        assert_eq!(
            Blend::new(Box::new([1, 2, 3].iter()), Box::new([4].iter())).cmp([1, 4, 2].iter()),
            Ordering::Equal
        );
    }
}

/// An iterator that simply counts sequentially.
pub struct Seq {
    counter: u32,
    remaining: u32,
}

impl Seq {
    fn new(from: u32, until: u32) -> Seq {
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

/// Count sequentially.
///
/// Creates an iterator that counts from a given value up to another given value.
///
/// # Example
/// ```
/// assert_eq!(picross::seq(2,5).collect::<Vec<u32>>(), vec![2,3,4,5]);
/// ```
pub fn seq(from: u32, until: u32) -> impl Iterator<Item = u32> {
    Seq::new(from, until)
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

/// Special combinations of integer lists.
///
/// Creates an iterator that produces all combinations
/// of integer lists of fixed length `len`,
/// whose sum of elements is `sum`
/// and each element is at least 1.
///
/// # Example
/// ```
/// assert_eq!(
///     picross::hfill(5,3).collect::<Vec<Vec<u32>>>(),
///     vec![
///         vec![1, 1, 3],
///         vec![1, 2, 2],
///         vec![1, 3, 1],
///         vec![2, 1, 2],
///         vec![2, 2, 1],
///         vec![3, 1, 1],
///     ]);
/// ```
pub fn hfill(sum: u32, len: u32) -> impl Iterator<Item = Vec<u32>> {
    Hfill::new(sum, len)
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
    fn new(sum: u32, len: u32) -> Xfill {
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

/// Special combinations of integer lists.
///
/// Creates an iterator that produces all combinations of integer lists where the number of
/// elements is `len`, the sum of elements is `sum`, the first and last elements are equal or
/// greater than 0, and the remaining elements are equal or greater than 1.
///
/// # Example
/// ```
/// assert_eq!(
///     picross::xfill(5,3).collect::<Vec<Vec<u32>>>(),
///     vec![
///         vec![0, 5, 0],
///         vec![0, 4, 1],
///         vec![0, 3, 2],
///         vec![0, 2, 3],
///         vec![0, 1, 4],
///         vec![1, 4, 0],
///         vec![1, 3, 1],
///         vec![1, 2, 2],
///         vec![1, 1, 3],
///         vec![2, 3, 0],
///         vec![2, 2, 1],
///         vec![2, 1, 2],
///         vec![3, 2, 0],
///         vec![3, 1, 1],
///         vec![4, 1, 0],
///     ]);
/// ```
pub fn xfill(sum: u32, len: u32) -> impl Iterator<Item = Vec<u32>> {
    Xfill::new(sum, len)
}

/// An iterator that blends two others.
pub struct Blend<'a, T> {
    use_first: bool,
    iter1: Box<dyn Iterator<Item = T> + 'a>,
    iter2: Box<dyn Iterator<Item = T> + 'a>,
}

impl<'a, T> Blend<'a, T> {
    fn new(
        iter1: impl Iterator<Item = T> + 'a,
        iter2: impl Iterator<Item = T> + 'a,
    ) -> Blend<'a, T> {
        Blend {
            use_first: true,
            iter1: Box::new(iter1),
            iter2: Box::new(iter2),
        }
    }
}

impl<'a, T> Iterator for Blend<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match if self.use_first {
            self.iter1.next()
        } else {
            self.iter2.next()
        } {
            Some(answer) => {
                self.use_first = !self.use_first;
                Some(answer)
            }
            None => None,
        }
    }
}

/// Blend two iterators.
///
/// Creates an iterator that consumes two others in order to produce outputs,
/// alternating between each one until one of them exhausts.
///
/// # Example
/// ```
/// let iter1 = vec![1,2].into_iter();
/// let iter2 = vec![3,4].into_iter();
/// let blended = picross::blend(iter1, iter2);
/// assert_eq!(blended.collect::<Vec<i32>>(), vec![1,3,2,4]);
// ```
pub fn blend<'a, T: 'a>(
    iter1: impl Iterator<Item = T> + 'a,
    iter2: impl Iterator<Item = T> + 'a,
) -> impl Iterator<Item = T> + 'a {
    Blend::new(iter1, iter2)
}

/// All possible combinations of a single picross line or column.
///
/// Creates an iterator that produces all gap/fill combinations
/// for a given list `fills` of how many sequential positions
/// are filled in a single line or column
/// with length `len` in a picross puzzle.
///
/// Each produced element is a list
/// where the first element tells how many sequential positions are not filled,
/// followed by how many sequential positions are filled,
/// and so on.
///
/// # Example
/// ```
/// assert_eq!(
///     picross::fill_combine(vec![2,3], 10).collect::<Vec<Vec<u32>>>(),
///     vec![
///         vec![0,2,5,3,0],
///         vec![0,2,4,3,1],
///         vec![0,2,3,3,2],
///         vec![0,2,2,3,3],
///         vec![0,2,1,3,4],
///         vec![1,2,4,3,0],
///         vec![1,2,3,3,1],
///         vec![1,2,2,3,2],
///         vec![1,2,1,3,3],
///         vec![2,2,3,3,0],
///         vec![2,2,2,3,1],
///         vec![2,2,1,3,2],
///         vec![3,2,2,3,0],
///         vec![3,2,1,3,1],
///         vec![4,2,1,3,0],
///     ]
/// );
pub fn fill_combine(fills: Vec<u32>, len: u32) -> impl Iterator<Item = Vec<u32>> {
    xfill(
        len.checked_sub(fills.iter().sum()).unwrap(),
        fills.len() as u32 + 1,
    )
    .map(move |x| blend(x.into_iter(), fills.clone().into_iter()).collect::<Vec<u32>>())
}
