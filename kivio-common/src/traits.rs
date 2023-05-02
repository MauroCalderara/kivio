// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

mod handle;
pub use handle::{Handle, HandleMut};

mod segment;
pub use segment::{FdSegment, FdSegmentMut, Segment, SegmentMut, VmemSegment, VmemSegmentMut};
