// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use std::cmp::{Ord, Ordering, PartialOrd};

use crate::err::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct BytePos(pub i64);

impl BytePos {
    pub(crate) fn is_absolute(&self) -> bool {
        !self.0 < 0
    }

    pub(crate) fn is_relative(&self) -> bool {
        !self.is_absolute()
    }

    pub(crate) fn to_absolute(&self, outer_len: usize) -> Result<BytePos, Error> {
        if self.is_absolute() {
            if (self.0 as usize) > outer_len {
                return Err(Error::InvalidBytePos {
                    pos: self.0,
                    outer_len,
                });
            } else {
                return Ok(Self(self.0));
            }
        } else if outer_len - ((-self.0) as usize) < (i64::MAX as usize) {
            let neg_pos = (-self.0) as usize;
            if neg_pos > outer_len {
                return Err(Error::InvalidBytePos {
                    pos: self.0,
                    outer_len,
                });
            } else {
                return Ok(Self((outer_len - neg_pos) as i64));
            }
        }
        panic!(
            "Unable to represent relative byte position ({}) for outer length ({}) using a \
             struct BytePos(i64). Outer length is larger than i64::MAX ({})",
            self.0, outer_len, i64::MAX
        )
    }

    fn to_usize_max_outer_len(&self) -> usize {
        if self.is_absolute() {
            self.0 as usize
        } else {
            usize::MAX - ((-self.0) as usize)
        }
    }

}

impl PartialOrd for BytePos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_usize_max_outer_len()
            .partial_cmp(&other.to_usize_max_outer_len())
    }
}

impl Ord for BytePos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.to_usize_max_outer_len()
            .cmp(&other.to_usize_max_outer_len())
    }
}
