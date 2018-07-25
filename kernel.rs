#![feature(panic_implementation, asm, lang_items)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_implementation]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn sleep(value: u32) {
 for _ in 1..value {
 unsafe { asm!(""); }
 }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
 let led_on = unsafe { 0x20200028 as *mut u32 };
 let led_off = unsafe { 0x2020001C as *mut u32 };
 let set_out = unsafe { 0x20200000 as *mut u32 };
 unsafe { *(set_out) = 0x40; }

loop {
 unsafe { *(led_on) = 0x4; }
 sleep(1000000);
 unsafe { *(led_off) = 0x4; }
 sleep(1000000);
 }
}

#[lang = "eh_personality"] extern fn eh_personality() {}

