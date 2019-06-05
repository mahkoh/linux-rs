// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{
    fmt::{self, Debug, Display, Formatter},
    ops::{Deref},
    mem,
};
use memchr::{memchr, memrchr};

use crate::string::byte_str::{ByteStr};

mod index;

/// A byte slice with no null bytes.
#[derive(Eq)]
#[repr(transparent)]
pub struct NoNullStr([u8]);

impl NoNullStr {
    /// Sets a byte in the slice to a value.
    ///
    /// [argument, idx]
    /// The index of the byte to be set.
    ///
    /// [argument, val]
    /// The value of the byte.
    ///
    /// = Remarks
    ///
    /// If `val == 0`, the process is aborted.
    pub fn set(&mut self, idx: usize, val: u8) {
        assert_ne!(val, 0);
        self.0[idx] = val;
    }

    /// Returns a `NoNullStr` that consists of the segment after the last '/'.
    pub fn file(&self) -> &NoNullStr {
        self.split_path().1
    }

    /// Returns a mutable `NoNullStr` that consists of the segment after the last '/'.
    pub fn file_mut(&mut self) -> &mut NoNullStr {
        self.split_path_mut().1
    }

    /// Returns a `NoNullStr` that consists of the segment before the last '/'.
    pub fn dir(&self) -> &NoNullStr {
        self.split_path().0
    }

    /// Returns a mutable `NoNullStr` that consists of the segment before the last '/'.
    pub fn dir_mut(&mut self) -> &mut NoNullStr {
        self.split_path_mut().0
    }

    /// Splits the string into its directory and file components.
    pub fn split_path(&self) -> (&NoNullStr, &NoNullStr) {
        let bytes = &self.0;
        let (l, r) = match memrchr(b'/', bytes) {
            Some(idx) => (&bytes[..idx], &bytes[idx+1..]),
            _ => (&[][..], bytes),
        };
        unsafe { mem::transmute((l, r)) }
    }

    /// Splits the string into its directory and file components.
    pub fn split_path_mut(&mut self) -> (&mut NoNullStr, &mut NoNullStr) {
        unsafe { mem::transmute(self.split_path()) }
    }
}

impl Deref for NoNullStr {
    type Target = ByteStr;
    fn deref(&self) -> &ByteStr {
        self.as_ref()
    }
}

impl Debug for NoNullStr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let bs: &ByteStr = self.as_ref();
        Debug::fmt(bs, f)
    }
}
