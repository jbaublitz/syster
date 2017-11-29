mod sys0;
pub use self::sys0::*;

mod sys1;
pub use self::sys1::*;

mod sys2;
pub use self::sys2::*;

use core::slice;

/// Trait for conversion between unsafe assembly input and output and Rust representations
pub trait Arg: Into<u64> {
    type Size;

    fn from_i64(i64, Self::Size) -> Ret<Self>;
}

/// Indicates no return value
pub struct NoRet;

impl Into<u64> for NoRet {
    fn into(self) -> u64 {
        unimplemented!()
    }
}

impl Arg for NoRet {
    type Size = ();

    fn from_i64(v: i64, _s: Self::Size) -> Ret<Self> {
        match v {
            i if i < 0 => Ret::Error(i),
            _ => unimplemented!("Should never return without an error code"),
        }
    }
}

/// Indicates a zero return value on success exclusively
pub struct Zero;

impl Into<u64> for Zero {
    fn into(self) -> u64 {
        0
    }
}

impl Arg for Zero {
    type Size = ();

    fn from_i64(v: i64, _s: Self::Size) -> Ret<Self> {
        match v {
            i if i == 0 => Ret::Success(Zero),
            i if i < 0 => Ret::Error(i),
            _ => unimplemented!("Unexpected syscall return value: should be < 1"),
        }
    }
}

/// Indicates an int return value such as in the case of the `read` syscall
pub struct Int(i64);

impl Arg for Int {
    type Size = ();

    fn from_i64(v: i64, _s: Self::Size) -> Ret<Self> {
        match v {
            i if i >= 0 => Ret::Success(Int(i)),
            i if i < 0 => Ret::Error(i),
            _ => unimplemented!("Unreachable"),
        }
    }
}

impl Into<u64> for Int {
    fn into(self) -> u64 {
        self.0 as u64
    }
}

/// Indicates a numeric identifier that maps to a kernel resource
pub struct Id(u64);

impl Arg for Id {
    type Size = ();

    fn from_i64(v: i64, _s: Self::Size) -> Ret<Self> {
        match v {
            i if i >= 0 => Ret::Success(Id(i as u64)),
            i if i < 0 => Ret::Error(i),
            _ => unimplemented!("Unreachable"),
        }
    }
}

impl Into<u64> for Id {
    fn into(self) -> u64 {
        self.0
    }
}

/// Indicates a numeric identifier that maps to a kernel file descriptor
pub struct Fd(u64);

impl Arg for Fd {
    type Size = ();

    fn from_i64(v: i64, _s: Self::Size) -> Ret<Self> {
        match v {
            i if i >= 0 => Ret::Success(Fd(i as u64)),
            i if i < 0 => Ret::Error(i),
            _ => unimplemented!("Unreachable"),
        }
    }
}

impl Into<u64> for Fd {
    fn into(self) -> u64 {
        self.0
    }
}

/// Represents a pointer to memory
pub struct Ptr<'a>(&'a [u8]);

impl<'a> Arg for Ptr<'a> {
    type Size = usize;

    fn from_i64(v: i64, s: Self::Size) -> Ret<Self> {
        match v {
            i if i >= 0 => Ret::Success(Ptr(unsafe { slice::from_raw_parts(i as *const _, s) })),
            i if i < 0 => Ret::Error(i),
            _ => unimplemented!("Unreachable"),
        }
    }
}

impl<'a> Into<u64> for Ptr<'a> {
    fn into(self) -> u64 {
        self.0.as_ptr() as u64
    }
}

/// The top level return type for intepreting syscall integer return values
pub enum Ret<T> {
    Success(T),
    Error(i64),
}

impl<T: Arg> From<(i64, T::Size)> for Ret<T> {
    fn from(v: (i64, T::Size)) -> Self {
        let (i, size) = v;
        T::from_i64(i, size)
    }
}

#[cfg(test)]
mod test {
}
