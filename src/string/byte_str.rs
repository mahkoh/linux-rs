// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{
    fmt::{self, Debug, Display, Formatter, LowerHex},
    ops::{Deref, DerefMut},
};
use crate::{
    parse::{Parse, Parsable},
    result::{Result},
    string::util::{self},
};

mod index;

/// A borrowed byte sequence that can be interpreted as a string.
///
/// = Remarks
///
/// The Debug implementation prints strings in the formk `"string"` where all letters that
/// are not in the printable ASCII set are printed as escape sequences of the form
/// `\u{number}`.
///
/// The Display implementation writes the contained bytes directly to the output.
#[derive(Eq)]
#[repr(transparent)]
pub struct ByteStr([u8]);

impl ByteStr {
    /// Returns a byte string created by removing spaces and tabs from the start and end
    /// of the string.
    pub fn trim(&self) -> &ByteStr {
        let mut start = 0;
        let mut end = self.0.len();
        while start < self.0.len() {
            match self.0[start] {
                b' ' | b'\t' => { },
                _ => break,
            }
            start += 1;
        }
        while end > start {
            match self.0[end-1] {
                b' ' | b'\t' => { },
                _ => break,
            }
            end -= 1;
        }
        self.0[start..end].as_ref()
    }

    /// Returns whether the string starts with a byte slice.
    ///
    /// [argument, arg]
    /// The byte slice to be checked.
    pub fn starts_with<A: ?Sized>(&self, arg: &A) -> bool
        where A: AsRef<[u8]>,
    {
        self.0.starts_with(arg.as_ref())
    }
}

impl Deref for ByteStr {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.0
    }
}

impl DerefMut for ByteStr {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Debug for ByteStr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut bytes: &[u8] = self.as_ref();
        f.write_str("\"")?;
        while bytes.len() > 0 {
            let remove = {
                let as_str = util::longest_sequence(bytes);
                util::debug_str_no_quotes(as_str, f)?;
                as_str.len()
            };
            bytes = &bytes[remove..];
            if bytes.len() > 0 {
                f.write_str("\\x")?;
                LowerHex::fmt(&bytes[0], f)?;
                bytes = &bytes[1..];
            }
        }
        f.write_str("\"")?;
        Ok(())
    }
}

impl Parse for ByteStr {
    fn parse<P: Parsable>(&self) -> Result<P> {
        self.0.parse()
    }
}
