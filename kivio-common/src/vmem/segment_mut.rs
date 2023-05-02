// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use std::any::type_name;
use std::convert::{From, TryFrom};
use std::ops::{Deref, DerefMut};
use std::slice;
use std::sync::Arc;
use std::vec::Vec;

use crate::err::Error;
use crate::io_vec::IoVec;
use crate::traits;

use super::{HandleMut, Vmem};

#[derive(Debug)]
pub struct SegmentMut {
    pub(super) mut_ptr: *mut u8,
    pub(super) len: usize,
    pub(super) vmem: Arc<Vmem>,
}

impl traits::SegmentMut for SegmentMut {
    type HandleMut = HandleMut;

    fn from_handle_mut(handle_mut: Self::HandleMut) -> Self {
        let (mut_ptr, len) = match *handle_mut.vmem {
            Vmem::VecU8(ref v) => (v.as_ptr() as *mut u8, v.len()),
            Vmem::AnonMmap(ref m) => (m.mut_ptr as *mut u8, m.len),
        };
        Self {
            mut_ptr,
            len,
            vmem: handle_mut.vmem,
        }
    }

    fn try_from_vec_segment_mut(mut v: Vec<Self>) -> Result<Self, (Error, Vec<Self>)> {
        // This is somewhat tricky to make reliable in the presence of different threads
        // potentially holding SegmentMuts to other parts of the memory region represented by the
        // Handle/Vmem. The idea goes as follows:
        // - An empty argument is an obvious error
        // - We check that all segments point to the same Vmem. If they do, there could be more
        //   SegmentMuts outstanding but never less than however many are passed in v.
        // - There can not be any outstanding HandleMut in existence because
        //   - HandleMut is not copyable AND
        //   - The conversion to SegmentMut(s) consumes the HandleMut.
        //   Since we're given at least one SegmentMut, we know that there cannot be a HandleMut
        //   out there.
        // - So iff v.len() == v[0].vmem.strong_count() we know that there are no outstanding
        //   SegmentMuts and we can merge what is given to us into a single SegmentMut.
        if v.is_empty() {
            Err((
                Error::ConversionFailed {
                    from_type: type_name::<Vec<Self>>().to_string(),
                    to_type: type_name::<Self>().to_string(),
                    reason: "Empty Vec<vmem::SegmentMut> provided".to_string(),
                },
                v,
            ))
        } else if !v
            .windows(2)
            .all(|w| Arc::<Vmem>::ptr_eq(&w[0].vmem, &w[1].vmem))
        {
            Err((
                Error::ConversionFailed {
                    from_type: type_name::<Vec<Self>>().to_string(),
                    to_type: type_name::<Self>().to_string(),
                    reason: "Elements of Vec<vmem::SegmentMut> point to different Fd resources"
                        .to_string(),
                },
                v,
            ))
        } else if !Arc::<Vmem>::strong_count(&v[0].vmem) == v.len() {
            Err((
                Error::ConversionFailed {
                    from_type: type_name::<Vec<Self>>().to_string(),
                    to_type: type_name::<Self>().to_string(),
                    reason: "Vec<vmem::SegmentMut> does not contain all existing SegmentMuts \
                             pointing to the Fd resource (Arc's strong_count is greater than the \
                             argument vector's length)"
                        .to_string(),
                },
                v,
            ))
        } else {
            let x = v.pop().unwrap();
            drop(v);
            assert!(
                Arc::<Vmem>::strong_count(&x.vmem) == 1,
                "Spurious outstanding vmem::SegmentMut detected after dropping all but one \
                     vmem::SegmentMut in vector of segments."
            );
            let (mut_ptr, len) = x.vmem.mut_ptr_len();
            Ok(Self {
                mut_ptr,
                len,
                vmem: x.vmem,
            })
        }
    }

    fn try_split(self, io_vec: &IoVec) -> Result<Vec<Self>, (Error, Self)> {
        match io_vec.is_overlapping(self.len) {
            Err(e) => Err((e, self)),
            Ok(true) => Err((
                Error::OverlappingIoVec {
                    outer_len: self.len.clone(),
                },
                self,
            )),
            Ok(false) => {
                let segment_vec: Result<Vec<_>, _> = io_vec
                    .iter()
                    .map(|&r| {
                        let (offset, len) = r.to_offset_len(self.len)?;
                        Ok(Self {
                            mut_ptr: unsafe { self.mut_ptr.offset(offset.try_into().unwrap()) },
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
    }
}

impl traits::VmemSegmentMut for SegmentMut {
    fn mut_ptr_len(&self) -> (*mut u8, usize) {
        (self.mut_ptr, self.len)
    }
}

impl From<HandleMut> for SegmentMut {
    fn from(item: HandleMut) -> Self {
        traits::SegmentMut::from_handle_mut(item)
    }
}

impl TryFrom<Vec<SegmentMut>> for SegmentMut {
    type Error = (Error, Vec<Self>);

    fn try_from(item: Vec<Self>) -> Result<Self, Self::Error> {
        traits::SegmentMut::try_from_vec_segment_mut(item)
    }
}

impl Deref for SegmentMut {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.mut_ptr as *const u8, self.len) }
    }
}

impl DerefMut for SegmentMut {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.mut_ptr, self.len) }
    }
}
