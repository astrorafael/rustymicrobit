#![no_std]
#![no_main]

use cortex_m::asm::nop;
use rtt_target::{rtt_init_print,rprintln} ;
use cortex_m_rt::entry;
use panic_halt as _;


#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello world!");
    loop {
        rprintln!("Hello world!");
        for _ in 0..100_0000 {
            nop();
        }
    } 
}
