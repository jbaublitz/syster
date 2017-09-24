use num::FromPrimitive;
use nix::errno::Errno;

#[derive(Debug,PartialEq)]
pub enum SyscallRet<T=u8> {
    Ret(T),
    Zero,
    Err(Errno),
}

impl<T: FromPrimitive> From<i64> for SyscallRet<T> {
    fn from(v: i64) -> Self {
        match v {
            i if i > 0 => match FromPrimitive::from_i64(i) {
                Some(conv) => SyscallRet::Ret(conv),
                None => SyscallRet::Err(Errno::EINVAL),
            },
            i if i == 0 => SyscallRet::Zero,
            i if i < 0 => SyscallRet::Err(Errno::last()),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug,PartialEq)]
pub struct Success;

impl From<i64> for Success {
    fn from(_v: i64) -> Self {
        Success
    }
}

pub trait SyscallZeroArgs {
    type Return: From<i64>;

    fn raw_call() -> Self::Return {
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
        rval.into()
    }

    fn into() -> u64;
}

macro_rules! syscall_zero {
    ( $name:ident => ($val:tt, $ret:ty) $call_impl:block ) => (
        pub struct $name;

        impl SyscallZeroArgs for $name {
            type Return = $ret;

            fn raw_call() -> Self::Return {
                $call_impl
            }

            fn into() -> u64 {
                $val
            }
        }
    );
    ( $name:ident => ($val:tt, $ret:ty) ) => (
        pub struct $name;

        impl SyscallZeroArgs for $name {
            type Return = $ret;

            fn into() -> u64 {
                $val
            }
        }
    );
}

syscall_zero!(SchedYield => (24, SyscallRet));
syscall_zero!(Pause => (34, SyscallRet));
syscall_zero!(Getpid => (39, SyscallRet<u64>));
syscall_zero!(Fork => (57, SyscallRet<u64>));
syscall_zero!(Vfork => (58, SyscallRet<u64>));
syscall_zero!(Getuid => (102, SyscallRet<u64>));
syscall_zero!(Getgid => (104, SyscallRet<u64>));
syscall_zero!(Geteuid => (107, SyscallRet<u64>));
syscall_zero!(Getegid => (108, SyscallRet<u64>));
syscall_zero!(Getppid => (110, SyscallRet<u64>));
syscall_zero!(Getpgrp => (111, SyscallRet<u64>));
syscall_zero!(Setsid => (112, SyscallRet<u64>));
syscall_zero!(Munlockall => (152, SyscallRet));
syscall_zero!(Vhangup => (153, SyscallRet));
syscall_zero!(Sync => (162, SyscallRet));
syscall_zero!(Gettid => (186, Success));
syscall_zero!(RestartSyscall => (219, SyscallRet<u64>));
syscall_zero!(InotifyInit => (253, SyscallRet<u64>));

pub trait SyscallOneArg {
    type ArgOne;
    type Return: From<i64>;

    fn call(arg0: Self::ArgOne) -> Self::Return {
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
        rval.into()
    }

    fn into() -> u64;
}

macro_rules! syscall_one {
    ( $( $name:ident => ($val:tt, $ret:ty, { $argty:ty }), $call_impl:block ),* ) => (
    );
    ( $( $name:ident => ($val:tt, $ret:ty, { $argty:ty }) ),* ) => (
        $(
            pub struct $name;

            impl SyscallOneArg for $name {
                type ArgOne = $argty;
                type Return = $ret;

                fn into() -> u64 {
                    $val
                }
            }
        )*
    );
}

syscall_one!(
    Close => (3, SyscallRet, { u64 }),
    Brk => (12, SyscallRet, { *const u64 })
    //RtSigreturn => (15, Sys
);

#[cfg(test)]
mod test {
    extern crate libc;
    use super::*;

    #[test]
    fn test_asm_no_params() {
        let pid = Getpid::raw_call();
        let libc_pid = unsafe { libc::getpid() };
        assert_eq!(pid, SyscallRet::Ret(libc_pid as u64));

        let gid = Getgid::raw_call();
        let libc_gid = unsafe { libc::getgid() };
        assert_eq!(gid, SyscallRet::Ret(libc_gid as u64));
    }
}
