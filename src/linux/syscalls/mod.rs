#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

pub trait Syscall {
    fn num() -> self::SyscallVal;
}

pub enum Zero {
    Success,
    Err(SyscallRet),
    InvalidData,
}

impl From<SyscallRet> for Zero {
    fn from(v: SyscallRet) -> Self {
        match v {
            i if i == 0 => Zero::Success,
            i if i > 0 => Zero::Err(i),
            _ => Zero::InvalidData,
        }
    }
}

pub struct Error(SyscallRet);

impl From<SyscallRet> for Error {
    fn from(v: SyscallRet) -> Self {
        Error(v)
    }
}

pub struct Pid(SyscallArg);

impl From<SyscallRet> for Pid {
    fn from(v: SyscallRet) -> Self {
        Pid(v as SyscallArg)
    }
}

pub struct Id(SyscallArg);

impl From<SyscallRet> for Id {
    fn from(v: SyscallRet) -> Self {
        Id(v as SyscallArg)
    }
}

pub struct Fd(SyscallArg);

impl Into<SyscallArg> for Fd {
    fn into(self) -> SyscallArg {
        self.0
    }
}

impl From<SyscallRet> for Fd {
    fn from(v: SyscallRet) -> Self {
        Fd(v as SyscallArg)
    }
}

pub struct Ptr(*const u8);

impl Into<SyscallArg> for Ptr {
    fn into(self) -> SyscallArg {
        self.0 as SyscallArg
    }
}

pub struct MutPtr(*mut u8);

impl Into<SyscallArg> for MutPtr {
    fn into(self) -> SyscallArg {
        self.0 as SyscallArg
    }
}

pub struct Int(SyscallArg);

impl Into<SyscallArg> for Int {
    fn into(self) -> SyscallArg {
        self.0
    }
}

impl From<SyscallRet> for Int {
    fn from(v: SyscallRet) -> Self {
        Int(v as SyscallArg)
    }
}

pub struct Never;

impl From<SyscallRet> for Never {
    fn from(_v: SyscallRet) -> Self {
        panic!("This function should never return")
    }
}
