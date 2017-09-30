mod sys0;
pub use self::sys0::*;

mod sys1;
pub use self::sys1::*;

mod sys2;
pub use self::sys2::*;

pub enum SyscallArg<'a> {
    Int(i64),
    Fd(u64),
    Id(u64),
    Ptr(&'a [u8]),
    StrPtr(&'a str),
    MutPtr(&'a mut [u8]),
}

impl<'a> Into<u64> for SyscallArg<'a> {
    fn into(self) -> u64 {
        match self {
            SyscallArg::Int(i) => i as u64,
            SyscallArg::Fd(f) => f,
            SyscallArg::Id(i) => i,
            SyscallArg::Ptr(p) => p.as_ptr() as u64,
            SyscallArg::StrPtr(p) => p.as_ptr() as u64,
            SyscallArg::MutPtr(mp) => mp.as_mut_ptr() as u64,
        }
    }
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
    Err(i64),
}

macro_rules! syscall_zero {
    ( $name:ident => ($val:tt) {
        $( $call_pat:pat if $call_expr_in:expr => $call_expr_out:expr ),*
    } ) => (
        pub struct $name;

        impl SyscallZeroArgs for $name {
            fn numval() -> u64 {
                $val
            }

            fn call() -> SyscallRet {
                match $name::raw_call() {
                    $( $call_pat if $call_expr_in => $call_expr_out, )*
                    i if i < 0 => SyscallRet::Err(i),
                    _ => unimplemented!(),
                }
            }
        }
    );
}

syscall_zero!(SchedYield => (24) {
    i if i == 0 => SyscallRet::Success
});
syscall_zero!(Pause => (34) {
    i if i == 0 => SyscallRet::Success
});
syscall_zero!(Getpid => (39) {
    i if i > 0 => SyscallRet::Id(i as u64)
});
syscall_zero!(Fork => (57) {
    i if i > 0 => SyscallRet::Id(i as u64),
    i if i == 0 => SyscallRet::Zero
});
syscall_zero!(Vfork => (58) {
    i if i > 0 => SyscallRet::Id(i as u64),
    i if i == 0 => SyscallRet::Zero
});
syscall_zero!(Getuid => (102) {
    i if i > 0 => SyscallRet::Id(i as u64),
    i if i == 0 => SyscallRet::Zero
});
syscall_zero!(Getgid => (104) {
    i if i > 0 => SyscallRet::Id(i as u64),
    i if i == 0 => SyscallRet::Zero
});
syscall_zero!(Geteuid => (107) {
    i if i > 0 => SyscallRet::Id(i as u64),
    i if i == 0 => SyscallRet::Zero
});
syscall_zero!(Getegid => (108) {
    i if i > 0 => SyscallRet::Id(i as u64),
    i if i == 0 => SyscallRet::Zero
});
syscall_zero!(Getppid => (110) {
    i if i > 0 => SyscallRet::Id(i as u64),
    i if i == 0 => SyscallRet::Zero
});
syscall_zero!(Getpgrp => (111) {
    i if i > 0 => SyscallRet::Id(i as u64),
    i if i == 0 => SyscallRet::Zero
});
syscall_zero!(Setsid => (112) {
    i if i > 0 => SyscallRet::Id(i as u64),
    i if i == 0 => SyscallRet::Zero
});
syscall_zero!(Munlockall => (152) {
    i if i == 0 => SyscallRet::Success
});
syscall_zero!(Vhangup => (153) {
    i if i == 0 => SyscallRet::Success
});
syscall_zero!(Sync => (162) {
    i if i == 0 => SyscallRet::Success
});
syscall_zero!(Gettid => (186) {
    i if i >= 0 => SyscallRet::Id(i as u64)
});
syscall_zero!(RestartSyscall => (219) {
    i if i >= 0 => SyscallRet::Ret(i)
});
syscall_zero!(InotifyInit => (253) {
    i if i >= 0 => SyscallRet::Fd(i as u64)
});

macro_rules! syscall_one {
    ( $name:ident => ($val:tt) {
        $( $call_pat:pat if $call_expr_in:expr => $call_expr_out:expr ),*
    } ) => (
        pub struct $name;

        impl SyscallOneArg for $name {
            fn numval() -> u64 {
                $val
            }

            fn call(arg0: SyscallArg) -> SyscallRet {
                match $name::raw_call(arg0) {
                    $( $call_pat if $call_expr_in => $call_expr_out, )*
                    i if i < 0 => SyscallRet::Err(i),
                    _ => unimplemented!(),
                }
            }
        }
    );
}

syscall_one!(Close => (3) {
    i if i == 0 => SyscallRet::Success
});
syscall_one!(Brk => (12) {
    i if i == 0 => SyscallRet::Success
});
//RtSigreturn => (15, Sys
syscall_one!(Pipe => (22) {
    i if i > 0 => SyscallRet::Fd(i as u64),
    i if i == 0 => SyscallRet::Zero
});
syscall_one!(Dup => (32) {
    i if i > 0 => SyscallRet::Fd(i as u64),
    i if i == 0 => SyscallRet::Zero
});
syscall_one!(Alarm => (37) {
    i if i > 0 => SyscallRet::Ret(i),
    i if i == 0 => SyscallRet::Zero
});
syscall_one!(Exit => (60) {});
syscall_one!(Uname => (63) {
    i if i == 0 => SyscallRet::Success
});
syscall_one!(Shmdt => (67) {
    i if i == 0 => SyscallRet::Success
});
syscall_one!(Fsync => (74) {
    i if i == 0 => SyscallRet::Success
});
syscall_one!(Fdatasync => (75) {
    i if i == 0 => SyscallRet::Success
});
syscall_one!(Chdir => (80) {
    i if i == 0 => SyscallRet::Success
});
syscall_one!(Fchdir => (81) {
    i if i == 0 => SyscallRet::Success
});
syscall_one!(Rmdir => (84) {
    i if i == 0 => SyscallRet::Success
});
syscall_one!(Unlink => (87) {
    i if i == 0 => SyscallRet::Success
});
syscall_one!(Umask => (95) {
    i if i >= 0 => SyscallRet::Id(i as u64)
});
syscall_one!(Sysinfo => (99) {
    i if i >= 0 => SyscallRet::Success
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

    #[test]
    fn test_asm_one_param() {
        assert_eq!(match Fork::call() {
            SyscallRet::Zero => Exit::call(SyscallArg::Id(0)),
            SyscallRet::Id(_) => SyscallRet::Success,
            _ => panic!(),
        }, SyscallRet::Success)
    }
}
