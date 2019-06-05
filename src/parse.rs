// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{mem};
use crate::{
    result::{Result},
    util::{error},
};

/// Types that can be parsed.
pub trait Parse {
    /// Tries to parse the object.
    fn parse<P: Parsable>(&self) -> Result<P>;
}

impl Parse for [u8] { fn parse<P: Parsable>(&self) -> Result<P> { P::parse_bytes(self) } }
impl Parse for str  { fn parse<P: Parsable>(&self) -> Result<P> { P::parse_bytes(self.as_ref()) } }

impl Parse for [i8] {
    fn parse<P: Parsable>(&self) -> Result<P> {
        P::parse_bytes(unsafe { mem::transmute(self.as_ref()) })
    }
}

impl<'a, T: Parse+?Sized> Parse for &'a T {
    fn parse<P: Parsable>(&self) -> Result<P> {
        (**self).parse()
    }
}

/// Types that are parsable.
pub trait Parsable : Sized {
    /// Tries to parse an initial sequence of bytes as an object of this type.
    ///
    /// [argument, bytes]
    /// The bytes to be parsed.
    ///
    /// [return_value]
    /// Returns the object and the number of bytes consumed.
    fn parse_bytes_init(bytes: &[u8]) -> Result<(Self, usize)>;

    /// Tries to parse a byte slice as an object of this type.
    ///
    /// [argument, bytes]
    /// The bytes to be parsed.
    ///
    /// [return_value]
    /// Returns the object.
    ///
    /// = Remarks
    ///
    /// This fails if the whole sequence cannot be parsed, that is, if `parse_bytes_init`
    /// returns that not all of the bytes were parsed.
    fn parse_bytes(bytes: &[u8]) -> Result<Self> {
        match Self::parse_bytes_init(bytes) {
            Ok((v, l)) => {
                if l == bytes.len() {
                    Ok(v)
                } else {
                    Err(error::InvalidArgument)
                }
            },
            Err(e) => Err(e),
        }
    }
}
