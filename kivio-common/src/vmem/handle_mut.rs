// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use std::any::type_name;
use std::convert::TryFrom;
use std::sync::Arc;

use crate::err::Error;
use crate::traits;

use super::{Handle, SegmentMut, Vmem};

#[derive(Debug)]
pub struct HandleMut {
    pub(super) vmem: Arc<Vmem>,
}

impl HandleMut {
    pub fn from_vmem(vmem: Vmem) -> Self {
        Self {
            vmem: Arc::new(vmem),
        }
    }
}

impl traits::HandleMut for HandleMut {
    type Handle = Handle;
    type SegmentMut = SegmentMut;

    fn try_from_handle(handle: Self::Handle) -> Result<Self, (Error, Self::Handle)> {
        match Arc::<Vmem>::try_unwrap(handle.vmem) {
            Ok(i) => Ok(Self { vmem: Arc::new(i) }),
            Err(arc_i) => Err((
                Error::ConversionFailed {
                    from_type: type_name::<Self::Handle>().to_string(),
                    to_type: type_name::<Self>().to_string(),
                    reason: "Outstanding vmem::Handle detected (unable to unwrap() Arc to \
                             Vmem resource)"
                        .to_string(),
                },
                Self::Handle { vmem: arc_i },
            )),
        }
    }

    fn try_from_segment_mut(
        segment_mut: Self::SegmentMut,
    ) -> Result<Self, (Error, Self::SegmentMut)> {
        match Arc::<Vmem>::try_unwrap(segment_mut.vmem) {
            Ok(v) => Ok(Self { vmem: Arc::new(v) }),
            Err(arc_v) => Err((
                Error::ConversionFailed {
                    from_type: type_name::<Self::SegmentMut>().to_string(),
                    to_type: type_name::<Self>().to_string(),
                    reason: "Outstanding vmem::SegmentMuts detected (unable to unwrap() Arc to \
                             Vmem resource)"
                        .to_string(),
                },
                Self::SegmentMut {
                    mut_ptr: segment_mut.mut_ptr,
                    len: segment_mut.len,
                    vmem: arc_v,
                },
            )),
        }
    }
}

impl TryFrom<Handle> for HandleMut {
    type Error = (Error, Handle);

    fn try_from(item: Handle) -> Result<Self, Self::Error> {
        traits::HandleMut::try_from_handle(item)
    }
}

impl TryFrom<SegmentMut> for HandleMut {
    type Error = (Error, SegmentMut);

    fn try_from(item: SegmentMut) -> Result<Self, Self::Error> {
        traits::HandleMut::try_from_segment_mut(item)
    }
}
