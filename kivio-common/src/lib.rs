// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

pub mod traits;
pub use traits::{
    FdSegment, FdSegmentMut, Handle, HandleMut, Segment, SegmentMut, VmemSegment, VmemSegmentMut,
};

pub mod err;
pub mod io_vec;

pub mod vmem;
