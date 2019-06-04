// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::util::int::{SignedInt};
use crate::util::error::{Errno, Interrupted};
use crate::kty::{c_int};
use crate::result::{Result};

#[cfg(feature = "retry-interrupted")]
pub fn retry<T: SignedInt>(mut f: impl FnMut() -> T) -> Result<T> {
    loop {
        let ret = f();
        if ret.negative() {
            let err = Errno(-ret.cast_i64() as c_int);
            if err != Interrupted {
                return Err(err);
            }
        } else {
            return Ok(ret);
        }
    }
}

#[cfg(not(feature = "retry-interrupted"))]
pub fn retry<T: SignedInt>(mut f: impl FnMut() -> T) -> Result<T> {
    let ret = f();
    if ret.negative() {
        Err(Errno(-ret.cast_i64() as c_int))
    } else {
        Ok(ret)
    }
}
