use super::super::super::int_width;
use super::*;

macro_rules! syscall1 {
    ( $name:ident ( $num:tt ) [ $( $impl_clause:tt )* ] [ $( $fn:tt )* ] ) => (
        pub struct $name;

        $( $impl_clause )* for $name {
            fn numval() -> u64 {
                $num
            }

            $( $fn )*
        }
    );
}

pub trait SyscallOneArg<A: Arg, R: Arg> {
    fn asm(arg0: u64) -> i64 {
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
    fn call(A) -> Ret<R>;
}

syscall1!(Close (3) [impl SyscallOneArg<Fd, Zero>] [
          fn call(f: Fd) -> Ret<Zero> {
              let v0: u64 = f.into();
              Zero::from_i64(Self::asm(v0), ())
          }]);
syscall1!(Brk (12) [impl<'a> SyscallOneArg<Ptr<'a>, Zero>] [
          fn call(f: Ptr) -> Ret<Zero> {
              let v0: u64 = f.into();
              Zero::from_i64(Self::asm(v0), ())
          }]);
// No sigreturn implementation
syscall1!(Pipe (22) [impl<'a> SyscallOneArg<Ptr<'a>, Zero>] [
          fn call(b: Ptr) -> Ret<Zero> {
              assert_eq!(b.buflen(), 2 * unsafe { int_width } as usize);
              let v0: u64 = b.into();
              Zero::from_i64(Self::asm(v0), ())
          }]);
