#![no_std]
#![no_main] // overwriting rusts' entry point
#![feature(custom_test_frameworks)]
#![test_runner(orust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{BootInfo, entry_point};

use core::panic::PanicInfo;

use orust_os::println;

use orust_os::task::{keyboard, Task, executor::Executor};

pub trait Testable {
    fn run(&self);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, Welcome to the ORUST Operating System{}", "!");

    orust_os::init();

    #[cfg(test)]
    test_main();

        let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    orust_os::hlt_loop()
}

#[cfg(test)]
// function called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    orust_os::test_panic_handler(info)
}