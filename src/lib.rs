#![allow(unused)]
#![cfg_attr(not(feature="std"), no_std)]
#![feature(asm, structural_match, read_initializer)]

#[macro_use]
#[allow(unused_imports)]
extern crate linux_macros;

extern crate alloc;

#[macro_use]
mod macros;

pub mod util;
pub mod kty;
pub mod syscall;
pub mod fd;
//pub mod lock;
pub mod result;
pub mod lmem;
pub mod time;
pub mod string;
pub mod parse;
