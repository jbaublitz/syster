use nix::errno::Errno;

pub enum SyscallArg<T=()> {
    Fd(u64),
    Id(u64),
    Ptr(*const T),
    MutPtr(*mut T),
    StrPtr(*const str),
}

#[derive(Debug,PartialEq)]
pub enum SyscallRet<T=()> {
    Ret(i64),
    Fd(u64),
    Id(u64),
    Ptr(*const T),
    MutPtr(*mut T),
    StrPtr(*const str),
    Success,
    Zero,
    Err(Errno),
}

pub trait SyscallZeroArgs {
    fn raw_call() -> i64 {
        let val: u64 = Self::into();
        let rval: i64;
        unsafe {
            asm!("movq $0, %rax
                  syscall"
                 :"=A"(rval)
                 :"r"(val)
                 :"rax"
                 :"volatile"
             )
        };
        rval
    }

    fn into() -> u64;
    fn call() -> SyscallRet;
}

macro_rules! syscall_zero {
    ( $name:ident => ($val:tt) $call_impl:block ) => (
        pub struct $name;

        impl SyscallZeroArgs for $name {
            fn into() -> u64 {
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

pub trait SyscallOneArg {
    fn raw_call<T>(arg0: T) -> i64 {
        let val: u64 = Self::into();
        let rval: i64;
        unsafe {
            asm!("movq $0, %rdi
                  movq $1, %rax
                  syscall"
                 :"=A"(rval)
                 :"r"(arg0),"r"(val)
                 :"rax","rdi"
                 :"volatile"
             )
        };
        rval
    }

    fn into() -> u64;
    fn call(SyscallArg) -> SyscallRet;
}

macro_rules! syscall_one {
    ( $name:ident => ($val:tt) { $( $call_pat:pat => $call_expr:expr ),* } ) => (
        pub struct $name;

        impl SyscallOneArg for $name {
            fn into() -> u64 {
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
    SyscallArg::Fd(fd) => match Close::raw_call::<u64>(fd) {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});
syscall_one!(Brk => (12) {
    SyscallArg::Ptr(p) => match Brk::raw_call::<*const ()>(p) {
        i if i == 0 => SyscallRet::Success,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});
//RtSigreturn => (15, Sys
syscall_one!(Pipe => (22) {
    SyscallArg::StrPtr(sp) => match Pipe::raw_call::<*const str>(sp) {
        i if i > 0 => SyscallRet::Fd(i as u64),
        i if i == 0 => SyscallRet::Zero,
        i if i < 0 => SyscallRet::Err(Errno::last()),
        _ => panic!(),
    },
    _ => SyscallRet::Err(Errno::EINVAL)
});

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
