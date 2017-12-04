mod syscalls;
pub use self::syscalls::*;

#[link(name = "limits")]
extern {
    pub static int_width: u8;
}
