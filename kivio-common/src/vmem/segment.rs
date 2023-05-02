// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use std::convert::From;
use std::ops::Deref;
use std::slice;
use std::sync::Arc;
use std::vec::Vec;

use crate::err::Error;
use crate::io_vec::IoVec;
use crate::traits;

use super::{Handle, Vmem};

#[derive(Debug, Clone)]
pub struct Segment {
    pub(super) ptr: *const u8,
    pub(super) len: usize,
    pub(super) vmem: Arc<Vmem>,
}

impl traits::Segment for Segment {
    type Handle = Handle;

    fn from_handle(handle: Self::Handle) -> Self {
        let (ptr, len) = match *handle.vmem {
            Vmem::VecU8(ref v) => (v.as_ptr(), v.len()),
            Vmem::AnonMmap(ref m) => (m.mut_ptr as *const u8, m.len),
        };
        Self {
            ptr,
            len,
            vmem: handle.vmem,
        }
    }

    fn try_split(self, io_vec: &IoVec) -> Result<Vec<Self>, (Error, Self)> {
        let segment_vec: Result<Vec<_>, _> = io_vec
            .iter()
            .map(|&r| {
                let (offset, len) = r.to_offset_len(self.len)?;
                Ok(Self {
                    ptr: unsafe { self.ptr.offset(offset.try_into().unwrap()) },
                    len,
                    vmem: self.vmem.clone(),
                })
            })
            .collect();
        match segment_vec {
            Ok(v) => Ok(v),
            Err(e) => Err((e, self)),
        }
    }
}

impl traits::VmemSegment for Segment {
    fn ptr_len(&self) -> (*const u8, usize) {
        (self.ptr, self.len)
    }
}

impl From<Handle> for Segment {
    fn from(item: Handle) -> Self {
        traits::Segment::from_handle(item)
    }
}

impl Deref for Segment {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.ptr, self.len) }
    }
}
