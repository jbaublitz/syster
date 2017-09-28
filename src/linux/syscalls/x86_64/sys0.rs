use linux::syscalls::x86_64::SyscallRet;

pub trait SyscallZeroArgs {
    fn raw_call() -> i64 {
        let val: u64 = Self::numval();
        let rval: i64;
        unsafe {
            asm!("syscall"
                 :"=A"(rval)
                 :"{rax}"(val)
                 :"rax"
                 :"volatile"
             )
        };
        rval
    }

    fn numval() -> u64;
    fn call() -> SyscallRet;
}
