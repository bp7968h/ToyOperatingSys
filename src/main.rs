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
    use ToyOperatingSys::memory::{active_level_4_table, translate_addr};
    use x86_64::VirtAddr;
    use x86_64::structures::paging::PageTable;

    println!("Hello World{}", "!");
    ToyOperatingSys::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];
    
    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
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