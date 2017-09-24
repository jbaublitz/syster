#![feature(asm)]
#![no_std]

extern crate nix;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::*;
