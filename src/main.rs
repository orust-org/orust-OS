#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, Welcome to the ORUST Operating System{}", "!");
    panic!("Some panic message");
    loop {}
}
