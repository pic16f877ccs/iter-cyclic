use num::{CheckedAdd, CheckedSub, One, Zero};
use num_convert::{TryFromByAdd, TryToByAdd};
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
///  start - the lower bound of the range (inclusive).
///  end - the upper bound of the range (inclusive).
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

#[derive(Copy, Clone, Debug)]
pub struct RangeStep<T> {
    start: T,
    stop: T,
    start_usize: usize,
    stop_usize: usize,
    step: usize,
    step_next: usize,
    flag: bool,
}

impl<T> Iterator for RangeStep<T>
where
    T: Clone
        + Copy
        + Clone
        + Debug
        + TryToByAdd
        + TryFromByAdd
        + One
        + AddAssign
        + CheckedAdd
        + PartialEq,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.flag {
            if self.step == 0 {
                return None;
            }
            self.flag = false;
            return Some(self.start);
        }

        if self.start == self.stop {
            if let Some(step_next) = self.step_next.checked_add(self.step) {
                self.step_next = step_next;
                match <T as TryFromByAdd>::try_from_usize(self.step_next) {
                    Some(_) => {
                        self.start_usize += self.step;
                        self.stop_usize += self.step;
                        self.start = <T as TryFromByAdd>::try_from_usize(self.start_usize).unwrap();
                        self.stop = <T as TryFromByAdd>::try_from_usize(self.stop_usize).unwrap();
                        return Some(self.start);
                    }
                    None => {
                        return None;
                    }
                }
            } else {
                return None;
            }
        }
        self.start += T::one();
        Some(self.start)
    }
}

pub fn range_step<T>(start: T, stop: T, step: usize) -> RangeStep<T>
where
    T: Clone + Copy + Debug + TryToByAdd + TryFromByAdd,
{
    let start_usize = start.try_into_usize().unwrap();
    let stop_usize = stop.try_into_usize().unwrap();
    let _ = <T as TryFromByAdd>::try_from_usize(step -1 ).unwrap();

    RangeStep {
        start,
        stop,
        start_usize,
        stop_usize,
        step: if start_usize > stop_usize || stop_usize >= step {
            0
        } else {
            step
        },
        step_next: step,
        flag: true,
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RangeStepIdx {
    start: usize,
    start_next: usize,
    stop: usize,
    step: usize,
    step_next: usize,
    end: usize,
    once_flag: bool,
}

impl Iterator for RangeStepIdx {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.once_flag {
            if self.step == 0 {
                return None;
            }
            self.once_flag = false;
            return Some(self.start);
        }

        if self.start == self.stop {
            self.step_next += self.step;
            if self.step_next > self.end {
                return None;
            }
            self.start_next += self.step;
            self.start = self.start_next;
            self.stop += self.step;
            return Some(self.start);
        }

        self.start += 1;
        Some(self.start)
    }
}

#[inline]
pub fn range_step_idx(start: usize, stop: usize, step: usize, end: usize) -> RangeStepIdx {
    RangeStepIdx {
        start,
        start_next: start,
        stop,
        step: if start > stop || stop >= step || step > end {
            0
        } else {
            step
        },
        step_next: step,
        end,
        once_flag: true,
    }
}
