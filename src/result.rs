use crate::util::error::{Errno};

pub type Result<T = ()> = core::result::Result<T, Errno>;
