use nix::Errno;

pub enum SyscallArg {
    Fd(u64),
    Id(u64),
    Ptr(*const [u8]),
    MutPtr(*mut [u8]),
}

#[derive(Debug,PartialEq)]
pub enum SyscallRet {
    Ret(i64),
    Fd(u64),
    Id(u64),
    Ptr(*const [u8]),
    MutPtr(*mut [u8]),
    StrPtr(*const str),
    Success,
    Zero,
    Err(Errno),
}

mod sys0;
pub use self::sys0::*;

mod sys1;
pub use self::sys1::*;

mod sys2;
pub use self::sys2::*;
