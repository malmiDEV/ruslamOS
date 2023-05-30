use core::fmt::{
    Write, 
    Arguments
};

use crate::sys::drivers::vesa::VesaGraphics;

static mut SHELL: Shell = Shell::None;

pub(crate) enum Shell {
    None,
    VesaGraphics(VesaGraphics)
}

impl Write for Shell {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self {
            &mut Shell::None => todo!(),
            Shell::VesaGraphics(sh) => sh.write_str(s),
        }
    }
}

pub(crate) unsafe fn shell_set(sh: Shell) {
    SHELL = sh;
}

#[doc(hidden)]
pub fn write_fmt(args: Arguments) {
    unsafe { let _ = core::fmt::write(&mut SHELL, args); };
}
