#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(n_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

//------------------ IMPORTS ------------------

use core::panic::PanicInfo;
use n_os::println;

//------------------ ENTRY POINT ------------------

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    n_os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test println output")
}
