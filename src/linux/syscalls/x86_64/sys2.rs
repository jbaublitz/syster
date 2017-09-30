use linux::syscalls::x86_64::{SyscallArg,SyscallRet};

pub trait SyscallTwoArgs {
    fn raw_call<A, B>(arg0: A, arg1: B) -> i64
            where A: Into<u64>, B: Into<u64> {
        let val: u64 = Self::numval();
        let intoarg0: u64 = arg0.into();
        let intoarg1: u64 = arg1.into();
        let rval: i64;
        unsafe {
            asm!("syscall"
                 :"=A"(rval)
                 :"{rax}"(val),"{rdi}"(intoarg0),"{rsi}"(intoarg1)
                 :"rax","rdi","rsi"
                 :"volatile"
             )
        };
        rval
    }

    fn numval() -> u64;
    fn call(SyscallArg, SyscallArg) -> SyscallRet;
}
