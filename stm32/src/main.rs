#![no_std] // for embedded environment
#![no_main] // for disable OS arguments

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use stm32f4xx_hal::{prelude::*, stm32, delay::Delay};

#[entry] // for calling with CPU init
// fn main() -> ! {
//     let _ = hprintln!("Hello, world!");
//
//     debug::exit(debug::EXIT_SUCCESS);
//
//     loop {}
// }
fn main() -> ! {
    // for GPIO control
    if let (Some(dp), Some(cp))
    = (stm32::Peripherals::take(), stm32::CorePeripherals::take()) {
        // enable RCC(Reset and clock control)
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // enable GPIO
        let gpiod = dp.GPIOD.split();
        let mut led = gpiod.pd15.into_push_pull_output();

        // blink LED
        let mut delay = Delay::new(cp.SYST, clocks);
        for _ in 0..5 {
            led.set_high().unwrap();
            delay.delay_ms(100_u32);

            led.set_low().unwrap();
            delay.delay_ms(100_u32);
        }
    }
    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}