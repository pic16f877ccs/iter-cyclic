#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    rustdoc::broken_intra_doc_links
)]

use num::{CheckedAdd, CheckedSub, One, Zero};
use num_convert::{TryFromByAdd, TryToByAdd};
use std::cmp::PartialOrd;
use std::fmt::{Debug, Display};
use std::ops::{AddAssign, Range};
use std::iter::Map;

/// An iterator that sequentially outputs a value in a range skipping n elements.
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
    #[inline]
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

    #[inline]
    fn next(&mut self) -> Option<T> {
        if self.flag {
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
        Some(self.start)
    }
}
/// Creates a new iterator that sequentially outputs a value in the range
/// with a skip of n elements.
///
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
#[inline]
pub fn range_skip<T>(range: Range<T>, skip: usize) -> RangeSkip<T>
where
    T: PartialOrd + Copy + Display + CheckedAdd + CheckedSub + One + Zero,
    usize: TryInto<T>,
    <usize as TryInto<T>>::Error: Debug,
{
    RangeSkip::new(range, skip)
}

/// An iterator that sequentially outputs a value in a range in increments of n elements.
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

    #[inline]
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

/// Creates a new iterator that sequentially outputs a value in the range
/// with a step of n elements.
///
/// Range,
///  start - the lower bound of the range (inclusive).
///  end - the upper bound of the range (inclusive).
/// Step,
///  step of n elements.
///
///```
/// use iter_cyclic::range_step;
///
/// let vec: Vec<u8> = range_step(0, 5, 20).take(12).collect();
/// assert_eq!(vec, [0, 1, 2, 3, 4, 5, 20, 21, 22, 23, 24, 25]);
///
///```
#[inline]
pub fn range_step<T>(start: T, stop: T, step: usize) -> RangeStep<T>
where
    T: Clone + Copy + Debug + TryToByAdd + TryFromByAdd,
{
    let start_usize = start.try_into_usize().unwrap();
    let stop_usize = stop.try_into_usize().unwrap();
    let _ = <T as TryFromByAdd>::try_from_usize(step - 1).unwrap();

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

/// An iterator that sequentially outputs a value in a range in increments of n elements of type usize.
#[derive(Clone, Copy, Debug)]
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
            if let Some(next_step) = self.step_next.checked_add(self.step) {
                self.step_next = next_step;
                if self.step_next > self.end {
                    return None;
                }
            } else {
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

/// Creates a new iterator that sequentially outputs a value in the range with a step of n elements of type usize.
///
/// Range,
///  start - the lower bound of the range (inclusive).
///  end - the upper bound of the range (inclusive).
/// Step,
///  step of n elements.
/// End,
///  iterator length.
///
///```
/// use iter_cyclic::range_step_idx;
///
/// let mut vec: Vec<u8> = (0..=21).collect();
/// range_step_idx(0, 2, 7, vec.len()).for_each(|idx| { vec[idx] += 10; });
/// assert_eq!(vec, [10, 11, 12, 3, 4, 5, 6, 17, 18, 19, 10, 11, 12, 13, 24, 25, 26, 17, 18, 19, 20, 21]);
///
///```
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

pub trait RangeStepVec<T> {
    fn range_step_val(&mut self, start: usize, stop: usize, step: usize, val: T);
    fn range_step_combine(&mut self, start: usize, stop: usize, step: usize, iter: impl Iterator<Item = T>);
    fn range_step_vec(&self, start: usize, stop: usize, step: usize) -> Vec<T>;
    fn range_step_iter(&self, start: usize, stop: usize, step: usize) ->  Map<RangeStepIdx, Box<dyn Fn(usize) -> T + '_>>;
}

impl<T> RangeStepVec<T> for Vec<T>
where
    T: Copy,
{
    #[inline]
    fn range_step_val(&mut self, start: usize, stop: usize, step: usize, val: T) {
        range_step_idx(start, stop, step, self.len()).for_each(|idx| {
            self[idx] = val;
        })
    }

    #[inline]
    fn range_step_combine(&mut self, start: usize, stop: usize, step: usize, mut iter: impl Iterator<Item = T>) {
        for idx in range_step_idx(start, stop, step, self.len()) {
            self[idx] = if let Some(val) = iter.next() { val } else { break; };
        }
    }

    #[inline]
    fn range_step_vec(&self, start: usize, stop: usize, step: usize) -> Vec<T> {
        let mut vec: Vec<T> = Vec::new();
        for idx in range_step_idx(start, stop, step, self.len()) {
            vec.push(self[idx]);
            //vec.push(unsafe { *self.get_unchecked(idx) });
        }
        vec
    }

    #[inline]
    fn range_step_iter(&self, start: usize, stop: usize, step: usize) ->  Map<RangeStepIdx, Box<dyn Fn(usize) -> T + '_>> {
        range_step_idx(start, stop, step, self.len()).map(Box::new(|idx| self[idx]))
    }
}

#[derive(Clone, Debug)]
pub struct RangeStepVecIter<T: Copy> {
    vec: Vec<T>,
    start: usize,
    start_next: usize,
    stop: usize,
    step: usize,
    step_next: usize,
    end: usize,
    once_flag: bool,
}

impl<T: Copy> Iterator for RangeStepVecIter<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.once_flag {
            if self.step == 0 {
                return None;
            }
            self.once_flag = false;
            return Some(self.vec[self.start]);
        }

        if self.start == self.stop {
            if let Some(next_step) = self.step_next.checked_add(self.step) {
                self.step_next = next_step;
                if self.step_next > self.end {
                    return None;
                }
            } else {
                return None;
            }

            self.start_next += self.step;
            self.start = self.start_next;
            self.stop += self.step;
            return Some(self.vec[self.start]);
        }

        self.start += 1;
        Some(self.vec[self.start])
    }
}


pub trait RangeStepIter<T: Copy> {
    fn range_step_iter(self, start: usize, stop: usize, step: usize) -> RangeStepVecIter<T>;
}

impl<T> RangeStepIter<T> for Vec<T>
where
    T: Copy + 'static,
{
    #[inline]
    fn range_step_iter(self, start: usize, stop: usize, step: usize) -> RangeStepVecIter<T> {
        let end = self.len();
        RangeStepVecIter {
            vec: self,
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
}

