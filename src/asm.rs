macro_rules! syscall_zero {
    ( $( $var:ident => $val:tt ),* ) => {
        pub enum Syscall0 {
            $( $var, )*
        }

        impl Syscall0 {
            pub fn call(self) -> usize {
                let val: usize = self.into();
                let rval: usize;
                unsafe {
                    asm!("movq $0, %rax
                          syscall"
                         :"=A"(rval)
                         :"r"(val)
                         :"rax"
                         :"volatile"
                     )
                };
                rval
            }
        }

        impl From<Syscall0> for usize {
            fn from(v: Syscall0) -> Self {
                match v {
                    $( Syscall0::$var => $val, )*
                }
            }
        }
    }
}

syscall_zero!(
    SchedYield => 24,
    Pause => 34,
    Getpid => 39
);

#[cfg(test)]
mod test {
    extern crate libc;
    use super::*;

    #[test]
    fn test_asm_no_params() {
        let pid = Syscall0::Getpid.call();
        let libc_pid = unsafe { libc::getpid() };
        assert_eq!(pid as i32, libc_pid);
    }
}
