pub use linux_macros::{PodPub as Pod};

pub unsafe trait Pod: Copy {
    fn _assert_pod() { }
}

macro_rules! i {
    ($t: ty) => {
        unsafe impl Pod for $t { }
    };
}

i!(u8);
i!(i8);
i!(u16);
i!(i16);
i!(u32);
i!(i32);
i!(u64);
i!(i64);
i!(usize);
i!(isize);

macro_rules! j {
    ($n:expr) => {
        unsafe impl <T: Pod> Pod for [T; $n] { }
    };
}

j!(0); j!(1); j!(2); j!(3); j!(4); j!(5); j!(6); j!(7); j!(8); j!(9); j!(10);
j!(11); j!(12); j!(13); j!(14); j!(15); j!(16); j!(17); j!(18); j!(19);
j!(20); j!(21); j!(22); j!(23); j!(24); j!(25); j!(26); j!(27); j!(28);
j!(29); j!(30); j!(31); j!(32);

unsafe impl<T> Pod for *const T { }
unsafe impl<T> Pod for *mut T { }