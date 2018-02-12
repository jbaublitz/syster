#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

pub trait Syscall {
    fn num() -> self::SyscallVal;
}

pub enum Zero {
    Success,
    Err(i64),
    InvalidData,
}

impl From<i64> for Zero {
    fn from(v: i64) -> Self {
        match v {
            i if i == 0 => Zero::Success,
            i if i > 0 => Zero::Err(i),
            _ => Zero::InvalidData,
        }
    }
}

pub struct Error(i64);

impl From<i64> for Error {
    fn from(v: i64) -> Self {
        Error(v)
    }
}

pub struct Pid(i64);

impl From<i64> for Pid {
    fn from(v: i64) -> Self {
        Pid(v)
    }
}

pub struct Id(i64);

impl From<i64> for Id {
    fn from(v: i64) -> Self {
        Id(v)
    }
}

pub struct Fd(i64);

impl From<i64> for Fd {
    fn from(v: i64) -> Self {
        Fd(v)
    }
}
