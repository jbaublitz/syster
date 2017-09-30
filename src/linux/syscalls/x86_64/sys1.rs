use linux::syscalls::x86_64::{SyscallArg,SyscallRet};

pub trait SyscallOneArg {
    fn raw_call<T: Into<u64>>(arg0: T) -> i64 {
        let val: u64 = Self::numval();
        let intoarg0: u64 = arg0.into();
        let rval: i64;
        unsafe {
            asm!("syscall"
                 :"=A"(rval)
                 :"{rax}"(val),"{rdi}"(intoarg0)
                 :"rax","rdi"
                 :"volatile"
             )
        };
        rval
    }

    fn numval() -> u64;
    fn call(SyscallArg) -> SyscallRet;
}
