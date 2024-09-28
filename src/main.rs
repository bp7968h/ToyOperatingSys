#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ToyOperatingSys::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ToyOperatingSys::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    ToyOperatingSys::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    ToyOperatingSys::hlt_loop();
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    ToyOperatingSys::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ToyOperatingSys::test_panic_handler(info)
}