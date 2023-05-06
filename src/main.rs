#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::arch::asm;

const LED_PTN_RED_RED: u16      = 0x3000;
const LED_PTN_RED_GREEN: u16    = 0x5000;
const LED_PTN_RED_BLUE: u16     = 0x9000;

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe { 
        // P6  : 0xFCFE3018
        // PM6 : 0xFCFE3318
        // PMC6: 0xFCFE3418
        core::ptr::write_volatile(0xFCFE3318 as *mut u16, 0x0001);
        core::ptr::write_volatile(0xFCFE3418 as *mut u16, 0x0000);
        loop { 
            core::ptr::write_volatile(0xFCFE3018 as *mut u16, LED_PTN_RED_RED);
            for _ in 1..1000 {
                asm!("nop");
            }
            core::ptr::write_volatile(0xFCFE3018 as *mut u16, LED_PTN_RED_GREEN);
            for _ in 1..1000 {
                asm!("nop");
            }
            core::ptr::write_volatile(0xFCFE3018 as *mut u16, LED_PTN_RED_BLUE);
            for _ in 1..1000 {
                asm!("nop");
            }
        } 
    }
} 
#[panic_handler] 
fn panic (_info: &PanicInfo) -> ! {
    loop {}
}

