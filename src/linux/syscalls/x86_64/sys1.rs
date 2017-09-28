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
