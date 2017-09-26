use nix::errno::Errno;

use linux::syscalls::x86_64::SyscallRet;

pub trait SyscallZeroArgs {
    fn raw_call() -> i64 {
        let val: u64 = Self::numval();
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

    fn numval() -> u64;
    fn call() -> SyscallRet;
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
