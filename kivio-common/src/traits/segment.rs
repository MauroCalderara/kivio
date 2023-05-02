// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use std::os::unix::io::RawFd;
use std::vec::Vec;

use crate::err::Error;
use crate::io_vec::IoVec;

use super::handle;

pub trait Segment {
    type Handle: handle::Handle;

    fn from_handle(handle: Self::Handle) -> Self
    where
        Self: Sized;

    fn try_split(self, io_vec: &IoVec) -> Result<Vec<Self>, (Error, Self)>
    where
        Self: Sized;
}

pub trait SegmentMut {
    type HandleMut: handle::HandleMut;

    fn from_handle_mut(handle_mut: Self::HandleMut) -> Self
    where
        Self: Sized;

    fn try_from_vec_segment_mut(vec_segment_mut: Vec<Self>) -> Result<Self, (Error, Vec<Self>)>
    where
        Self: Sized;

    fn try_split(self, io_vec: &IoVec) -> Result<Vec<Self>, (Error, Self)>
    where
        Self: Sized;
}

pub trait VmemSegment: Segment {
    fn ptr_len(&self) -> (*const u8, usize);
}

pub trait VmemSegmentMut: SegmentMut {
    fn mut_ptr_len(&self) -> (*mut u8, usize);
}

pub trait FdSegment: Segment {
    fn fd_offset_len(&self) -> (RawFd, usize, usize);
}

pub trait FdSegmentMut: SegmentMut {
    fn fd_offset_len(&self) -> (RawFd, usize, usize);
}

