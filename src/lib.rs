// Remove if STD is supported for your platform and you plan to use it
#![no_std]

use e310x_hal as hal;

// Remove if STD is supported for your platform and you plan to use it
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

//
// Entry point
//

#[no_mangle]
extern "C" fn main() -> i32 {
    0
}
