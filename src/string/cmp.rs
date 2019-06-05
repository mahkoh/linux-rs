// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::cmp::{Eq, PartialOrd, Ord, Ordering};
use crate::string::{
    byte_str::{ByteStr},
    c_str::{CStr},
    no_null_str::{NoNullStr},
};

#[inline(always)] fn dd<T: AsRef<[u8]>+?Sized>(b: &T) -> &[u8] { b.as_ref() }

impl<T: AsRef<[u8]>+?Sized> PartialEq<T> for ByteStr {
    fn eq(&self, other: &T) -> bool {
        dd(self) == dd(other)
    }
}

impl<T: AsRef<[u8]>+?Sized> PartialEq<T> for CStr {
    fn eq(&self, other: &T) -> bool {
        dd(self) == dd(other)
    }
}

impl<T: AsRef<[u8]>+?Sized> PartialEq<T> for NoNullStr {
    fn eq(&self, other: &T) -> bool {
        dd(self) == dd(other)
    }
}

impl PartialEq<ByteStr> for str {
    fn eq(&self, other: &ByteStr) -> bool {
        dd(self) == dd(other)
    }
}

impl PartialEq<ByteStr> for [u8] {
    fn eq(&self, other: &ByteStr) -> bool {
        dd(self) == dd(other)
    }
}

impl PartialEq<CStr> for str {
    fn eq(&self, other: &CStr) -> bool {
        dd(self) == dd(other)
    }
}

impl PartialEq<CStr> for [u8] {
    fn eq(&self, other: &CStr) -> bool {
        dd(self) == dd(other)
    }
}

impl PartialEq<NoNullStr> for str {
    fn eq(&self, other: &NoNullStr) -> bool {
        dd(self) == dd(other)
    }
}

impl PartialEq<NoNullStr> for [u8] {
    fn eq(&self, other: &NoNullStr) -> bool {
        dd(self) == dd(other)
    }
}

impl<T: AsRef<[u8]>+?Sized> PartialOrd<T> for ByteStr {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        Some(dd(self).cmp(dd(other)))
    }
}

impl Ord for ByteStr {
    fn cmp(&self, other: &ByteStr) -> Ordering {
        dd(self).cmp(dd(other))
    }
}
