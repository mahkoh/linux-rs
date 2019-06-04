// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem, slice};
use crate::{lmem};

#[allow(non_camel_case_types)]
#[derive(Pod, Copy, Clone)]
#[repr(transparent)]
pub struct d8(u8);

impl d8 {
    pub const fn new(byte: u8) -> d8 {
        d8(byte)
    }

    pub fn from_byte_slice(bytes: &[u8]) -> &[d8] {
        unsafe {
            slice::from_raw_parts(bytes.as_ptr() as *const _, bytes.len())
        }
    }

    pub fn from_byte_slice_mut(bytes: &mut [u8]) -> &mut [d8] {
        unsafe {
            slice::from_raw_parts_mut(bytes.as_mut_ptr() as *mut _, bytes.len())
        }
    }

    pub fn from_byte_slice_slice<'a, 'b>(bytes: &'a [&'b [u8]]) -> &'a [&'b [d8]] {
        unsafe {
            slice::from_raw_parts(bytes.as_ptr() as *const _, bytes.len())
        }
    }

    pub unsafe fn as_byte(&self) -> &u8 {
        &self.0
    }

    pub unsafe fn as_mut_byte(&mut self) -> &mut u8 {
        &mut self.0
    }
}

pub trait DataSlice {
    unsafe fn as_bytes(&self) -> &[u8];
    unsafe fn as_mut_bytes(&mut self) -> &mut [u8];
    fn align_for<T>(&self) -> &[d8];
    fn align_for_mut<T>(&mut self) -> &mut [d8];
}

impl DataSlice for [d8] {
    unsafe fn as_bytes(&self) -> &[u8] {
        mem::transmute(self)
    }

    unsafe fn as_mut_bytes(&mut self) -> &mut [u8] {
        mem::transmute(self)
    }

    fn align_for<T>(&self) -> &[d8] {
        unsafe { mem::transmute(lmem::align_for::<T>(self.as_bytes())) }
    }

    fn align_for_mut<T>(&mut self) -> &mut [d8] {
        unsafe { mem::transmute(lmem::align_for_mut::<T>(self.as_mut_bytes())) }
    }
}
