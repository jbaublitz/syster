#![feature(asm)]
#![no_std]

extern crate nix;
extern crate num;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::*;
