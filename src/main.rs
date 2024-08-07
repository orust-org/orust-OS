#![no_std]
#![no_main] // overwriting rusts' entry point
#![feature(custom_test_frameworks)]
#![test_runner(orust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use orust_os::println;

pub trait Testable {
    fn run(&self);
}
// don't mangle this fucntion since the linker looks for a function 
// named '_start' by default
#[no_mangle]
// entrypoint function with "C" calling convention.
pub extern "C" fn _start() -> ! {
    println!("Hello, Welcome To The ORUST Operating System{}", "!");

    orust_os::init();

    #[cfg(test)] 
    test_main();

    println!("It did not crash!");

    #[allow(clippy::empty_loop)]
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[cfg(test)]
// function called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    orust_os::test_panic_handler(info)
}

