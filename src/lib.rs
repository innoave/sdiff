#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
mod std {
    extern crate alloc;
    pub use alloc::*;
    pub use core::*;
}

#[cfg(feature = "std")]
mod std {
    pub use std::*;
}

use crate::std::{
    boxed::Box,
    ops::{Index, IndexMut},
    vec,
    vec::Vec,
};

#[must_use]
pub fn diff_str(left: &str, right: &str) -> Vec<Diff> {
    diff(
        &left.chars().collect::<Vec<_>>(),
        &right.chars().collect::<Vec<_>>(),
    )
}

#[must_use]
pub fn diff<T>(left: &[T], right: &[T]) -> Vec<Diff>
where
    T: PartialEq,
{
    let trace = find_shortest_trace(left, right);
    list_diffs(left, right, &trace)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Diff {
    Left {
        index: usize,
        length: usize,
    },
    Both {
        left_index: usize,
        right_index: usize,
        length: usize,
    },
    Right {
        index: usize,
        length: usize,
    },
}

/// The shortest trace found in the edit space.
pub struct ShortestTrace {
    data: Box<[isize]>,
    len: isize,
}

impl ShortestTrace {
    #[must_use]
    #[allow(clippy::cast_sign_loss, clippy::len_without_is_empty)]
    pub const fn len(&self) -> usize {
        self.len as usize
    }

    #[must_use]
    pub fn get(&self, d: isize, k: isize) -> &isize {
        let idx = Trace::calculate_index(d, k);
        &self.data[idx]
    }

    #[must_use]
    pub fn get_mut(&mut self, d: isize, k: isize) -> &mut isize {
        let idx = Trace::calculate_index(d, k);
        &mut self.data[idx]
    }
}

impl Index<(isize, isize)> for ShortestTrace {
    type Output = isize;

    fn index(&self, (d, k): (isize, isize)) -> &Self::Output {
        self.get(d, k)
    }
}

impl IndexMut<(isize, isize)> for ShortestTrace {
    fn index_mut(&mut self, (d, k): (isize, isize)) -> &mut Self::Output {
        self.get_mut(d, k)
    }
}

/// Recorded path through the edit space.
///
/// The index *k* is calculated as *k = x - y*. *d* is the depth in the graph
/// that is examined. The values stored in the matrix are the best *x* value
/// that can be achieved at each point.
///
/// # Layout
///
/// ```text
///     |                k
///     |-5 -4 -3 -2 -1  0  1  2  3  4  5
/// ----+---------------------------------
///   0 |                o
///   1 |             o  o  o
/// d 2 |          o  o  o  o  o
///   3 |       o  o  o  o  o  o  o
///   4 |    o  o  o  o  o  o  o  o  o
///   5 | o  o  o  o  o  o  o  o  o  o  o
/// ```
///
/// # Example
///
/// Trace for diff of sequences 'ABCABBA' and 'CBABAC':
///
/// ```text
///     |                k
///     |-5 -4 -3 -2 -1  0  1  2  3  4  5
/// ----+---------------------------------
///   0 |                0
///   1 |             0  0  1
/// d 2 |          2  0  2  1  3
///   3 |       3  2  4  2  5  3  5
///   4 |       3  4  4  5  5  7  5  7
///   5 |       3  4  5  5  7  7  5  7
/// ```
struct Trace {
    data: Box<[isize]>,
}

impl Trace {
    /// Max length of the sequences that is supported.
    pub const MAX_SEQUENCE_LENGTH: usize = 2 * (isize::MAX.isqrt() as usize - 2);

    /// Constructs a new `Trace` with pre-allocated slots.
    ///
    /// * *d* is iterated from *0* to max depth
    /// * For each value of *d* we need *1 + d* slots
    /// * sum of integers is *n * (n + 1) / 2*
    /// * *k* is iterated from *-d* to *+d* on every other.
    pub fn new(left_len: usize, right_len: usize) -> Self {
        assert!(
            left_len <= Self::MAX_SEQUENCE_LENGTH,
            "the left sequence is longer than the max supported length of {}",
            Self::MAX_SEQUENCE_LENGTH
        );
        assert!(
            right_len <= Self::MAX_SEQUENCE_LENGTH,
            "the right sequence is longer than the max supported length of {}",
            Self::MAX_SEQUENCE_LENGTH
        );

        let max_depth = left_len + right_len;
        let num_slots = (max_depth + 1) * (max_depth + 2) / 2;

        Self {
            data: vec![0; num_slots].into(),
        }
    }

    /// Calculates the index into the internal matrix for *(d, k)*.
    #[inline]
    #[allow(clippy::cast_sign_loss)]
    fn calculate_index(d: isize, k: isize) -> usize {
        assert!(k >= -d && k <= d, "invalid index in matrix {:?}", (d, k));
        let k_offset = d * (d + 1) / 2;
        // *k* goes from *-d* to *d* so we need to map [-d, d] -> [0, 2d]
        let unsigned_k = k + d;
        (unsigned_k / 2 + k_offset) as usize
    }

    #[must_use]
    pub fn get(&self, d: isize, k: isize) -> &isize {
        let idx = Self::calculate_index(d, k);
        &self.data[idx]
    }

    #[must_use]
    pub fn get_mut(&mut self, d: isize, k: isize) -> &mut isize {
        let idx = Self::calculate_index(d, k);
        &mut self.data[idx]
    }
}

impl Index<(isize, isize)> for Trace {
    type Output = isize;

    fn index(&self, (d, k): (isize, isize)) -> &Self::Output {
        self.get(d, k)
    }
}

impl IndexMut<(isize, isize)> for Trace {
    fn index_mut(&mut self, (d, k): (isize, isize)) -> &mut Self::Output {
        self.get_mut(d, k)
    }
}

/// Find the shortest path from *(0,0)* till the end of the edit graph.
#[allow(clippy::cast_possible_wrap, clippy::cast_sign_loss)]
fn find_shortest_trace<T>(left: &[T], right: &[T]) -> ShortestTrace
where
    T: PartialEq,
{
    let left_len = left.len();
    let right_len = right.len();

    let max_depth = left_len + right_len;

    let mut trace = Trace::new(left_len, right_len);

    let max_depth = max_depth as isize;
    let left_len = left_len as isize;
    let right_len = right_len as isize;

    for d in 0..=max_depth {
        for k in (-d..=d).step_by(2) {
            let mut x = if d == 0 {
                0
            } else if k == -d {
                trace[(d - 1, k + 1)]
            } else if k == d {
                trace[(d - 1, k - 1)] + 1
            } else {
                let left = trace[(d - 1, k - 1)];
                let right = trace[(d - 1, k + 1)];
                if left < right {
                    right
                } else {
                    left + 1
                }
            };

            let mut y = x - k;
            assert!(
                y >= 0,
                "y should always be greater than or equal to 0, but is: {y:?}"
            );

            #[allow(clippy::suspicious_operation_groupings)]
            while x < left_len && y < right_len && left[x as usize] == right[y as usize] {
                x += 1;
                y += 1;
            }

            trace[(d, k)] = x;

            if x >= left_len && y >= right_len {
                return ShortestTrace {
                    data: trace.data,
                    len: d,
                };
            }
        }
    }

    panic!("length of a trace is longer than the maximum, which is `left.len() + right.len()`")
}

/// List common subsequences and differences between two sequences by
/// backtracking the given trace.
#[allow(clippy::cast_possible_wrap)]
fn list_diffs<T>(left: &[T], right: &[T], trace: &ShortestTrace) -> Vec<Diff> {
    if left.len() + right.len() == 0 {
        return vec![Diff::Both {
            left_index: 0,
            right_index: 0,
            length: 0,
        }];
    }

    let mut x = left.len() as isize;
    let mut y = right.len() as isize;

    let mut diffs = Vec::new();

    for d in (0..=trace.len).rev() {
        let k = x - y;

        let prev_k = if d == 0 {
            0
        } else if k == -d {
            k + 1
        } else if k == d {
            k - 1
        } else {
            let left = trace[(d - 1, k - 1)];
            let right = trace[(d - 1, k + 1)];
            if left < right {
                k + 1
            } else {
                k - 1
            }
        };

        let prev_x = if d == 0 { 0 } else { trace[(d - 1, prev_k)] };
        let prev_y = prev_x - prev_k;

        while x > prev_x && y > prev_y {
            x -= 1;
            y -= 1;
            if y < 0 {
                y = 0;
            }
            if let Some(Diff::Both {
                left_index,
                right_index,
                length,
            }) = diffs.last_mut()
            {
                *left_index -= 1;
                *right_index -= 1;
                *length += 1;
            } else {
                #[allow(clippy::cast_sign_loss)]
                diffs.push(Diff::Both {
                    left_index: x as usize,
                    right_index: y as usize,
                    length: 1,
                });
            }
        }

        if d > 0 {
            if prev_y == y {
                if let Some(Diff::Left { index, length }) = diffs.last_mut() {
                    *index -= 1;
                    *length += 1;
                } else {
                    #[allow(clippy::cast_sign_loss)]
                    diffs.push(Diff::Left {
                        index: prev_x as usize,
                        length: 1,
                    });
                }
            } else if prev_x == x {
                if let Some(Diff::Right { index, length }) = diffs.last_mut() {
                    *index -= 1;
                    *length += 1;
                } else {
                    #[allow(clippy::cast_sign_loss)]
                    diffs.push(Diff::Right {
                        index: prev_y as usize,
                        length: 1,
                    });
                }
            } else {
                unreachable!("we should not come here!")
            }
        }

        x = prev_x;
        y = prev_y;
    }

    diffs.reverse();
    diffs
}

#[cfg(test)]
mod tests;
