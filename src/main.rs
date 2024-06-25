#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(orust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};

use core::panic::PanicInfo;

use orust_os::println;

pub trait Testable {
    fn run(&self);
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use orust_os:: memory::translate_addr;
    use x86_64::VirtAddr;

    println!("Hello, Welcome to the ORUST Operating System{}", "!");
    orust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let addresses = [ 
        0xb8000, 
        0x201008, 
        0x0100_0020_1a10, 
        boot_info.physical_memory_offset
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = unsafe { translate_addr(virt, phys_mem_offset) };
        println!("{:?} -> {:?}", virt, phys);
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    orust_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    orust_os::hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    orust_os::test_panic_handler(info)
}