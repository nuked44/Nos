<<<<<<< HEAD
fn main() {
    println!("Hello, world!");
=======
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
>>>>>>> 2931023 (custom architecture implemented)
}
