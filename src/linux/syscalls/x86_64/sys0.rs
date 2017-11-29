use super::*;

macro_rules! syscall0 {
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

pub trait SyscallZeroArgs<R: Arg> {
    fn asm() -> i64 {
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
    fn call() -> Ret<R>;
}

syscall0!(SchedYield (24) [impl SyscallZeroArgs<Zero>] [
          fn call() -> Ret<Zero> {
              Zero::from_i64(Self::asm(), ())
          }]);
syscall0!(Pause (34) [impl SyscallZeroArgs<NoRet>] [
          fn call() -> Ret<NoRet> {
              NoRet::from_i64(Self::asm(), ())
          }]);
syscall0!(Getpid (39) [impl SyscallZeroArgs<Id>] [
          fn call() -> Ret<Id> {
              Id::from_i64(Self::asm(), ())
          }]);
syscall0!(Fork (57) [impl SyscallZeroArgs<Id>] [
          fn call() -> Ret<Id> {
              Id::from_i64(Self::asm(), ())
          }]);
syscall0!(Vfork (58) [impl SyscallZeroArgs<Id>] [
          fn call() -> Ret<Id> {
              Id::from_i64(Self::asm(), ())
          }]);
syscall0!(Getuid (102) [impl SyscallZeroArgs<Id>] [
          fn call() -> Ret<Id> {
              Id::from_i64(Self::asm(), ())
          }]);
syscall0!(Getgid (104) [impl SyscallZeroArgs<Id>] [
          fn call() -> Ret<Id> {
              Id::from_i64(Self::asm(), ())
          }]);
syscall0!(Geteuid (107) [impl SyscallZeroArgs<Id>] [
          fn call() -> Ret<Id> {
              Id::from_i64(Self::asm(), ())
          }]);
syscall0!(Getegid (108) [impl SyscallZeroArgs<Id>] [
          fn call() -> Ret<Id> {
              Id::from_i64(Self::asm(), ())
          }]);
syscall0!(Getppid (110) [impl SyscallZeroArgs<Id>] [
          fn call() -> Ret<Id> {
              Id::from_i64(Self::asm(), ())
          }]);
syscall0!(Getpgrp (111) [impl SyscallZeroArgs<Id>] [
          fn call() -> Ret<Id> {
              Id::from_i64(Self::asm(), ())
          }]);
syscall0!(Setsid (112) [impl SyscallZeroArgs<Id>] [
          fn call() -> Ret<Id> {
              Id::from_i64(Self::asm(), ())
          }]);
syscall0!(Munlockall (152) [impl SyscallZeroArgs<Zero>] [
          fn call() -> Ret<Zero> {
              Zero::from_i64(Self::asm(), ())
          }]);
syscall0!(Vhangup (153) [impl SyscallZeroArgs<Zero>] [
          fn call() -> Ret<Zero> {
              Zero::from_i64(Self::asm(), ())
          }]);
syscall0!(Sync (162) [impl SyscallZeroArgs<Zero>] [
          fn call() -> Ret<Zero> {
              Zero::from_i64(Self::asm(), ())
          }]);
syscall0!(Gettid (186) [impl SyscallZeroArgs<Id>] [
          fn call() -> Ret<Id> {
              Id::from_i64(Self::asm(), ())
          }]);
//RestartSyscall has no userspace application
syscall0!(InotifyInit (253) [impl SyscallZeroArgs<Fd>] [
          fn call() -> Ret<Fd> {
              Fd::from_i64(Self::asm(), ())
          }]);
