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
    println!("Hello World{}", "!");

    n_os::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {
    }
}

//------------------ PANIC HANDLING ------------------

#[cfg(any(not(test), rust_analyzer))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(all(test, not(rust_analyzer)))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    n_os::test_panic_handler(info)
}
