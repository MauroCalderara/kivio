// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use std::convert::From;
use std::sync::Arc;

use crate::traits;

use super::{HandleMut, Segment, Vmem};

#[derive(Debug, Clone)]
pub struct Handle {
    pub(super) vmem: Arc<Vmem>,
}

impl traits::Handle for Handle {
    type HandleMut = HandleMut;
    type Segment = Segment;

    fn from_handle_mut(handle_mut: Self::HandleMut) -> Self {
        Self {
            vmem: handle_mut.vmem,
        }
    }

    fn from_segment(segment: Self::Segment) -> Self {
        Self { vmem: segment.vmem }
    }
}

impl From<HandleMut> for Handle {
    fn from(item: HandleMut) -> Self {
        traits::Handle::from_handle_mut(item)
    }
}

impl From<Segment> for Handle {
    fn from(item: Segment) -> Self {
        traits::Handle::from_segment(item)
    }
}
