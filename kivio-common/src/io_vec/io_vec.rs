// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use std::ops::Deref;
use std::vec::Vec;

use crate::err::Error;

use super::ByteRange;

#[derive(Debug, Clone)]
pub struct IoVec {
    tallest_range: ByteRange, // The range that contributes the largest implied minimal outer_size

    // Some(False) -> not overlapping (but still need to check min_outer_len)
    // Some(True) -> overlapping (but still need to check min_outer_len)
    // None -> must compute with outer_len
    cached_overlapping: Option<bool>,

    byte_ranges: Vec<ByteRange>,
}

impl IoVec {
    pub fn from_chunk_size(total_len: usize, chunk_size: usize) -> Self {
        if chunk_size == 0 {
            panic!("Chunk size of zero (0) specified");
        }

        let n_full_chunks = total_len / chunk_size;
        let last_size = total_len % chunk_size;
        let n_chunks = if last_size > 0 {
            n_full_chunks + 1
        } else {
            n_full_chunks
        };

        let mut byte_ranges = Vec::<ByteRange>::with_capacity(n_chunks);

        for i in 0..n_full_chunks {
            byte_ranges.push(ByteRange::new_usize(i * chunk_size, (i + 1) * chunk_size).unwrap())
        }
        if last_size > 0 {
            byte_ranges.push(
                ByteRange::new_usize(
                    n_full_chunks * chunk_size,
                    n_full_chunks * chunk_size + last_size,
                )
                .unwrap(),
            )
        }

        Self {
            tallest_range: byte_ranges[n_chunks - 1],
            cached_overlapping: Some(false),
            byte_ranges,
        }
    }

    pub fn from_vec_byte_range(byte_ranges: Vec<ByteRange>) -> Self {
        Self {
            tallest_range: helper::get_tallest_range(&byte_ranges),
            cached_overlapping: helper::get_cached_overlapping(&byte_ranges),
            byte_ranges,
        }
    }

    pub fn min_outer_len(&self) -> usize {
        self.tallest_range.min_outer_len()
    }

    pub fn is_overlapping(&self, outer_len: usize) -> Result<bool, Error> {
        if outer_len < self.min_outer_len() {
            return Err(Error::InvalidIoVec {
                start: self.tallest_range.start.0,
                end: self.tallest_range.end.0,
                outer_len,
            });
        }
        match self.cached_overlapping {
            Some(o) => Ok(o),
            None => Ok(helper::get_overlapping(&self.byte_ranges, outer_len)?),
        }
    }
}

impl Deref for IoVec {
    type Target = Vec<ByteRange>;
    fn deref(&self) -> &Self::Target {
        &self.byte_ranges
    }
}

pub(super) mod helper {

    use std::ops::Deref;

    use crate::err::Error;

    use super::ByteRange;

    enum SortedVecByteRange<'a> {
        IsAlreadySorted(&'a Vec<ByteRange>),
        SortedCopy(Vec<ByteRange>),
    }

    impl<'a> SortedVecByteRange<'a> {
        fn new(v: &'a Vec<ByteRange>) -> Self {
            // If we were on nightly we could use .is_sorted()
            if v.windows(2).all(|w| w[0] <= w[1]) {
                Self::IsAlreadySorted(v)
            } else {
                let mut new_v = v.clone();
                new_v.sort_unstable();
                Self::SortedCopy(new_v)
            }
        }
    }

    impl<'a> Deref for SortedVecByteRange<'a> {
        type Target = Vec<ByteRange>;
        fn deref(&self) -> &Self::Target {
            match self {
                Self::IsAlreadySorted(ref_v) => ref_v,
                Self::SortedCopy(ref ref_inner) => ref_inner,
            }
        }
    }

    pub(crate) fn get_tallest_range(byte_ranges: &Vec<ByteRange>) -> ByteRange {
        match byte_ranges
            .iter()
            .max_by(|a, b| a.min_outer_len().cmp(&b.min_outer_len()))
        {
            Some(br) => br.clone(),
            None => ByteRange::new_i64(0, 0).unwrap(),
        }
    }

    fn subrange_has_overlap(ranges: &[ByteRange]) -> bool {
        for w in ranges.windows(2) {
            assert!(
                w[0].start.is_absolute() == w[0].end.is_absolute()
                    && w[0].start.is_absolute() == w[1].start.is_absolute()
                    && w[0].start.is_absolute() == w[1].end.is_absolute(),
                "not all ranges are absolute or not all ranges are relative"
            );
            if w[0].end > w[1].start {
                return true;
            }
        }
        return false;
    }

    pub(crate) fn get_cached_overlapping(byte_ranges: &Vec<ByteRange>) -> Option<bool> {
        // Return values:
        //  Ok(Some(true)) -> will overlap or but might be invalid given outer_len
        //  Ok(Some(false)) -> will not overlap but might be invalid given outer_len
        //  Ok(None) -> indeterminate (must check with outer_len set)
        //
        // Conclusive cases are, after sorting, assumed to look like:
        //   [a0,...][m][r0, ...]
        // where
        //   - each subarray [] might be empty
        //   - all a<i> are absolute
        //   - m is mixed of the form (start: absolute, end: relative)
        //   - all r<j> are relative
        //   - all windows of size 2 have no overlap
        // All other cases will have to be checked with outer_len being set.

        if byte_ranges.len() < 2 {
            // An empty or single range will never overlap
            return Some(false);
        }

        let hodl = SortedVecByteRange::new(byte_ranges);
        let s = &hodl as &Vec<ByteRange>;

        // Check that all a<i> are non_overlapping
        let len_a = match s
            .iter()
            .position(|&x| x.start.is_relative() || x.end.is_relative())
        {
            None => s.len(),
            Some(len) => len,
        };
        if subrange_has_overlap(&s[0..len_a]) {
            // At least one of the [a<i>, a<j>] pairs overlap
            return Some(true);
        }
        if len_a == s.len() {
            // No [m] and [r<i>] => if we didn't overlap yet, we never will
            return Some(false);
        }

        // Check m - if it exists
        let mut potential_r_start = len_a;
        if s[len_a].start.is_absolute() && s[len_a].end.is_relative() {
            let pos_m = &len_a;
            if pos_m > &0 {
                if s[pos_m - 1].end > s[*pos_m].start {
                    // m overlaps with a< -1>
                    return Some(true);
                }
            }
            if *pos_m == s.len() {
                // No [r<i>] => if we didn't overlap yet, we never will
                return Some(false);
            }
            potential_r_start += 1;
            let potential_r0 = &s[potential_r_start];
            if potential_r0.start.is_absolute() || potential_r0.end.is_absolute() {
                // potential_r0 is not all relative => can't tell without outer_len
                return None;
            }
            if s[*pos_m].end > potential_r0.start {
                // m overlaps with r0
                return Some(true);
            }
        }

        // Check r - if it exists
        match s
            .iter()
            .skip(potential_r_start)
            .position(|&x| x.start.is_absolute() || x.end.is_absolute())
        {
            // Tail after potential_r_start are all relative and might overlap
            None => return Some(subrange_has_overlap(&s[potential_r_start..])),

            // Tail is not all relative -> can't tell without outer_len
            Some(_l) => return None,
        };
    }

    pub(crate) fn get_overlapping(
        byte_ranges: &Vec<ByteRange>,
        outer_len: usize,
    ) -> Result<bool, Error> {
        let rv: Result<Vec<_>, _> = byte_ranges
            .iter()
            .map(|&br| br.to_absolute(outer_len))
            .collect();
        match rv {
            Err(e) => match e {
                Error::InvalidByteRange {
                    start: e_start,
                    end: e_end,
                    outer_len: _,
                } => Err(Error::InvalidIoVec {
                    start: e_start,
                    end: e_end,
                    outer_len,
                }),
                j => Err(j),
            },
            Ok(mut s) => {
                s.sort_unstable();
                Ok(subrange_has_overlap(&s))
            }
        }
    }
}
