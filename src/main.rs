#![no_std]
#![no_main]

#![feature(custom_test_frameworks)]
#![test_runner(n_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

//------------------ IMPORTS ------------------

use core::panic::PanicInfo;
use n_os::{ println };

//------------------ ENTRY POINT ------------------

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

//------------------ PANIC HANDLING ------------------

#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    n_os::test_panic_handler(info)
}
