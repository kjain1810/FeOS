#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(FeOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use FeOS::println;

#[no_mangle] 
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    FeOS::test_panic_handler(info)
}


#[test_case]
fn test_vga_single_line() {
    println!("vga buffer");
}
