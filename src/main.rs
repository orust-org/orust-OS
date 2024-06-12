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

    //#[allow(clippy::empty_loop)]
    loop {
        use orust_os::print;
        print!("-");
        for _ in 0..10000{}
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    orust_os::test_panic_handler(info)
}