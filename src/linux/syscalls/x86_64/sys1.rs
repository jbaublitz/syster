use super::*;

macro_rules! one {
    ( $syscall:ident, $num:tt, $arg0ty: ty, $ty:ty ) => {
        pub struct $syscall;

        impl Syscall for $syscall {
            #[inline]
            fn num() -> SyscallVal {
                $num
            }
        }

        impl OneArg for $syscall {
            type Arg0 = $arg0ty;
            type Return = $ty;
        }
    }
}

pub trait OneArg: Syscall + Sized {
    type Arg0: Into<u64>;
    type Return: From<i64>;

    fn call(self, arg0: Self::Arg0) -> Self::Return {
        let val: u64 = Self::num();
        let arg0i64 = arg0.into();
        let rval: i64;
        unsafe {
            asm!("syscall"
                 :"=A"(rval)
                 :"{rax}"(val), "{rdi}"(arg0i64)
                 :"rax"
                 :"volatile"
             )
        };
        Self::Return::from(rval)
    }
}

one!(Close, 3, Fd, Zero);
one!(Brk, 12, Ptr, Zero);
one!(Pipe, 22, Ptr, Zero);
one!(Dup, 32, Fd, Fd);
one!(Alarm, 37, Int, Int);
one!(Exit, 60, Int, Never);
one!(Uname, 63, MutPtr, Zero);
one!(Shmdt, 67, Ptr, Zero);
one!(Fsync, 74, Fd, Zero);
one!(Fdatasync, 75, Fd, Zero);
