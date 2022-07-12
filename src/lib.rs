use num::{CheckedAdd, CheckedSub, One, Zero};
use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, Range};

#[derive(Debug, Clone)]
pub struct RangeSkip<T> {
    start: T,
    end: T,
    skip: T,
    diff: T,
    flag: bool,
}

impl<T> RangeSkip<T>
where
    T: PartialOrd + Copy + Display + CheckedAdd + CheckedSub + One + Zero,
    usize: TryInto<T>,
    <usize as TryInto<T>>::Error: Debug,
{
    fn new(range: Range<T>, skip: usize) -> Self {
        let Range { start, end } = range;
        if start > end {
            panic!("start {start} > end {end}");
        }
        let skip = skip.try_into().expect("expected conversion to succeed");
        let diff = if skip != T::zero() {
            let diff_tmp = end
                .checked_sub(&start)
                .expect("expected subtraction to succeed")
                .checked_add(&skip)
                .expect("expected addition to succeed")
                .checked_add(&T::one())
                .expect("expected addition to succeed");
            let _ = end
                .checked_add(&diff_tmp)
                .expect("expected addition to succeed");
            diff_tmp
        } else {
            T::zero()
        };
        RangeSkip {
            start,
            end,
            skip,
            diff,
            flag: true,
        }
    }
}

impl<T> Iterator for RangeSkip<T>
where
    T: PartialOrd + Copy + AddAssign + CheckedAdd + One + Zero,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.flag == true {
            self.flag = false;
            return Some(self.start);
        }
        if self.start == self.end {
            if self.skip == Zero::zero() {
                return None;
            }
            if let Some(end) = self.end.checked_add(&self.diff) {
                self.end = end;
                self.start += self.skip + T::one();
                return Some(self.start);
            } else {
                return None;
            };
        }
        self.start += T::one();
        return Some(self.start);
    }
}
///
/// Creates a new iterator that sequentially outputs a value in the range
/// with a skip of n elements.
/// Range,
///   start - the lower bound of the range (inclusive).
///   end - the upper bound of the range (inclusive).
/// Skip,
///  skip of n elements.
///
/// If the start value is greater than the end value, panic.
/// Panic if value skip conversion to output type error.
///
///```
/// use iter_cyclic::range_skip;
///
/// let vec: Vec<u8> = range_skip(0..5, 200).collect();
/// assert_eq!(vec, [0, 1, 2, 3, 4, 5, 206, 207, 208, 209, 210, 211]);
///
///```
pub fn range_skip<T>(range: Range<T>, skip: usize) -> RangeSkip<T>
where
    T: PartialOrd + Copy + Display + CheckedAdd + CheckedSub + One + Zero,
    usize: TryInto<T>,
    <usize as TryInto<T>>::Error: Debug,
{
    RangeSkip::new(range, skip)
}
