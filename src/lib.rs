//! # Syster
//! ## Syscalls with no use of the standard library, no external crate dependencies, and no nonsense
//!
//! More documentation coming soon - this crate is under heavy development

#![feature(asm)]
#![no_std]

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::*;
