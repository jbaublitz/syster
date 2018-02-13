mod sys0;
pub use self::sys0::*;

mod sys1;
pub use self::sys1::*;

use super::*;

pub type SyscallVal = u64;
pub type SyscallArg = u64;
pub type SyscallRet = i64;
