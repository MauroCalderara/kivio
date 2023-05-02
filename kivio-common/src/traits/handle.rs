// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use crate::err::Error;

use super::segment;

pub trait Handle {
    type HandleMut: HandleMut;
    type Segment: segment::Segment;

    fn from_handle_mut(handle_mut: Self::HandleMut) -> Self;

    fn from_segment(segment: Self::Segment) -> Self;
}

pub trait HandleMut {
    type Handle: Handle;
    type SegmentMut: segment::SegmentMut;

    fn try_from_handle(handle: Self::Handle) -> Result<Self, (Error, Self::Handle)>
    where
        Self: Sized;

    fn try_from_segment_mut(
        segment_mut: Self::SegmentMut,
    ) -> Result<Self, (Error, Self::SegmentMut)>
    where
        Self: Sized;
}
