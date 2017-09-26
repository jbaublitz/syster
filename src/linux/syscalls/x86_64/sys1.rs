use nix::Errno;

use linux::syscalls::x86_64::{SyscallArg,SyscallRet};

pub trait SyscallOneArg {
    fn raw_call(arg0: u64) -> i64 {
        let val: u64 = Self::numval();
        let rval: i64;
        unsafe {
            asm!("syscall"
                 :"=A"(rval)
                 :"{rax}"(val),"{rdi}"(arg0)
                 :"rax","rdi"
                 :"volatile"
             )
        };
        rval
    }

    fn numval() -> u64;
    fn call(SyscallArg) -> SyscallRet;
}

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
