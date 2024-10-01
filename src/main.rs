#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ToyOperatingSys::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ToyOperatingSys::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use ToyOperatingSys::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    ToyOperatingSys::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe {
        active_level_4_table(phys_mem_offset)
    };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    // Read Level 4 Page Table
    // use x86_64::registers::control::Cr3;
    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

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