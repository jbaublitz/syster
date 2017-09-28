use nix::Errno;

mod sys0;
pub use self::sys0::*;

mod sys1;
pub use self::sys1::*;

mod sys2;
pub use self::sys2::*;

pub enum SyscallArg {
    Int(i64),
    Fd(u64),
    Id(u64),
    Ptr(*const [u8]),
    MutPtr(*mut [u8]),
}

#[derive(Debug,PartialEq)]
pub enum SyscallRet {
    Ret(i64),
    Fd(u64),
    Id(u64),
    Ptr(*const [u8]),
    MutPtr(*mut [u8]),
    Success,
    Zero,
    Err(Errno),
}

macro_rules! syscall_zero {
    ( $name:ident => ($val:tt) { $call_impl:expr } ) => (
        pub struct $name;

        impl SyscallZeroArgs for $name {
            fn numval() -> u64 {
                $val
            }

            fn call() -> SyscallRet {
                $call_impl
            }
        }
    );
}

syscall_zero!(SchedYield => (24) {
    match SchedYield::raw_call() {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Pause => (34) {
    match Pause::raw_call() {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Getpid => (39) {
    match Getpid::raw_call() {
        i if i > 0 => SyscallRet::Id(i as u64),
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Fork => (57) {
    match Fork::raw_call() {
        i if i > 0 => SyscallRet::Id(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Vfork => (58) {
    match Vfork::raw_call() {
        i if i > 0 => SyscallRet::Id(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Getuid => (102) {
    match Getuid::raw_call() {
        i if i > 0 => SyscallRet::Id(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Getgid => (104) {
    match Getgid::raw_call() {
        i if i > 0 => SyscallRet::Id(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Geteuid => (107) {
    match Geteuid::raw_call() {
        i if i > 0 => SyscallRet::Id(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Getegid => (108) {
    match Getegid::raw_call() {
        i if i > 0 => SyscallRet::Id(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Getppid => (110) {
    match Getppid::raw_call() {
        i if i > 0 => SyscallRet::Id(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Getpgrp => (111) {
    match Getpgrp::raw_call() {
        i if i > 0 => SyscallRet::Id(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Setsid => (112) {
    match Setsid::raw_call() {
        i if i > 0 => SyscallRet::Id(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Munlockall => (152) {
    match Munlockall::raw_call() {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Vhangup => (153) {
    match Vhangup::raw_call() {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Sync => (162) {
    match Sync::raw_call() {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    }
});
syscall_zero!(Gettid => (186) {
    SyscallRet::Id(Gettid::raw_call() as u64)
});
syscall_zero!(RestartSyscall => (219) {
    SyscallRet::Ret(RestartSyscall::raw_call())
});
syscall_zero!(InotifyInit => (253) {
    SyscallRet::Fd(InotifyInit::raw_call() as u64)
});

macro_rules! syscall_one {
    ( $name:ident => ($val:tt) { $( $call_pat:pat => $call_expr:expr ),* } ) => (
        pub struct $name;

        impl SyscallOneArg for $name {
            fn numval() -> u64 {
                $val
            }

            fn call(arg0: SyscallArg) -> SyscallRet {
                match arg0 {
                    $( $call_pat => $call_expr, )*
                }
            }
        }
    );
}

syscall_one!(Close => (3) {
    SyscallArg::Fd(fd) => match Close::raw_call(fd) {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});
syscall_one!(Brk => (12) {
    SyscallArg::Ptr(p) => match Brk::raw_call(p as *const u8 as u64) {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});
//RtSigreturn => (15, Sys
syscall_one!(Pipe => (22) {
    SyscallArg::Ptr(sp) => match Pipe::raw_call(sp as *const u8 as u64) {
        i if i > 0 => SyscallRet::Fd(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});
syscall_one!(Dup => (32) {
    SyscallArg::Fd(fd) => match Dup::raw_call(fd) {
        i if i > 0 => SyscallRet::Fd(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});
syscall_one!(Alarm => (37) {
    SyscallArg::Int(num) => match Alarm::raw_call(num as u64) {
        i if i > 0 => SyscallRet::Ret(i),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});
syscall_one!(Exit => (60) {
    SyscallArg::Int(ecode) => match Exit::raw_call(ecode as u64) {
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});
syscall_one!(Uname => (63) {
    SyscallArg::MutPtr(p) => match Uname::raw_call(p as *mut u8 as u64) {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});
syscall_one!(Shmdt => (67) {
    SyscallArg::Ptr(p) => match Shmdt::raw_call(p as *const u8 as u64) {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});
syscall_one!(Fsync => (74) {
    SyscallArg::Ptr(p) => match Shmdt::raw_call(p as *const u8 as u64) {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});

macro_rules! syscall_two {
    ( $name:ident => ($val:tt) { $( $call_pat:pat => $call_expr:expr ),* },
      { $( $call_pat:pat => $call_expr:expr ),* } ) => (
        pub struct $name;

        impl SyscallTwoArgs for $name {
            fn numval() -> u64 {
                $val
            }

            fn call(arg0: SyscallArg, arg1: SyscallTwoArgs) -> SyscallRet {
                match (arg0, arg1) {
                    $( $call_pat => $call_expr, )*
                }
            }
        }
    );
}

#[cfg(test)]
mod test {
    extern crate libc;
    use super::*;

    #[test]
    fn test_asm_no_params() {
        let pid = Getpid::call();
        let libc_pid = unsafe { libc::getpid() };
        assert_eq!(pid, SyscallRet::Id(libc_pid as u64));

        let gid = Getgid::call();
        let libc_gid = unsafe { libc::getgid() };
        assert_eq!(gid, SyscallRet::Id(libc_gid as u64));
    }
}
