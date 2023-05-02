// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use std::cmp::max;

use crate::err::Error;

use super::BytePos;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct ByteRange {
    pub(crate) start: BytePos,
    pub(crate) end: BytePos,
}

impl ByteRange {
    pub fn new(start: BytePos, end: BytePos) -> Result<Self, Error> {
        if start.is_absolute() == end.is_absolute() && start > end {
            Err(Error::InvalidByteRangeOrdering {
                start: start.0,
                end: end.0,
            })
        } else {
            Ok(Self { start, end })
        }
    }

    pub fn new_i64(start: i64, end: i64) -> Result<Self, Error> {
        Self::new(BytePos(start), BytePos(end))
    }

    pub fn new_usize(start: usize, end: usize) -> Result<Self, Error> {
        Self::new(
            BytePos(i64::try_from(start).unwrap()),
            BytePos(i64::try_from(end).unwrap()),
        )
    }

    pub fn is_absolute(&self) -> bool {
        return self.start.is_absolute() && self.end.is_absolute();
    }

    pub fn absolute_start_end(&self, outer_len: usize) -> Result<(BytePos, BytePos), Error> {
        let (abs_start, abs_end) = match (
            self.start.to_absolute(outer_len),
            self.end.to_absolute(outer_len),
        ) {
            (Ok(s), Ok(e)) => (s, e),
            _ => {
                return Err(Error::InvalidByteRange {
                    start: self.start.0,
                    end: self.end.0,
                    outer_len,
                })
            }
        };
        if abs_start < abs_end {
            Ok((abs_start, abs_end))
        } else {
            Err(Error::InvalidByteRange {
                start: self.start.0,
                end: self.end.0,
                outer_len,
            })
        }
    }

    pub fn to_absolute(&self, outer_len: usize) -> Result<Self, Error> {
        let (start, end) = self.absolute_start_end(outer_len)?;
        Ok(Self { start, end })
    }

    pub fn len(&self, outer_len: usize) -> Result<usize, Error> {
        let (abs_start, abs_end) = self.absolute_start_end(outer_len)?;
        Ok((abs_end.0 - abs_start.0) as usize)
    }

    pub fn min_outer_len(&self) -> usize {
        let start_min_outer_len = if self.start.is_absolute() {
            self.start.0 as usize + 1
        } else {
            (-self.start.0) as usize
        };
        let end_min_outer_len = if self.end.is_absolute() {
            self.end.0 as usize
        } else {
            (-self.end.0) as usize
        };
        max(start_min_outer_len, end_min_outer_len)
    }

    pub fn to_offset_len(&self, outer_len: usize) -> Result<(usize, usize), Error> {
        let (abs_start, abs_end) = self.absolute_start_end(outer_len)?;
        Ok((abs_start.0 as usize, (abs_end.0 - abs_start.0) as usize))
    }
}
