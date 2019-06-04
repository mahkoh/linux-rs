// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(nonstandard_style, dead_code)]

use core::{mem};
use crate::{kty};

// XXX: Only use fully qualified types here!!! The types in here might be overridden by
// the types in the non-generic files, therefore we can't use them directly.

pub const BYTES_PER_KERNEL_MODE_T : usize = kty::BYTES_PER_INT;

/////////////////////////////////
// include/uapi/linux/eventpoll.h
/////////////////////////////////

// XXX: This is the epoll_event for all platforms except x86_64
#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct epoll_event {
    pub events: kty::__u32,
    pub data:   kty::__u64,
}

///////////////////////////////
// include/uapi/linux/fadvise.h
///////////////////////////////

// NOTE: These are for all platforms except s390x
pub const POSIX_FADV_DONTNEED : kty::c_int = 4;
pub const POSIX_FADV_NOREUSE  : kty::c_int = 5;


//////////////////////
// include/linux/net.h
//////////////////////

// NOTE: These are for all platforms except MIPS
pub const SOCK_STREAM    : kty::c_int = 1;
pub const SOCK_DGRAM     : kty::c_int = 2;
pub const SOCK_RAW       : kty::c_int = 3;
pub const SOCK_RDM       : kty::c_int = 4;
pub const SOCK_SEQPACKET : kty::c_int = 5;
pub const SOCK_DCCP      : kty::c_int = 6;
pub const SOCK_PACKET    : kty::c_int = 10;
pub const SOCK_MAX       : kty::c_int = kty::SOCK_PACKET + 1;
pub const SOCK_TYPE_MASK : kty::c_int = 0xf;
pub const SOCK_CLOEXEC   : kty::c_int = kty::O_CLOEXEC;
pub const SOCK_NONBLOCK  : kty::c_int = kty::O_NONBLOCK;

/////////////////////////////////////////
// include/uapi/asm-generic/bitsperlong.h
/////////////////////////////////////////

pub const __BITS_PER_LONG: usize = 32;

////////////////////////////////////////
// include/uapi/asm-generic/errno-base.h
////////////////////////////////////////

// XXX: lives in lrs_error

///////////////////////////////////
// include/uapi/asm-generic/errno.h
///////////////////////////////////

// XXX: lives in lrs_error

///////////////////////////////////
// include/uapi/asm-generic/fcntl.h
///////////////////////////////////

pub const O_ACCMODE       : kty::c_int = 0o0000003;
pub const O_RDONLY        : kty::c_int = 0o0000000;
pub const O_WRONLY        : kty::c_int = 0o0000001;
pub const O_RDWR          : kty::c_int = 0o0000002;
pub const O_CREAT         : kty::c_int = 0o0000100;
pub const O_EXCL          : kty::c_int = 0o0000200;
pub const O_NOCTTY        : kty::c_int = 0o0000400;
pub const O_TRUNC         : kty::c_int = 0o0001000;
pub const O_APPEND        : kty::c_int = 0o0002000;
pub const O_NONBLOCK      : kty::c_int = 0o0004000;
pub const O_DSYNC         : kty::c_int = 0o0010000;
pub const FASYNC          : kty::c_int = 0o0020000;
pub const O_DIRECT        : kty::c_int = 0o0040000;
pub const O_LARGEFILE     : kty::c_int = 0o0100000;
pub const O_DIRECTORY     : kty::c_int = 0o0200000;
pub const O_NOFOLLOW      : kty::c_int = 0o0400000;
pub const O_NOATIME       : kty::c_int = 0o1000000;
pub const O_CLOEXEC       : kty::c_int = 0o2000000;
pub const __O_SYNC        : kty::c_int = 0o4000000;
pub const O_SYNC          : kty::c_int = kty::__O_SYNC | kty::O_DSYNC;
pub const O_PATH          : kty::c_int = 0o10000000;
pub const __O_TMPFILE     : kty::c_int = 0o20000000;
pub const O_TMPFILE       : kty::c_int = kty::__O_TMPFILE | kty::O_DIRECTORY;
pub const O_TMPFILE_MASK  : kty::c_int = kty::__O_TMPFILE | kty::O_DIRECTORY | kty::O_CREAT;
pub const O_NDELAY        : kty::c_int = kty::O_NONBLOCK;
pub const F_DUPFD         : kty::c_uint = 0;
pub const F_GETFD         : kty::c_uint = 1;
pub const F_SETFD         : kty::c_uint = 2;
pub const F_GETFL         : kty::c_uint = 3;
pub const F_SETFL         : kty::c_uint = 4;
pub const F_GETLK         : kty::c_uint = 5;
pub const F_SETLK         : kty::c_uint = 6;
pub const F_SETLKW        : kty::c_uint = 7;
pub const F_SETOWN        : kty::c_uint = 8;
pub const F_GETOWN        : kty::c_uint = 9;
pub const F_SETSIG        : kty::c_uint = 10;
pub const F_GETSIG        : kty::c_uint = 11;
pub const F_GETLK64       : kty::c_uint = 12;
pub const F_SETLK64       : kty::c_uint = 13;
pub const F_SETLKW64      : kty::c_uint = 14;
pub const F_SETOWN_EX     : kty::c_uint = 15;
pub const F_GETOWN_EX     : kty::c_uint = 16;
pub const F_GETOWNER_UIDS : kty::c_uint = 17;
pub const F_OFD_GETLK     : kty::c_uint = 36;
pub const F_OFD_SETLK     : kty::c_uint = 37;
pub const F_OFD_SETLKW    : kty::c_uint = 38;
pub const F_OWNER_TID     : kty::c_uint = 0;
pub const F_OWNER_PID     : kty::c_uint = 1;
pub const F_OWNER_PGRP    : kty::c_uint = 2;

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct f_owner_ex {
    pub type_: kty::c_int,
    pub pid: kty::__kernel_pid_t,
}

pub const F_LINUX_SPECIFIC_BASE : kty::c_uint = 1024;
pub const FD_CLOEXEC            : kty::c_int = 1;
pub const F_RDLCK               : kty::c_uint = 0;
pub const F_WRLCK               : kty::c_uint = 1;
pub const F_UNLCK               : kty::c_uint = 2;
pub const F_EXLCK               : kty::c_uint = 4;
pub const F_SHLCK               : kty::c_uint = 8;
pub const LOCK_SH               : kty::c_int = 1;
pub const LOCK_EX               : kty::c_int = 2;
pub const LOCK_NB               : kty::c_int = 4;
pub const LOCK_UN               : kty::c_int = 8;
pub const LOCK_MAND             : kty::c_int = 32;
pub const LOCK_READ             : kty::c_int = 64;
pub const LOCK_WRITE            : kty::c_int = 128;
pub const LOCK_RW               : kty::c_int = 192;

//////////////////////////////////////
// include/uapi/asm-generic/int-ll64.h
//////////////////////////////////////

pub type __s8  = kty::c_schar;
pub type __u8  = kty::c_uchar;
pub type __s16 = kty::c_short;
pub type __u16 = kty::c_ushort;
pub type __s32 = kty::c_int;
pub type __u32 = kty::c_uint;
pub type __s64 = kty::c_longlong;
pub type __u64 = kty::c_ulonglong;

///////////////////////////////////
// include/uapi/asm-generic/ioctl.h
///////////////////////////////////

pub const _IOC_NRBITS   : kty::c_uint = 8;
pub const _IOC_TYPEBITS : kty::c_uint = 8;

pub const _IOC_SIZEBITS : kty::c_uint = 14;
pub const _IOC_DIRBITS  : kty::c_uint = 2;

pub const _IOC_NRMASK    : kty::c_uint = (1 << kty::_IOC_NRBITS) - 1;
pub const _IOC_TYPEMASK  : kty::c_uint = (1 << kty::_IOC_TYPEBITS) - 1;
pub const _IOC_SIZEMASK  : kty::c_uint = (1 << kty::_IOC_SIZEBITS) - 1;
pub const _IOC_DIRMASK   : kty::c_uint = (1 << kty::_IOC_DIRBITS) - 1;
pub const _IOC_NRSHIFT   : kty::c_uint = 0;
pub const _IOC_TYPESHIFT : kty::c_uint = kty::_IOC_NRSHIFT + kty::_IOC_NRBITS;
pub const _IOC_SIZESHIFT : kty::c_uint = kty::_IOC_TYPESHIFT + kty::_IOC_TYPEBITS;
pub const _IOC_DIRSHIFT  : kty::c_uint = kty::_IOC_SIZESHIFT + kty::_IOC_SIZEBITS;

pub const _IOC_NONE  : kty::c_uint = 0;
pub const _IOC_WRITE : kty::c_uint = 1;
pub const _IOC_READ  : kty::c_uint = 2;

pub const fn _IOC(dir: kty::c_uint, ty: kty::c_uint, nr: kty::c_uint,
            size: kty::c_uint) -> kty::c_uint {
    (dir << kty::_IOC_DIRSHIFT) | (ty   << kty::_IOC_TYPESHIFT) |
    (nr  << kty::_IOC_NRSHIFT)  | (size << kty::_IOC_SIZESHIFT)
}

pub fn _IOC_TYPECHECK<T>(_: T) -> kty::c_uint { mem::size_of::<T>() as kty::c_uint }

pub const fn _IO(ty: kty::c_uint, nr: kty::c_uint) -> kty::c_uint {
    _IOC(kty::_IOC_NONE, ty, nr, 0)
}

pub const fn _IOR<T>(ty: kty::c_uint, nr: kty::c_uint) -> kty::c_uint {
    _IOC(kty::_IOC_READ, ty, nr, mem::size_of::<T>() as kty::c_uint)
}

pub const fn _IOW<T>(ty: kty::c_uint, nr: kty::c_uint) -> kty::c_uint {
    _IOC(kty::_IOC_WRITE, ty, nr, mem::size_of::<T>() as kty::c_uint)
}

pub const fn _IOWR<T>(ty: kty::c_uint, nr: kty::c_uint) -> kty::c_uint {
    _IOC(kty::_IOC_READ|kty::_IOC_WRITE, ty, nr, mem::size_of::<T>() as kty::c_uint)
}

pub const fn _IOR_BAD<T>(ty: kty::c_uint, nr: kty::c_uint) -> kty::c_uint {
    _IOC(kty::_IOC_READ, ty, nr, mem::size_of::<T>() as kty::c_uint)
}

pub const fn _IOW_BAD<T>(ty: kty::c_uint, nr: kty::c_uint) -> kty::c_uint {
    _IOC(kty::_IOC_WRITE, ty, nr, mem::size_of::<T>() as kty::c_uint)
}

pub const fn _IOWR_BAD<T>(ty: kty::c_uint, nr: kty::c_uint) -> kty::c_uint {
    _IOC(kty::_IOC_READ|kty::_IOC_WRITE, ty, nr, mem::size_of::<T>() as kty::c_uint)
}

pub const fn _IOC_DIR(nr:  kty::c_uint) -> kty::c_uint { (nr >> kty::_IOC_DIRSHIFT)  & kty::_IOC_DIRMASK  }
pub const fn _IOC_TYPE(nr: kty::c_uint) -> kty::c_uint { (nr >> kty::_IOC_TYPESHIFT) & kty::_IOC_TYPEMASK }
pub const fn _IOC_NR(nr:   kty::c_uint) -> kty::c_uint { (nr >> kty::_IOC_NRSHIFT)   & kty::_IOC_NRMASK   }
pub const fn _IOC_SIZE(nr: kty::c_uint) -> kty::c_uint { (nr >> kty::_IOC_SIZESHIFT) & kty::_IOC_SIZEMASK }

pub const IOC_IN        : kty::c_uint = kty::_IOC_WRITE             << kty::_IOC_DIRSHIFT;
pub const IOC_OUT       : kty::c_uint = kty::_IOC_READ              << kty::_IOC_DIRSHIFT;
pub const IOC_INOUT     : kty::c_uint = (kty::_IOC_WRITE | kty::_IOC_READ) << kty::_IOC_DIRSHIFT;
pub const IOCSIZE_MASK  : kty::c_uint = kty::_IOC_SIZEMASK          << kty::_IOC_SIZESHIFT;
pub const IOCSIZE_SHIFT : kty::c_uint = kty::_IOC_SIZESHIFT;

////////////////////////////////////
// include/uapi/asm-generic/ioctls.h
////////////////////////////////////

pub const TCGETS             : kty::c_uint = 0x5401;
pub const TCSETS             : kty::c_uint = 0x5402;
pub const TCSETSW            : kty::c_uint = 0x5403;
pub const TCSETSF            : kty::c_uint = 0x5404;
pub const TCGETA             : kty::c_uint = 0x5405;
pub const TCSETA             : kty::c_uint = 0x5406;
pub const TCSETAW            : kty::c_uint = 0x5407;
pub const TCSETAF            : kty::c_uint = 0x5408;
pub const TCSBRK             : kty::c_uint = 0x5409;
pub const TCXONC             : kty::c_uint = 0x540A;
pub const TCFLSH             : kty::c_uint = 0x540B;
pub const TIOCEXCL           : kty::c_uint = 0x540C;
pub const TIOCNXCL           : kty::c_uint = 0x540D;
pub const TIOCSCTTY          : kty::c_uint = 0x540E;
pub const TIOCGPGRP          : kty::c_uint = 0x540F;
pub const TIOCSPGRP          : kty::c_uint = 0x5410;
pub const TIOCOUTQ           : kty::c_uint = 0x5411;
pub const TIOCSTI            : kty::c_uint = 0x5412;
pub const TIOCGWINSZ         : kty::c_uint = 0x5413;
pub const TIOCSWINSZ         : kty::c_uint = 0x5414;
pub const TIOCMGET           : kty::c_uint = 0x5415;
pub const TIOCMBIS           : kty::c_uint = 0x5416;
pub const TIOCMBIC           : kty::c_uint = 0x5417;
pub const TIOCMSET           : kty::c_uint = 0x5418;
pub const TIOCGSOFTCAR       : kty::c_uint = 0x5419;
pub const TIOCSSOFTCAR       : kty::c_uint = 0x541A;
pub const FIONREAD           : kty::c_uint = 0x541B;
pub const TIOCINQ            : kty::c_uint = kty::FIONREAD;
pub const TIOCLINUX          : kty::c_uint = 0x541C;
pub const TIOCCONS           : kty::c_uint = 0x541D;
pub const TIOCGSERIAL        : kty::c_uint = 0x541E;
pub const TIOCSSERIAL        : kty::c_uint = 0x541F;
pub const TIOCPKT            : kty::c_uint = 0x5420;
pub const FIONBIO            : kty::c_uint = 0x5421;
pub const TIOCNOTTY          : kty::c_uint = 0x5422;
pub const TIOCSETD           : kty::c_uint = 0x5423;
pub const TIOCGETD           : kty::c_uint = 0x5424;
pub const TCSBRKP            : kty::c_uint = 0x5425;
pub const TIOCSBRK           : kty::c_uint = 0x5427;
pub const TIOCCBRK           : kty::c_uint = 0x5428;
pub const TIOCGSID           : kty::c_uint = 0x5429;
pub const TIOCGRS485         : kty::c_uint = 0x542E;
pub const TIOCSRS485         : kty::c_uint = 0x542F;
pub const TCGETX             : kty::c_uint = 0x5432;
pub const TCSETX             : kty::c_uint = 0x5433;
pub const TCSETXF            : kty::c_uint = 0x5434;
pub const TCSETXW            : kty::c_uint = 0x5435;
pub const TIOCVHANGUP        : kty::c_uint = 0x5437;
pub const FIONCLEX           : kty::c_uint = 0x5450;
pub const FIOCLEX            : kty::c_uint = 0x5451;
pub const FIOASYNC           : kty::c_uint = 0x5452;
pub const TIOCSERCONFIG      : kty::c_uint = 0x5453;
pub const TIOCSERGWILD       : kty::c_uint = 0x5454;
pub const TIOCSERSWILD       : kty::c_uint = 0x5455;
pub const TIOCGLCKTRMIOS     : kty::c_uint = 0x5456;
pub const TIOCSLCKTRMIOS     : kty::c_uint = 0x5457;
pub const TIOCSERGSTRUCT     : kty::c_uint = 0x5458;
pub const TIOCSERGETLSR      : kty::c_uint = 0x5459;
pub const TIOCSERGETMULTI    : kty::c_uint = 0x545A;
pub const TIOCSERSETMULTI    : kty::c_uint = 0x545B;
pub const TIOCMIWAIT         : kty::c_uint = 0x545C;
pub const TIOCGICOUNT        : kty::c_uint = 0x545D;
pub const FIOQSIZE           : kty::c_uint = 0x5460;
pub const TIOCPKT_DATA       : kty::c_uint = 0;
pub const TIOCPKT_FLUSHREAD  : kty::c_uint = 1;
pub const TIOCPKT_FLUSHWRITE : kty::c_uint = 2;
pub const TIOCPKT_STOP       : kty::c_uint = 4;
pub const TIOCPKT_START      : kty::c_uint = 8;
pub const TIOCPKT_NOSTOP     : kty::c_uint = 16;
pub const TIOCPKT_DOSTOP     : kty::c_uint = 32;
pub const TIOCPKT_IOCTL      : kty::c_uint = 64;
pub const TIOCSER_TEMT       : kty::c_uint = 0x01;

// XXX: Maybe replace these by their values?
pub const fn TCGETS2()    -> kty::c_uint { _IOR::<kty::termios2>(b'T' as kty::c_uint, 0x2A) }
pub const fn TCSETS2()    -> kty::c_uint { _IOW::<kty::termios2>(b'T' as kty::c_uint, 0x2B) }
pub const fn TCSETSW2()   -> kty::c_uint { _IOW::<kty::termios2>(b'T' as kty::c_uint, 0x2C) }
pub const fn TCSETSF2()   -> kty::c_uint { _IOW::<kty::termios2>(b'T' as kty::c_uint, 0x2D) }
pub const fn TIOCGPTN()   -> kty::c_uint { _IOR::<kty::c_uint>(b'T'   as kty::c_uint, 0x30) }
pub const fn TIOCSPTLCK() -> kty::c_uint { _IOW::<kty::c_int>(b'T'    as kty::c_uint, 0x31) }
pub const fn TIOCGDEV()   -> kty::c_uint { _IOR::<kty::c_uint>(b'T'   as kty::c_uint, 0x32) }
pub const fn TIOCSIG()    -> kty::c_uint { _IOW::<kty::c_int>(b'T'    as kty::c_uint, 0x36) }
pub const fn TIOCGPKT()   -> kty::c_uint { _IOR::<kty::c_int>(b'T'    as kty::c_uint, 0x38) }
pub const fn TIOCGPTLCK() -> kty::c_uint { _IOR::<kty::c_int>(b'T'    as kty::c_uint, 0x39) }
pub const fn TIOCGEXCL()  -> kty::c_uint { _IOR::<kty::c_int>(b'T'    as kty::c_uint, 0x40) }

////////////////////////////////////
// include/uapi/asm-generic/ipcbuf.h
////////////////////////////////////

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct ipc64_perm {
    pub key:  kty::__kernel_key_t,
    pub uid:  kty::__kernel_uid32_t,
    pub gid:  kty::__kernel_gid32_t,
    pub cuid: kty::__kernel_uid32_t,
    pub cgid: kty::__kernel_gid32_t,
    pub mode: kty::__kernel_mode_t,
    pub __pad1: [kty::c_uchar; 4 - kty::BYTES_PER_KERNEL_MODE_T],
    pub seq:       kty::c_ushort,
    pub __pad2:    kty::c_ushort,
    pub __unused1: kty::__kernel_ulong_t,
    pub __unused2: kty::__kernel_ulong_t,
}

/////////////////////////////////////////
// include/uapi/asm-generic/mman-common.h
/////////////////////////////////////////

pub const PROT_READ         : kty::c_int = 0x1;
pub const PROT_WRITE        : kty::c_int = 0x2;
pub const PROT_EXEC         : kty::c_int = 0x4;
pub const PROT_SEM          : kty::c_int = 0x8;
pub const PROT_NONE         : kty::c_int = 0x0;
pub const PROT_GROWSDOWN    : kty::c_int = 0x01000000;
pub const PROT_GROWSUP      : kty::c_int = 0x02000000;
pub const MAP_SHARED        : kty::c_int = 0x01;
pub const MAP_PRIVATE       : kty::c_int = 0x02;
pub const MAP_TYPE          : kty::c_int = 0x0f;
pub const MAP_FIXED         : kty::c_int = 0x10;
pub const MAP_ANONYMOUS     : kty::c_int = 0x20;
pub const MAP_UNINITIALIZED : kty::c_int = 0x4000000;
pub const MS_ASYNC          : kty::c_int = 1;
pub const MS_INVALIDATE     : kty::c_int = 2;
pub const MS_SYNC           : kty::c_int = 4;
pub const MADV_NORMAL       : kty::c_int = 0;
pub const MADV_RANDOM       : kty::c_int = 1;
pub const MADV_SEQUENTIAL   : kty::c_int = 2;
pub const MADV_WILLNEED     : kty::c_int = 3;
pub const MADV_DONTNEED     : kty::c_int = 4;
pub const MADV_REMOVE       : kty::c_int = 9;
pub const MADV_DONTFORK     : kty::c_int = 10;
pub const MADV_DOFORK       : kty::c_int = 11;
pub const MADV_HWPOISON     : kty::c_int = 100;
pub const MADV_SOFT_OFFLINE : kty::c_int = 101;
pub const MADV_MERGEABLE    : kty::c_int = 12;
pub const MADV_UNMERGEABLE  : kty::c_int = 13;
pub const MADV_HUGEPAGE     : kty::c_int = 14;
pub const MADV_NOHUGEPAGE   : kty::c_int = 15;
pub const MADV_DONTDUMP     : kty::c_int = 16;
pub const MADV_DODUMP       : kty::c_int = 17;
pub const MAP_FILE          : kty::c_int = 0;
pub const MAP_HUGE_SHIFT    : kty::c_int = 26;
pub const MAP_HUGE_MASK     : kty::c_int = 0x3f;

//////////////////////////////////
// include/uapi/asm-generic/mman.h
//////////////////////////////////

pub const MAP_GROWSDOWN  : kty::c_int = 0x0100;
pub const MAP_DENYWRITE  : kty::c_int = 0x0800;
pub const MAP_EXECUTABLE : kty::c_int = 0x1000;
pub const MAP_LOCKED     : kty::c_int = 0x2000;
pub const MAP_NORESERVE  : kty::c_int = 0x4000;
pub const MAP_POPULATE   : kty::c_int = 0x8000;
pub const MAP_NONBLOCK   : kty::c_int = 0x10000;
pub const MAP_STACK      : kty::c_int = 0x20000;
pub const MAP_HUGETLB    : kty::c_int = 0x40000;
pub const MCL_CURRENT    : kty::c_int = 1;
pub const MCL_FUTURE     : kty::c_int = 2;

////////////////////////////////////
// include/uapi/asm-generic/msgbuf.h
////////////////////////////////////

// struct msqid64_ds {
//         struct ipc64_perm msg_perm;
//         __kernel_time_t msg_stime;        /* last msgsnd time */
// #if __BITS_PER_LONG != 64
//         unsigned long        __unused1;
// #endif
//         __kernel_time_t msg_rtime;        /* last msgrcv time */
// #if __BITS_PER_LONG != 64
//         unsigned long        __unused2;
// #endif
//         __kernel_time_t msg_ctime;        /* last change time */
// #if __BITS_PER_LONG != 64
//         unsigned long        __unused3;
// #endif
//         __kernel_ulong_t msg_cbytes;        /* current number of bytes on queue */
//         __kernel_ulong_t msg_qnum;        /* number of messages in queue */
//         __kernel_ulong_t msg_qbytes;        /* max number of bytes on queue */
//         __kernel_pid_t msg_lspid;        /* pid of last msgsnd */
//         __kernel_pid_t msg_lrpid;        /* last receive pid */
//         __kernel_ulong_t __unused4;
//         __kernel_ulong_t __unused5;
// };

///////////////////////////////////
// include/uapi/asm-generic/param.h
///////////////////////////////////

pub const HZ             : usize = 100;
pub const EXEC_PAGESIZE  : usize = 4096;
pub const NOGROUP        : usize = !0; // XXX: Not sure about the type of this
pub const MAXHOSTNAMELEN : usize = 64;

//////////////////////////////////
// include/uapi/asm-generic/poll.h
//////////////////////////////////

pub const POLLIN         : kty::c_uint = 0x0001;
pub const POLLPRI        : kty::c_uint = 0x0002;
pub const POLLOUT        : kty::c_uint = 0x0004;
pub const POLLERR        : kty::c_uint = 0x0008;
pub const POLLHUP        : kty::c_uint = 0x0010;
pub const POLLNVAL       : kty::c_uint = 0x0020;
pub const POLLRDNORM     : kty::c_uint = 0x0040;
pub const POLLRDBAND     : kty::c_uint = 0x0080;
pub const POLLWRNORM     : kty::c_uint = 0x0100;
pub const POLLWRBAND     : kty::c_uint = 0x0200;
pub const POLLMSG        : kty::c_uint = 0x0400;
pub const POLLREMOVE     : kty::c_uint = 0x1000;
pub const POLLRDHUP      : kty::c_uint = 0x2000;
pub const POLLFREE       : kty::c_uint = 0x4000;
pub const POLL_BUSY_LOOP : kty::c_uint = 0x8000;

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct pollfd {
    pub fd:      kty::c_int,
    pub events:  kty::c_short,
    pub revents: kty::c_short,
}

/////////////////////////////////////////
// include/uapi/asm-generic/posix_types.h
/////////////////////////////////////////

pub type __kernel_long_t      = kty::c_long;
pub type __kernel_ulong_t     = kty::c_ulong;
pub type __kernel_ino_t       = kty::__kernel_ulong_t;
pub type __kernel_mode_t      = kty::c_uint;
pub type __kernel_pid_t       = kty::c_int;
pub type __kernel_ipc_pid_t   = kty::c_int;
pub type __kernel_uid_t       = kty::c_uint;
pub type __kernel_gid_t       = kty::c_uint;
pub type __kernel_suseconds_t = kty::__kernel_long_t;
pub type __kernel_daddr_t     = kty::c_int;
pub type __kernel_uid32_t     = kty::c_uint;
pub type __kernel_gid32_t     = kty::c_uint;
pub type __kernel_old_uid_t   = kty::__kernel_uid_t;
pub type __kernel_old_gid_t   = kty::__kernel_gid_t;
pub type __kernel_old_dev_t   = kty::c_uint;
pub type __kernel_off_t       = kty::__kernel_long_t;
pub type __kernel_loff_t      = kty::c_longlong;
pub type __kernel_time_t      = kty::__kernel_long_t;
pub type __kernel_clock_t     = kty::__kernel_long_t;
pub type __kernel_timer_t     = kty::c_int;
pub type __kernel_clockid_t   = kty::c_int;
pub type __kernel_caddr_t     = *mut kty::c_char;
pub type __kernel_uid16_t     = kty::c_ushort;
pub type __kernel_gid16_t     = kty::c_ushort;

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct __kernel_fsid_t {
    pub val: [kty::c_int; 2],
}

//////////////////////////////////////
// include/uapi/asm-generic/resource.h
//////////////////////////////////////

pub const RLIMIT_CPU        : kty::c_ulong = 0;
pub const RLIMIT_FSIZE      : kty::c_ulong = 1;
pub const RLIMIT_DATA       : kty::c_ulong = 2;
pub const RLIMIT_STACK      : kty::c_ulong = 3;
pub const RLIMIT_CORE       : kty::c_ulong = 4;
pub const RLIMIT_RSS        : kty::c_ulong = 5;
pub const RLIMIT_NPROC      : kty::c_ulong = 6;
pub const RLIMIT_NOFILE     : kty::c_ulong = 7;
pub const RLIMIT_MEMLOCK    : kty::c_ulong = 8;
pub const RLIMIT_AS         : kty::c_ulong = 9;
pub const RLIMIT_LOCKS      : kty::c_ulong = 10;
pub const RLIMIT_SIGPENDING : kty::c_ulong = 11;
pub const RLIMIT_MSGQUEUE   : kty::c_ulong = 12;
pub const RLIMIT_NICE       : kty::c_ulong = 13;
pub const RLIMIT_RTPRIO     : kty::c_ulong = 14;
pub const RLIMIT_RTTIME     : kty::c_ulong = 15;
pub const RLIM_NLIMITS      : kty::c_ulong = 16;
pub const RLIM_INFINITY     : kty::c_ulong = !0;

////////////////////////////////////
// include/uapi/asm-generic/sembuf.h
////////////////////////////////////

// struct semid64_ds {
//         struct ipc64_perm sem_perm;        /* permissions .. see ipc.h */
//         __kernel_time_t        sem_otime;        /* last semop time */
// #if __BITS_PER_LONG != 64
//         unsigned long        __unused1;
// #endif
//         __kernel_time_t        sem_ctime;        /* last change time */
// #if __BITS_PER_LONG != 64
//         unsigned long        __unused2;
// #endif
//         unsigned long        sem_nsems;        /* no. of semaphores in array */
//         unsigned long        __unused3;
//         unsigned long        __unused4;
// };

///////////////////////////////////
// include/uapi/asm-generic/setup.h
///////////////////////////////////

pub const COMMAND_LINE_SIZE : usize = 512;

////////////////////////////////////
// include/uapi/asm-generic/shmbuf.h
////////////////////////////////////

// struct shmid64_ds {
//         struct ipc64_perm        shm_perm;        /* operation perms */
//         size_t                        shm_segsz;        /* size of segment (bytes) */
//         __kernel_time_t                shm_atime;        /* last attach time */
// #if __BITS_PER_LONG != 64
//         unsigned long                __unused1;
// #endif
//         __kernel_time_t                shm_dtime;        /* last detach time */
// #if __BITS_PER_LONG != 64
//         unsigned long                __unused2;
// #endif
//         __kernel_time_t                shm_ctime;        /* last change time */
// #if __BITS_PER_LONG != 64
//         unsigned long                __unused3;
// #endif
//         __kernel_pid_t                shm_cpid;        /* pid of creator */
//         __kernel_pid_t                shm_lpid;        /* pid of last operator */
//         __kernel_ulong_t        shm_nattch;        /* no. of current attaches */
//         __kernel_ulong_t        __unused4;
//         __kernel_ulong_t        __unused5;
// };

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct shminfo64 {
    pub shmmax:    kty::__kernel_ulong_t,
    pub shmmin:    kty::__kernel_ulong_t,
    pub shmmni:    kty::__kernel_ulong_t,
    pub shmseg:    kty::__kernel_ulong_t,
    pub shmall:    kty::__kernel_ulong_t,
    pub __unused1: kty::__kernel_ulong_t,
    pub __unused2: kty::__kernel_ulong_t,
    pub __unused3: kty::__kernel_ulong_t,
    pub __unused4: kty::__kernel_ulong_t,
}

//////////////////////////////////////
// include/uapi/asm-generic/shmparam.h
//////////////////////////////////////

pub const SHMLBA : usize = kty::PAGE_SIZE;

/////////////////////////////////////
// include/uapi/asm-generic/siginfo.h
/////////////////////////////////////

// XXX: Assumes that usize == *void >= int
#[repr(C)]
#[derive(Pod, Copy, Clone)]
pub union sigval_t {
    sigval_int: kty::c_int,
    sigval_ptr: *mut kty::c_void,
}

pub const __ARCH_SI_PREAMBLE_SIZE: usize = 3 * kty::BYTES_PER_INT;
pub const SI_MAX_SIZE: usize = 128;
pub const SI_PAD_SIZE: usize = (kty::SI_MAX_SIZE - kty::__ARCH_SI_PREAMBLE_SIZE) / kty::BYTES_PER_INT;
pub type __ARCH_SI_UID_T   = kty::__kernel_uid32_t;
pub type __ARCH_SI_BAND_T  = kty::c_long;
pub type __ARCH_SI_CLOCK_T = kty::__kernel_clock_t;
pub const BYTES_PER_ARCH_SI_UID_T: usize = kty::BYTES_PER_INT;

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct siginfo_kill {
    pub _pid: kty::__kernel_pid_t,
    pub _uid: kty::__ARCH_SI_UID_T,
}

#[repr(C)]
#[derive(Pod, Copy, Clone)]
pub struct siginfo_timer {
    pub _tid: kty::__kernel_timer_t,
    pub _overrun: kty::c_int,
    pub _pad: [kty::c_char; kty::BYTES_PER_ARCH_SI_UID_T - kty::BYTES_PER_INT],
    pub _sigval: kty::sigval_t,
    pub _sys_private: kty::c_int,
}

#[repr(C)]
#[derive(Pod, Copy, Clone)]
pub struct siginfo_rt {
    pub _pid: kty::__kernel_pid_t,
    pub _uid: kty::__ARCH_SI_UID_T,
    pub _sigval: kty::sigval_t,
}

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct siginfo_sigchld {
    pub _pid: kty::__kernel_pid_t,
    pub _uid: kty::__ARCH_SI_UID_T,
    pub _status: kty::c_int,
    pub _utime: kty::__ARCH_SI_CLOCK_T,
    pub _stime: kty::__ARCH_SI_CLOCK_T,
}

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct siginfo_addr_bnd {
    pub _lower: *mut kty::c_void,
    pub _upper: *mut kty::c_void,
}

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct siginfo_sigpoll {
    pub _band: kty::__ARCH_SI_BAND_T,
    pub _fd: kty::c_int,
}

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct siginfo_sigsys {
    pub _call_addr: *mut kty::c_void,
    pub _syscall: kty::c_int,
    pub _arch: kty::c_uint,
}

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct siginfo_sigfault {
    pub _addr: *mut kty::c_void,
}

#[repr(C)]
#[derive(Pod, Copy, Clone)]
pub union _siginfo_variant {
    _kill: kty::siginfo_kill,
    _timer: kty::siginfo_timer,
    _rt: kty::siginfo_rt,
    _sigchld: kty::siginfo_sigchld,
    _sigfault: kty::siginfo_sigfault,
    _sigpoll: kty::siginfo_sigpoll,
    _sigsys: kty::siginfo_sigsys,
}

#[repr(C)]
#[derive(Pod, Copy, Clone)]
pub struct _siginfo_fields {
    si_signo: kty::c_int,
    si_errno: kty::c_int,
    si_code: kty::c_int,
    _variant: kty::_siginfo_variant,
}

#[repr(C)]
#[derive(Pod, Copy, Clone)]
pub union siginfo_t {
    _fields: kty::_siginfo_fields,
    _si_pad: [kty::c_int; SI_MAX_SIZE / mem::size_of::<kty::c_int>()],
}

macro_rules! si_define_outer {
    ($fn: ident, $fn_ref: ident, $fn_mut: ident) => {
        impl kty::siginfo_t {
            pub fn $fn(&self) -> kty::c_int {
                unsafe { self._fields.$fn }
            }

            pub fn $fn_ref(&self) -> &kty::c_int {
                unsafe { &self._fields.$fn }
            }

            pub fn $fn_mut(&mut self) -> &mut kty::c_int {
                unsafe { &mut self._fields.$fn }
            }
        }
    };
}

si_define_outer!(si_signo, si_signo_ref, si_signo_mut);
si_define_outer!(si_errno, si_errno_ref, si_errno_mut);
si_define_outer!(si_code, si_code_ref, si_code_mut);

macro_rules! si_define_inner {
    ($fn: ident, $fn_ref: ident, $fn_mut: ident, $ty: ty, $($path: ident).+) => {
        impl kty::siginfo_t {
            pub unsafe fn $fn(&self) -> $ty {
                self._fields._variant.$($path).+
            }

            pub unsafe fn $fn_ref(&self) -> &$ty {
                &self._fields._variant.$($path).+
            }

            pub unsafe fn $fn_mut(&mut self) -> &mut $ty {
                &mut self._fields._variant.$($path).+
            }
        }
    };
}

si_define_inner!(si_pid,         si_pid_ref,         si_pid_mut,         kty::c_int,             _kill._pid             );
si_define_inner!(si_uid,         si_uid_ref,         si_uid_mut,         kty::c_uint,            _kill._uid             );
si_define_inner!(si_tid,         si_tid_ref,         si_tid_mut,         kty::__kernel_timer_t,  _timer._tid            );
si_define_inner!(si_overrun,     si_overrun_ref,     si_overrun_mut,     kty::c_int,             _timer._overrun        );
si_define_inner!(si_sys_private, si_sys_private_ref, si_sys_private_mut, kty::c_int,             _timer._sys_private    );
si_define_inner!(si_status,      si_status_ref,      si_status_mut,      kty::c_int,             _sigchld._status       );
si_define_inner!(si_utime,       si_utime_ref,       si_utime_mut,       kty::__ARCH_SI_CLOCK_T, _sigchld._utime        );
si_define_inner!(si_stime,       si_stime_ref,       si_stime_mut,       kty::__ARCH_SI_CLOCK_T, _sigchld._stime        );
si_define_inner!(si_value,       si_value_ref,       si_value_mut,       kty::sigval_t,          _rt._sigval            );
si_define_inner!(si_int,         si_int_ref,         si_int_mut,         kty::c_int,             _rt._sigval.sigval_int );
si_define_inner!(si_ptr,         si_ptr_ref,         si_ptr_mut,         *mut kty::c_void,       _rt._sigval.sigval_ptr );
si_define_inner!(si_addr,        si_addr_ref,        si_addr_mut,        *mut kty::c_void,       _sigfault._addr        );
si_define_inner!(si_band,        si_band_ref,        si_band_mut,        kty::__ARCH_SI_BAND_T,  _sigpoll._band         );
si_define_inner!(si_fd,          si_fd_ref,          si_fd_mut,          kty::c_int,             _sigpoll._fd           );
si_define_inner!(si_call_addr,   si_call_addr_ref,   si_call_addr_mut,   *mut kty::c_void,       _sigsys._call_addr     );
si_define_inner!(si_syscall,     si_syscall_ref,     si_syscall_mut,     kty::c_int,             _sigsys._syscall       );
si_define_inner!(si_arch,        si_arch_ref,        si_arch_mut,        kty::c_uint,            _sigsys._arch          );

pub const SI_USER     : kty::c_int = 0;
pub const SI_KERNEL   : kty::c_int = 0x80;
pub const SI_QUEUE    : kty::c_int = -1;
pub const SI_TIMER    : kty::c_int = -2;
pub const SI_MESGQ    : kty::c_int = -3;
pub const SI_ASYNCIO  : kty::c_int = -4;
pub const SI_SIGIO    : kty::c_int = -5;
pub const SI_TKILL    : kty::c_int = -6;
pub const SI_DETHREAD : kty::c_int = -7;

pub const ILL_ILLOPC : kty::c_int = 1;
pub const ILL_ILLOPN : kty::c_int = 2;
pub const ILL_ILLADR : kty::c_int = 3;
pub const ILL_ILLTRP : kty::c_int = 4;
pub const ILL_PRVOPC : kty::c_int = 5;
pub const ILL_PRVREG : kty::c_int = 6;
pub const ILL_COPROC : kty::c_int = 7;
pub const ILL_BADSTK : kty::c_int = 8;
pub const NSIGILL : usize = 8;

pub const FPE_INTDIV : kty::c_int = 1;
pub const FPE_INTOVF : kty::c_int = 2;
pub const FPE_FLTDIV : kty::c_int = 3;
pub const FPE_FLTOVF : kty::c_int = 4;
pub const FPE_FLTUND : kty::c_int = 5;
pub const FPE_FLTRES : kty::c_int = 6;
pub const FPE_FLTINV : kty::c_int = 7;
pub const FPE_FLTSUB : kty::c_int = 8;
pub const NSIGFPE : usize = 8;

pub const SEGV_MAPERR : kty::c_int = 1;
pub const SEGV_ACCERR : kty::c_int = 2;
pub const SEGV_BNDERR : kty::c_int = 3;

pub const BUS_ADRALN    : kty::c_int = 1;
pub const BUS_ADRERR    : kty::c_int = 2;
pub const BUS_OBJERR    : kty::c_int = 3;
pub const BUS_MCEERR_AR : kty::c_int = 4;
pub const BUS_MCEERR_AO : kty::c_int = 5;

pub const TRAP_BRKPT  : kty::c_int = 1;
pub const TRAP_TRACE  : kty::c_int = 2;
pub const TRAP_BRANCH : kty::c_int = 3;
pub const TRAP_HWBKPT : kty::c_int = 4;
pub const NSIGTRAP : usize = 4;

pub const CLD_EXITED    : kty::c_int = 1;
pub const CLD_KILLED    : kty::c_int = 2;
pub const CLD_DUMPED    : kty::c_int = 3;
pub const CLD_TRAPPED   : kty::c_int = 4;
pub const CLD_STOPPED   : kty::c_int = 5;
pub const CLD_CONTINUED : kty::c_int = 6;
pub const NSIGCHLD : usize = 6;

pub const POLL_IN  : kty::c_int = 1;
pub const POLL_OUT : kty::c_int = 2;
pub const POLL_MSG : kty::c_int = 3;
pub const POLL_ERR : kty::c_int = 4;
pub const POLL_PRI : kty::c_int = 5;
pub const POLL_HUP : kty::c_int = 6;
pub const NSIGPOLL : usize = 6;

pub const SYS_SECCOMP : kty::c_int = 1;
pub const NSIGSYS : usize = 1;

pub const SIGEV_SIGNAL    : kty::c_int = 0;
pub const SIGEV_NONE      : kty::c_int = 1;
pub const SIGEV_THREAD    : kty::c_int = 2;
pub const SIGEV_THREAD_ID : kty::c_int = 4;

/////////////////////////////////////////
// include/uapi/asm-generic/signal-defs.h
/////////////////////////////////////////

pub const SIG_BLOCK   : kty::c_int = 0;
pub const SIG_UNBLOCK : kty::c_int = 1;
pub const SIG_SETMASK : kty::c_int = 2;

// can't write this
// type __signalfn_t = *(extern fn(c_int));
pub type __sighandler_t = extern fn(kty::c_int);

// can't write this
// type __restorefn_t = *(extern fn());
pub type __sigrestore_t = extern fn();

pub const SIG_DFL : usize = 0;
pub const SIG_IGN : usize = 1;
pub const SIG_ERR : usize = !0;

////////////////////////////////////
// include/uapi/asm-generic/signal.h
////////////////////////////////////

pub const _NSIG       : usize = 64;
pub const _NSIG_BPW   : usize = kty::BITS_PER_C_ULONG; // c_ulong for compat
pub const _NSIG_WORDS : usize = kty::_NSIG / kty::_NSIG_BPW;

pub const SIGHUP    : kty::c_int = 1;
pub const SIGINT    : kty::c_int = 2;
pub const SIGQUIT   : kty::c_int = 3;
pub const SIGILL    : kty::c_int = 4;
pub const SIGTRAP   : kty::c_int = 5;
pub const SIGABRT   : kty::c_int = 6;
pub const SIGIOT    : kty::c_int = 6;
pub const SIGBUS    : kty::c_int = 7;
pub const SIGFPE    : kty::c_int = 8;
pub const SIGKILL   : kty::c_int = 9;
pub const SIGUSR1   : kty::c_int = 10;
pub const SIGSEGV   : kty::c_int = 11;
pub const SIGUSR2   : kty::c_int = 12;
pub const SIGPIPE   : kty::c_int = 13;
pub const SIGALRM   : kty::c_int = 14;
pub const SIGTERM   : kty::c_int = 15;
pub const SIGSTKFLT : kty::c_int = 16;
pub const SIGCHLD   : kty::c_int = 17;
pub const SIGCONT   : kty::c_int = 18;
pub const SIGSTOP   : kty::c_int = 19;
pub const SIGTSTP   : kty::c_int = 20;
pub const SIGTTIN   : kty::c_int = 21;
pub const SIGTTOU   : kty::c_int = 22;
pub const SIGURG    : kty::c_int = 23;
pub const SIGXCPU   : kty::c_int = 24;
pub const SIGXFSZ   : kty::c_int = 25;
pub const SIGVTALRM : kty::c_int = 26;
pub const SIGPROF   : kty::c_int = 27;
pub const SIGWINCH  : kty::c_int = 28;
pub const SIGIO     : kty::c_int = 29;
pub const SIGPOLL   : kty::c_int = kty::SIGIO;
pub const SIGPWR    : kty::c_int = 30;
pub const SIGSYS    : kty::c_int = 31;
pub const SIGUNUSED : kty::c_int = 31;

pub const SA_NOCLDSTOP : kty::c_int = 0x00000001;
pub const SA_NOCLDWAIT : kty::c_int = 0x00000002;
pub const SA_SIGINFO   : kty::c_int = 0x00000004;
pub const SA_ONSTACK   : kty::c_int = 0x08000000;
pub const SA_RESTART   : kty::c_int = 0x10000000;
pub const SA_NODEFER   : kty::c_int = 0x40000000;
#[allow(overflowing_literals)]
pub const SA_RESETHAND : kty::c_int = 0x80000000;
pub const SA_NOMASK    : kty::c_int = kty::SA_NODEFER;
pub const SA_ONESHOT   : kty::c_int = kty::SA_RESETHAND;

pub const MINSIGSTKSZ : usize = 2048;
pub const SIGSTKSZ    : usize = 8192;

pub type SigsetVal = kty::c_ulong;

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct sigset_t {
    pub sig: [kty::c_ulong; kty::_NSIG_WORDS], // c_ulong for compat
}

pub type old_sigset_t = kty::c_ulong;

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct sigaltstack {
    pub ss_sp: *mut kty::c_void,
    pub ss_flags: kty::c_int,
    pub ss_size: kty::user_size_t,
}

pub type stack_t = kty::sigaltstack;

/////////////////////////
// include/linux/signal.h
/////////////////////////

// For some reason the uapi exposes another type in uapi/asm-generic/signal.h. But this is
// the type expected is syscalls.

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct sigaction {
    pub sa_handler:  usize,
    pub sa_flags:    kty::c_ulong, // c_ulong because compat
    pub sa_restorer: usize,
    pub sa_mask:     kty::sigset_t,
}

////////////////////////////////////
// include/uapi/asm-generic/socket.h
////////////////////////////////////

pub const SOL_SOCKET                       : kty::c_int = 1;
pub const SO_DEBUG                         : kty::c_int = 1;
pub const SO_REUSEADDR                     : kty::c_int = 2;
pub const SO_TYPE                          : kty::c_int = 3;
pub const SO_ERROR                         : kty::c_int = 4;
pub const SO_DONTROUTE                     : kty::c_int = 5;
pub const SO_BROADCAST                     : kty::c_int = 6;
pub const SO_SNDBUF                        : kty::c_int = 7;
pub const SO_RCVBUF                        : kty::c_int = 8;
pub const SO_SNDBUFFORCE                   : kty::c_int = 32;
pub const SO_RCVBUFFORCE                   : kty::c_int = 33;
pub const SO_KEEPALIVE                     : kty::c_int = 9;
pub const SO_OOBINLINE                     : kty::c_int = 10;
pub const SO_NO_CHECK                      : kty::c_int = 11;
pub const SO_PRIORITY                      : kty::c_int = 12;
pub const SO_LINGER                        : kty::c_int = 13;
pub const SO_BSDCOMPAT                     : kty::c_int = 14;
pub const SO_REUSEPORT                     : kty::c_int = 15;
pub const SO_PASSCRED                      : kty::c_int = 16;
pub const SO_PEERCRED                      : kty::c_int = 17;
pub const SO_RCVLOWAT                      : kty::c_int = 18;
pub const SO_SNDLOWAT                      : kty::c_int = 19;
pub const SO_RCVTIMEO                      : kty::c_int = 20;
pub const SO_SNDTIMEO                      : kty::c_int = 21;
pub const SO_SECURITY_AUTHENTICATION       : kty::c_int = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT : kty::c_int = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK   : kty::c_int = 24;
pub const SO_BINDTODEVICE                  : kty::c_int = 25;
pub const SO_ATTACH_FILTER                 : kty::c_int = 26;
pub const SO_DETACH_FILTER                 : kty::c_int = 27;
pub const SO_GET_FILTER                    : kty::c_int = kty::SO_ATTACH_FILTER;
pub const SO_PEERNAME                      : kty::c_int = 28;
pub const SO_TIMESTAMP                     : kty::c_int = 29;
pub const SCM_TIMESTAMP                    : kty::c_int = kty::SO_TIMESTAMP;
pub const SO_ACCEPTCONN                    : kty::c_int = 30;
pub const SO_PEERSEC                       : kty::c_int = 31;
pub const SO_PASSSEC                       : kty::c_int = 34;
pub const SO_TIMESTAMPNS                   : kty::c_int = 35;
pub const SCM_TIMESTAMPNS                  : kty::c_int = kty::SO_TIMESTAMPNS;
pub const SO_MARK                          : kty::c_int = 36;
pub const SO_TIMESTAMPING                  : kty::c_int = 37;
pub const SCM_TIMESTAMPING                 : kty::c_int = kty::SO_TIMESTAMPING;
pub const SO_PROTOCOL                      : kty::c_int = 38;
pub const SO_DOMAIN                        : kty::c_int = 39;
pub const SO_RXQ_OVFL                      : kty::c_int = 40;
pub const SO_WIFI_STATUS                   : kty::c_int = 41;
pub const SCM_WIFI_STATUS                  : kty::c_int = kty::SO_WIFI_STATUS;
pub const SO_PEEK_OFF                      : kty::c_int = 42;
pub const SO_NOFCS                         : kty::c_int = 43;
pub const SO_LOCK_FILTER                   : kty::c_int = 44;
pub const SO_SELECT_ERR_QUEUE              : kty::c_int = 45;
pub const SO_BUSY_POLL                     : kty::c_int = 46;
pub const SO_MAX_PACING_RATE               : kty::c_int = 47;
pub const SO_BPF_EXTENSIONS                : kty::c_int = 48;

/////////////////////////////////////
// include/uapi/asm-generic/sockios.h
/////////////////////////////////////

pub const FIOSETOWN    : kty::c_int = 0x8901;
pub const SIOCSPGRP    : kty::c_int = 0x8902;
pub const FIOGETOWN    : kty::c_int = 0x8903;
pub const SIOCGPGRP    : kty::c_int = 0x8904;
pub const SIOCATMARK   : kty::c_int = 0x8905;
pub const SIOCGSTAMP   : kty::c_int = 0x8906;
pub const SIOCGSTAMPNS : kty::c_int = 0x8907;

////////////////////////////////////
// include/uapi/asm-generic/statfs.h
////////////////////////////////////

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct statfs {
    pub f_type:    kty::__statfs_word,
    pub f_bsize:   kty::__statfs_word,
    pub f_blocks:  kty::__statfs_word,
    pub f_bfree:   kty::__statfs_word,
    pub f_bavail:  kty::__statfs_word,
    pub f_files:   kty::__statfs_word,
    pub f_ffree:   kty::__statfs_word,
    pub f_fsid:    kty::__kernel_fsid_t,
    pub f_namelen: kty::__statfs_word,
    pub f_frsize:  kty::__statfs_word,
    pub f_flags:   kty::__statfs_word,
    pub f_spare:   [kty::__statfs_word; 4],
}

//////////////////////////////////
// include/uapi/asm-generic/stat.h
//////////////////////////////////

pub const STAT_HAVE_NSEC: kty::c_int = 1;

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct stat {
    pub st_dev:        kty::c_ulong,
    pub st_ino:        kty::c_ulong,
    pub st_mode:       kty::c_uint,
    pub st_nlink:      kty::c_uint,
    pub st_uid:        kty::c_uint,
    pub st_gid:        kty::c_uint,
    pub st_rdev:       kty::c_ulong,
    pub __pad1:        kty::c_ulong,
    pub st_size:       kty::c_long,
    pub st_blksize:    kty::c_int,
    pub __pad2:        kty::c_int,
    pub st_blocks:     kty::c_long,
    pub st_atime:      kty::c_long,
    pub st_atime_nsec: kty::c_ulong,
    pub st_mtime:      kty::c_long,
    pub st_mtime_nsec: kty::c_ulong,
    pub st_ctime:      kty::c_long,
    pub st_ctime_nsec: kty::c_ulong,
    pub __unused4:     kty::c_uint,
    pub __unused5:     kty::c_uint,
}

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct stat64 {
    pub st_dev:        kty::c_ulonglong,
    pub st_ino:        kty::c_ulonglong,
    pub st_mode:       kty::c_uint,
    pub st_nlink:      kty::c_uint,
    pub st_uid:        kty::c_uint,
    pub st_gid:        kty::c_uint,
    pub st_rdev:       kty::c_ulonglong,
    pub __pad1:        kty::c_ulonglong,
    pub st_size:       kty::c_longlong,
    pub st_blksize:    kty::c_int,
    pub __pad2:        kty::c_int,
    pub st_blocks:     kty::c_longlong,
    pub st_atime:      kty::c_int,
    pub st_atime_nsec: kty::c_uint,
    pub st_mtime:      kty::c_int,
    pub st_mtime_nsec: kty::c_uint,
    pub st_ctime:      kty::c_int,
    pub st_ctime_nsec: kty::c_uint,
    pub __unused4:     kty::c_uint,
    pub __unused5:     kty::c_uint,
}

//////////////////////////////////////
// include/uapi/asm-generic/termbits.h
//////////////////////////////////////

pub type cc_t     = kty::c_uchar;
pub type speed_t  = kty::c_uint;
pub type tcflag_t = kty::c_uint;

pub const NCCS : usize = 19;

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct termios {
    pub c_iflag:    kty::tcflag_t,
    pub c_oflag:    kty::tcflag_t,
    pub c_cflag:    kty::tcflag_t,
    pub c_lflag:    kty::tcflag_t,
    pub c_line:     kty::cc_t,
    pub c_cc: [kty::cc_t; kty::NCCS],
}

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct termios2 {
    pub c_iflag:    kty::tcflag_t,
    pub c_oflag:    kty::tcflag_t,
    pub c_cflag:    kty::tcflag_t,
    pub c_lflag:    kty::tcflag_t,
    pub c_line:     kty::cc_t,
    pub c_cc: [kty::cc_t; kty::NCCS],
    pub c_ispeed:   kty::speed_t,
    pub c_ospeed:   kty::speed_t,
}

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct ktermios {
    pub c_iflag:    kty::tcflag_t,
    pub c_oflag:    kty::tcflag_t,
    pub c_cflag:    kty::tcflag_t,
    pub c_lflag:    kty::tcflag_t,
    pub c_line:     kty::cc_t,
    pub c_cc: [kty::cc_t; kty::NCCS],
    pub c_ispeed:   kty::speed_t,
    pub c_ospeed:   kty::speed_t,
}

pub const VINTR    : kty::cc_t = 0;
pub const VQUIT    : kty::cc_t = 1;
pub const VERASE   : kty::cc_t = 2;
pub const VKILL    : kty::cc_t = 3;
pub const VEOF     : kty::cc_t = 4;
pub const VTIME    : kty::cc_t = 5;
pub const VMIN     : kty::cc_t = 6;
pub const VSWTC    : kty::cc_t = 7;
pub const VSTART   : kty::cc_t = 8;
pub const VSTOP    : kty::cc_t = 9;
pub const VSUSP    : kty::cc_t = 10;
pub const VEOL     : kty::cc_t = 11;
pub const VREPRINT : kty::cc_t = 12;
pub const VDISCARD : kty::cc_t = 13;
pub const VWERASE  : kty::cc_t = 14;
pub const VLNEXT   : kty::cc_t = 15;
pub const VEOL2    : kty::cc_t = 16;

pub const IGNBRK  : kty::tcflag_t = 0o000001;
pub const BRKINT  : kty::tcflag_t = 0o000002;
pub const IGNPAR  : kty::tcflag_t = 0o000004;
pub const PARMRK  : kty::tcflag_t = 0o000010;
pub const INPCK   : kty::tcflag_t = 0o000020;
pub const ISTRIP  : kty::tcflag_t = 0o000040;
pub const INLCR   : kty::tcflag_t = 0o000100;
pub const IGNCR   : kty::tcflag_t = 0o000200;
pub const ICRNL   : kty::tcflag_t = 0o000400;
pub const IUCLC   : kty::tcflag_t = 0o001000;
pub const IXON    : kty::tcflag_t = 0o002000;
pub const IXANY   : kty::tcflag_t = 0o004000;
pub const IXOFF   : kty::tcflag_t = 0o010000;
pub const IMAXBEL : kty::tcflag_t = 0o020000;
pub const IUTF8   : kty::tcflag_t = 0o040000;

pub const OPOST  : kty::tcflag_t = 0o000001;
pub const OLCUC  : kty::tcflag_t = 0o000002;
pub const ONLCR  : kty::tcflag_t = 0o000004;
pub const OCRNL  : kty::tcflag_t = 0o000010;
pub const ONOCR  : kty::tcflag_t = 0o000020;
pub const ONLRET : kty::tcflag_t = 0o000040;
pub const OFILL  : kty::tcflag_t = 0o000100;
pub const OFDEL  : kty::tcflag_t = 0o000200;
pub const NLDLY  : kty::tcflag_t = 0o000400;
pub const NL0    : kty::tcflag_t = 0o000000;
pub const NL1    : kty::tcflag_t = 0o000400;
pub const CRDLY  : kty::tcflag_t = 0o003000;
pub const CR0    : kty::tcflag_t = 0o000000;
pub const CR1    : kty::tcflag_t = 0o001000;
pub const CR2    : kty::tcflag_t = 0o002000;
pub const CR3    : kty::tcflag_t = 0o003000;
pub const TABDLY : kty::tcflag_t = 0o014000;
pub const TAB0   : kty::tcflag_t = 0o000000;
pub const TAB1   : kty::tcflag_t = 0o004000;
pub const TAB2   : kty::tcflag_t = 0o010000;
pub const TAB3   : kty::tcflag_t = 0o014000;
pub const XTABS  : kty::tcflag_t = 0o014000;
pub const BSDLY  : kty::tcflag_t = 0o020000;
pub const BS0    : kty::tcflag_t = 0o000000;
pub const BS1    : kty::tcflag_t = 0o020000;
pub const VTDLY  : kty::tcflag_t = 0o040000;
pub const VT0    : kty::tcflag_t = 0o000000;
pub const VT1    : kty::tcflag_t = 0o040000;
pub const FFDLY  : kty::tcflag_t = 0o100000;
pub const FF0    : kty::tcflag_t = 0o000000;
pub const FF1    : kty::tcflag_t = 0o100000;

pub const CBAUD    : kty::tcflag_t = 0o010017;
pub const B0       : kty::tcflag_t = 0o000000;
pub const B50      : kty::tcflag_t = 0o000001;
pub const B75      : kty::tcflag_t = 0o000002;
pub const B110     : kty::tcflag_t = 0o000003;
pub const B134     : kty::tcflag_t = 0o000004;
pub const B150     : kty::tcflag_t = 0o000005;
pub const B200     : kty::tcflag_t = 0o000006;
pub const B300     : kty::tcflag_t = 0o000007;
pub const B600     : kty::tcflag_t = 0o000010;
pub const B1200    : kty::tcflag_t = 0o000011;
pub const B1800    : kty::tcflag_t = 0o000012;
pub const B2400    : kty::tcflag_t = 0o000013;
pub const B4800    : kty::tcflag_t = 0o000014;
pub const B9600    : kty::tcflag_t = 0o000015;
pub const B19200   : kty::tcflag_t = 0o000016;
pub const B38400   : kty::tcflag_t = 0o000017;
pub const EXTA     : kty::tcflag_t = kty::B19200;
pub const EXTB     : kty::tcflag_t = kty::B38400;
pub const CSIZE    : kty::tcflag_t = 0o000060;
pub const CS5      : kty::tcflag_t = 0o000000;
pub const CS6      : kty::tcflag_t = 0o000020;
pub const CS7      : kty::tcflag_t = 0o000040;
pub const CS8      : kty::tcflag_t = 0o000060;
pub const CSTOPB   : kty::tcflag_t = 0o000100;
pub const CREAD    : kty::tcflag_t = 0o000200;
pub const PARENB   : kty::tcflag_t = 0o000400;
pub const PARODD   : kty::tcflag_t = 0o001000;
pub const HUPCL    : kty::tcflag_t = 0o002000;
pub const CLOCAL   : kty::tcflag_t = 0o004000;
pub const CBAUDEX  : kty::tcflag_t = 0o010000;
pub const BOTHER   : kty::tcflag_t = 0o010000;
pub const B57600   : kty::tcflag_t = 0o010001;
pub const B115200  : kty::tcflag_t = 0o010002;
pub const B230400  : kty::tcflag_t = 0o010003;
pub const B460800  : kty::tcflag_t = 0o010004;
pub const B500000  : kty::tcflag_t = 0o010005;
pub const B576000  : kty::tcflag_t = 0o010006;
pub const B921600  : kty::tcflag_t = 0o010007;
pub const B1000000 : kty::tcflag_t = 0o010010;
pub const B1152000 : kty::tcflag_t = 0o010011;
pub const B1500000 : kty::tcflag_t = 0o010012;
pub const B2000000 : kty::tcflag_t = 0o010013;
pub const B2500000 : kty::tcflag_t = 0o010014;
pub const B3000000 : kty::tcflag_t = 0o010015;
pub const B3500000 : kty::tcflag_t = 0o010016;
pub const B4000000 : kty::tcflag_t = 0o010017;
pub const CIBAUD   : kty::tcflag_t = 0o02003600000;
pub const CMSPAR   : kty::tcflag_t = 0o10000000000;
pub const CRTSCTS  : kty::tcflag_t = 0o20000000000;

pub const IBSHIFT : kty::tcflag_t = 16;

pub const ISIG    : kty::tcflag_t = 0o000001;
pub const ICANON  : kty::tcflag_t = 0o000002;
pub const XCASE   : kty::tcflag_t = 0o000004;
pub const ECHO    : kty::tcflag_t = 0o000010;
pub const ECHOE   : kty::tcflag_t = 0o000020;
pub const ECHOK   : kty::tcflag_t = 0o000040;
pub const ECHONL  : kty::tcflag_t = 0o000100;
pub const NOFLSH  : kty::tcflag_t = 0o000200;
pub const TOSTOP  : kty::tcflag_t = 0o000400;
pub const ECHOCTL : kty::tcflag_t = 0o001000;
pub const ECHOPRT : kty::tcflag_t = 0o002000;
pub const ECHOKE  : kty::tcflag_t = 0o004000;
pub const FLUSHO  : kty::tcflag_t = 0o010000;
pub const PENDIN  : kty::tcflag_t = 0o040000;
pub const IEXTEN  : kty::tcflag_t = 0o100000;
pub const EXTPROC : kty::tcflag_t = 0o200000;

pub const TCOOFF : kty::c_uint = 0;
pub const TCOON  : kty::c_uint = 1;
pub const TCIOFF : kty::c_uint = 2;
pub const TCION  : kty::c_uint = 3;

pub const TCIFLUSH  : kty::c_uint = 0;
pub const TCOFLUSH  : kty::c_uint = 1;
pub const TCIOFLUSH : kty::c_uint = 2;

pub const TCSANOW   : kty::c_uint = 0;
pub const TCSADRAIN : kty::c_uint = 1;
pub const TCSAFLUSH : kty::c_uint = 2;

/////////////////////////////////////
// include/uapi/asm-generic/termios.h
/////////////////////////////////////

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct winsize {
    pub ws_row:    kty::c_ushort,
    pub ws_col:    kty::c_ushort,
    pub ws_xpixel: kty::c_ushort,
    pub ws_ypixel: kty::c_ushort,
}

pub const NCC : usize = 8;

#[repr(C)]
#[derive(Pod, Copy, Clone, Eq, PartialEq)]
pub struct termio {
    pub c_iflag:   kty::c_ushort,
    pub c_oflag:   kty::c_ushort,
    pub c_cflag:   kty::c_ushort,
    pub c_lflag:   kty::c_ushort,
    pub c_line:    kty::c_uchar,
    pub c_cc: [kty::c_uchar; kty::NCC],
}

pub const TIOCM_LE   : kty::c_uint = 0x001;
pub const TIOCM_DTR  : kty::c_uint = 0x002;
pub const TIOCM_RTS  : kty::c_uint = 0x004;
pub const TIOCM_ST   : kty::c_uint = 0x008;
pub const TIOCM_SR   : kty::c_uint = 0x010;
pub const TIOCM_CTS  : kty::c_uint = 0x020;
pub const TIOCM_CAR  : kty::c_uint = 0x040;
pub const TIOCM_RNG  : kty::c_uint = 0x080;
pub const TIOCM_DSR  : kty::c_uint = 0x100;
pub const TIOCM_CD   : kty::c_uint = kty::TIOCM_CAR;
pub const TIOCM_RI   : kty::c_uint = kty::TIOCM_RNG;
pub const TIOCM_OUT1 : kty::c_uint = 0x2000;
pub const TIOCM_OUT2 : kty::c_uint = 0x4000;
pub const TIOCM_LOOP : kty::c_uint = 0x8000;

//////////////////////////////////////
// include/uapi/asm-generic/ucontext.h
//////////////////////////////////////

// #[repr(C)]
// #[derive(Pod, Copy, Clone, Eq, PartialEq)]
// pub struct ucontext {
//     pub uc_flags:    kty::c_ulong,
//     pub uc_link:     *mut kty::ucontext,
//     pub uc_stack:    kty::stack_t,
//     pub uc_mcontext: kty::sigcontext,
//     pub uc_sigmask:  kty::sigset_t,
// }
