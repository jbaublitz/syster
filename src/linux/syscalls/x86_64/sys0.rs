use super::*;

macro_rules! zero {
    ( $syscall:ident, $num:tt, $ty:ty ) => {
        pub struct $syscall;

        impl Syscall for $syscall {
            #[inline]
            fn num() -> SyscallVal {
                $num
            }
        }

        impl ZeroArgs for $syscall {
            type Return = $ty;
        }
    }
}

pub trait ZeroArgs: Syscall + Sized {
    type Return: From<i64>;

    fn call(self) -> Self::Return {
        let val: u64 = Self::num();
        let rval: i64;
        unsafe {
            asm!("syscall"
                 :"=A"(rval)
                 :"{rax}"(val)
                 :"rax"
                 :"volatile"
             )
        };
        Self::Return::from(rval)
    }
}

zero!(SchedYield, 24, Zero);
zero!(Pause, 34, Error);
zero!(Getpid, 39, Pid);
zero!(Fork, 57, Pid);
zero!(Vfork, 58, Pid);
zero!(Getuid, 102, Id);
zero!(Getgid, 104, Id);
zero!(Geteuid, 107, Id);
zero!(Getegid, 108, Id);
zero!(Getppid, 110, Pid);
zero!(Getpgrp, 111, Id);
zero!(Setsid, 112, Id);
zero!(Munlockall, 152, Zero);
zero!(Vhangup, 153, Zero);
zero!(Sync, 162, Zero);
zero!(Gettid, 186, Id);
zero!(InotifyInit, 253, Fd);
