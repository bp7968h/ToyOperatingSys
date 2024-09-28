use spin::Mutex;
use lazy_static::lazy_static;
use core::fmt::{self,Write};
use crate::vga_buffer::{Writer, Color, ColorCode, Buffer};
use x86_64::instructions::interrupts;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new(
        0, 
        ColorCode::new(Color::White, Color::Black), 
        unsafe { &mut *(0xb8000 as *mut Buffer) }
    ));
}



#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::printer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    })
}