use super::{Arg,Ret};

pub trait SyscallTwoArgs<A: Arg, B: Arg, R: Arg> {
    fn asm(arg0: u64, arg1: u64) -> i64 {
        let val: u64 = Self::numval();
        let rval: i64;
        unsafe {
            asm!("syscall"
                 :"=A"(rval)
                 :"{rax}"(val),"{rdi}"(arg0),"{rsi}"(arg1)
                 :"rax","rdi","rsi"
                 :"volatile"
             )
        };
        rval
    }

    fn numval() -> u64;
    fn call(A, B) -> Ret<R>;
}
