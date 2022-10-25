// panic.rs
// cargo add panic-semihosting
#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    panic!("panic!");
}