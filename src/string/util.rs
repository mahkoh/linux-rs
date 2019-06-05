// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use core::{
    fmt::{Result, Formatter, LowerHex, Write},
    mem, ptr,
};
use crate::kty::{c_char};

fn debug_char_no_quotes(c: char, f: &mut Formatter, esc_double: bool, esc_single: bool) -> Result {
    let val = c as u32;
    if c == '\\' {
        f.write_str("\\\\")?;
    } else if esc_double && c == '"' {
        f.write_str("\\\"")?;
    } else if esc_single && c == '\'' {
        f.write_str("\\'")?;
    } else if 31 < val && val < 127 {
        f.write_char(c)?;
    } else {
        f.write_str("\\u{")?;
        LowerHex::fmt(&val, f)?;
        f.write_str("}")?;
    }
    Ok(())
}

pub fn debug_str_no_quotes(s: &str, f: &mut Formatter) -> Result {
    for c in s.chars() {
        debug_char_no_quotes(c, f, true, false)?;
    }
    Ok(())
}

/// Returns the longest initial sequence of valid UTF-8 in a byte slice.
///
/// [argument, b]
/// The byte slice.
pub fn longest_sequence(b: &[u8]) -> &str {
    let mut idx = 0;
    while idx < b.len() {
        let len = UTF8_CHAR_LEN[b[idx] as usize] as usize;
        if len == 1 { idx += 1; continue; }
        if len == 0 || idx + len > b.len() { break; }
        if len == 2 && (!b[idx+1]).leading_zeros() != 1 { break; }
        if len == 3 {
            match (b[idx], b[idx+1], b[idx+2]) {
                (0xE0,        0xA0...0xBF, 0x80...0xBF) => { },
                (0xE1...0xEC, 0x80...0xBF, 0x80...0xBF) => { },
                (0xED,        0x80...0x9F, 0x80...0xBF) => { },
                _ => break,
            }
        }
        if len == 4 {
            match (b[idx], b[idx+1], b[idx+2], b[idx+3]) {
                (0xF0,        0x90...0xBF, 0x80...0xBF, 0x80...0xBF) => { },
                (0xF1...0xF3, 0x80...0xBF, 0x80...0xBF, 0x80...0xBF) => { },
                (0xF4,        0x80...0x8F, 0x80...0xBF, 0x80...0xBF) => { },
                _ => break,
            }
        }
        idx += len;
    }
    unsafe { mem::transmute(&b[..idx]) }
}

pub static UTF8_CHAR_LEN: [u8; 256] = [
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3,
    4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub unsafe fn strlen(ptr: *const c_char) -> usize {
    let mut i = 0;
    loop {
        if *ptr.offset(i) == 0 {
            return i as usize;
        }
        i += 1;
    }
}
