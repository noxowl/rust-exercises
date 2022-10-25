// heapless.rs
// cargo add heapless = 0.6.1
#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
// heapless is implemented without memory allocator
use heapless::consts::U2; // removed in ^0.7
use heapless::Vec;

#[entry]
fn main() -> ! {
    let mut x = Vec::<_, U2>::new();
    let _ = x.push(1234);
    let _ = x.push(4567);
    let _ = x.push(7890);

    let _ = hprintln!("{:?}", x);
    // (qemu) [123, 456]
    // because we created x by Vec::<_, U2>. so, x.push(7890) was ignored.

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
