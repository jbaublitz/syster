use linux::syscalls::x86_64::{SyscallArg,SyscallRet};

pub trait SyscallTwoArgs {
    fn raw_call(arg0: u64, arg1: u64) -> i64 {
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
    fn call(SyscallArg, SyscallArg) -> SyscallRet;
}

macro_rules! syscall_two {
    ( $name:ident => ($val:tt) { $( $call_pat:pat => $call_expr:expr ),* },
      { $( $call_pat:pat => $call_expr:expr ),* } ) => (
        pub struct $name;

        impl SyscallTwoArgs for $name {
            fn numval() -> u64 {
                $val
            }

            fn call(arg0: SyscallArg, arg1: SyscallTwoArgs) -> SyscallRet {
                match (arg0, arg1) {
                    $( $call_pat => $call_expr, )*
                }
            }
        }
    );
}
