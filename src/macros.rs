macro_rules! rv {
    ($x:expr) => {{
        let val = $x;
        if val < 0 {
            Err(crate::util::error::Errno(-val as c_int))
        } else {
            Ok(())
        }
    }};
    ($x:expr, -> $t:ty) => {{
        let val = $x;
        if val < 0 {
            Err(crate::util::error::Errno(-val as c_int))
        } else {
            Ok(val as $t)
        }
    }};
}

#[cfg(feature = "std")]
macro_rules! impl_std_write {
    ($ty:ty) => {
        impl std::io::Write for $ty {
            fn write(&mut self, buf: &[u8]) -> std::result::Result<usize, std::io::Error> {
                Ok(Write::write(self, buf)?)
            }

            fn write_vectored(&mut self, bufs: &[std::io::IoSlice]) -> std::result::Result<usize, std::io::Error> {
                unsafe {
                    Ok(Write::gather_write(self, core::mem::transmute(bufs))?)
                }
            }

            fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
                Ok(Write::flush(self)?)
            }
        }
    }
}

#[cfg(feature = "std")]
macro_rules! impl_std_read {
    ($ty:ty) => {
        impl std::io::Read for $ty {
            fn read(&mut self, buf: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
                Ok(Read::read(self, crate::util::data::d8::from_byte_slice_mut(buf))?)
            }

            fn read_vectored(&mut self, bufs: &mut [std::io::IoSliceMut]) -> std::result::Result<usize, std::io::Error> {
                unsafe {
                    Ok(Read::scatter_read(self, core::mem::transmute(bufs))?)
                }
            }

            unsafe fn initializer(&self) -> std::io::Initializer {
                std::io::Initializer::nop()
            }
        }
    }
}

macro_rules! impl_fmt_write {
    ($ty:ty) => {
        impl core::fmt::Write for $ty {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                self.write_all(s.as_bytes()).map(|_| ()).map_err(|_| core::fmt::Error)
            }
        }
    }
}

#[macro_export]
macro_rules! write {
    ($dst:expr, $($arg:tt)*) => ($dst.write_fmt_linux(format_args!($($arg)*)))
}

#[macro_export]
macro_rules! writeln {
    ($dst:expr) => (
        write!($dst, "\n")
    );
    ($dst:expr,) => (
        writeln!($dst)
    );
    ($dst:expr, $arg:expr) => (
        $dst.write_fmt_linux(format_args!(concat!($arg, "\n")))
    );
    ($dst:expr, $arg:expr, $($args:tt)*) => (
        $dst.write_fmt_linux(format_args!(concat!($arg, "\n"), $($args)*))
    );
}