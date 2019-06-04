#![feature(proc_macro_hygiene, type_ascription)]

use linux::{
    syscall, lmem,
    kty::{CAP_LAST_CAP, k_ulong, __user_cap_data_struct, __u32, c_int},
    util::io::{Write as X},
};
use std::{
    mem,
    io::{Write},
};
use linux::time::clock::{REAL, MONO};

fn set(caps: &[__user_cap_data_struct; 2], cap: c_int) -> bool {
    let aidx = cap as usize / (mem::size_of_val(&caps[0].effective) * 8);
    let sidx = (cap as usize % (mem::size_of_val(&caps[0].effective) * 8)) as __u32;
    let buf = caps[aidx];
    buf.inheritable >> sidx & 1 != 0 &&
        buf.effective >> sidx & 1 != 0 &&
        buf.permitted >> sidx & 1 != 0
}

fn main() {
    let mut buf = lmem::zeroed();
    syscall::capget_v3(0, &mut buf);
    println!("{:032b}", buf[0].permitted);
    println!("{:032b}", buf[0].effective);
    println!("{:032b}", buf[0].inheritable);
    println!("{:032b}", buf[1].permitted);
    println!("{:032b}", buf[1].effective);
    println!("{:032b}", buf[1].inheritable);
    for i in 0..CAP_LAST_CAP {
        if set(&buf, i) {
            syscall::prctl_pr_cap_ambient_raise(i as k_ulong);
        }
    }

    let i = 1;
    linux::writeln!({i}, "Hello World {}", 1 + 1);
//    writeln!(linux::util::fd::STDOUT, "Hello World {}", 1 + 1);
    let mut x: &mut [u8] = &mut [0u8; 128][..];
    writeln!(&mut x, "Hello World");

    //////////////

    println!("{:?}", MONO.get_time());
}