// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

//use std::backtrace::Backtrace;
use std::string::String;

use thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Conversion from {from_type} to {to_type} failed: {reason}")]
    ConversionFailed {
        from_type: String,
        to_type: String,
        reason: String,
        //backtrace: Backtrace
    },

    #[error("Byte position ({pos}) is invalid for given outer length ({outer_len})")]
    InvalidBytePos { pos: i64, outer_len: usize },

    #[error("Byte range ({start}:{end}) is invalid")]
    InvalidByteRangeOrdering { start: i64, end: i64 },

    #[error("Byte range ({start}:{end}) is invalid for given outer length ({outer_len})")]
    InvalidByteRange {
        start: i64,
        end: i64,
        outer_len: usize,
    },

    #[error(
        "IoVec is invalid for given outer length ({outer_len}), invalid range is (\
             {start}:{end})"
    )]
    InvalidIoVec {
        start: i64,
        end: i64,
        outer_len: usize,
    },

    #[error("IoVec elements overlap, possibly due to given outer length ({outer_len})")]
    OverlappingIoVec { outer_len: usize },

}
