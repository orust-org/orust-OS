#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(orust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use orust_os::println;

pub trait Testable {
    fn run(&self);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, Welcome to the ORUST Operating System{}", "!");

    orust_os::init();

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