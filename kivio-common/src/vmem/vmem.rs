// SPDX-License-Identifier: PolyForm-Noncommercial-1.0.0

use std::vec::Vec;

pub type VecU8 = Vec<u8>;

#[derive(Debug)]
pub struct AnonMmap {
    pub mut_ptr: *mut u8,
    pub len: usize,
}

// TODO need to implement drop and some constructors on Mmap

#[derive(Debug)]
pub enum Vmem {
    VecU8(VecU8),
    AnonMmap(AnonMmap),
}

impl Vmem {
    pub fn new_vec_u8(len: usize) -> Self {
        Self::VecU8(vec![0; len])
    }

    pub fn from_vec_u8(vec: VecU8) -> Self {
        Self::VecU8(vec)
    }

    pub fn from_anon_mmap(anon_mmap: AnonMmap) -> Self {
        Self::AnonMmap(anon_mmap)
    }

    pub(crate) fn mut_ptr_len(&self) -> (*mut u8, usize) {
        match self {
            Self::VecU8(ref v) => (v.as_ptr() as *mut u8, v.len()),
            Self::AnonMmap(ref m) => (m.mut_ptr, m.len),
        }
    }
}
