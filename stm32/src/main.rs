#![no_std] // for embedded environment
#![no_main] // for disable OS arguments

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use stm32f4xx_hal::{prelude::*, stm32, delay::Delay};
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::serial::config::{Config, StopBits};
use stm32f4xx_hal::serial::Serial;

#[entry] // for calling with CPU init
fn main() -> ! {
    let _ = hprintln!("Hello, world!");

    debug::exit(debug::EXIT_SUCCESS);

    loop {}
}

#[entry]
fn blink_main() -> ! {
    // for GPIO control
    // Controlling hardware with stm32::Peripherals by embedded-hal
    if let (Some(dp), Some(cp))
    = (stm32::Peripherals::take(), stm32::CorePeripherals::take()) {
        // take() must call only once. if it called more than one, it occur panic.
        // enable RCC(Reset and clock control)
        let rcc = dp.RCC.constrain();
        // let rcc = dp.RCC.constrain(); // ← RCC does not implement the Copy trait.
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // enable GPIO
        let gpiod = dp.GPIOD.split();
        let mut led = gpiod.pd15.into_push_pull_output();
        // let mut led = gpiod.pd15.into_floating_input(); ← Compile error. type mismatch.

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

#[entry]
fn main_uart() -> ! {
    if let Some(dp) = stm32::Peripherals::take() {
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
        let gpioc = dp.GPIOC.split();

        let config = Config::default()
            .baudrate(115200.bps())
            .wordlength_8()
            .parity_none()
            .stopbits(StopBits::STOP1);

        let pins = (
            gpioc.pc10.into_alternate_af8(),
            gpioc.pc11.into_alternate_af8()
            );
        let mut uart4 = Serial::uart4(dp.UART4, pins, config, clocks).unwrap();

        let _ = uart4.write(b'H');
        let _ = uart4.write(b'e');
        let _ = uart4.write(b'l');
        let _ = uart4.write(b'l');
        let _ = uart4.write(b'o');
        let _ = uart4.write(b' ');
        let _ = uart4.write(b'w');
        let _ = uart4.write(b'o');
        let _ = uart4.write(b'r');
        let _ = uart4.write(b'l');
        let _ = uart4.write(b'd');
        let _ = uart4.write(b'!');
        let _ = uart4.flush();
    }

    loop {}
}

#[entry]
fn main_i2c() -> ! {
    if let Some(dp) = stm32::Peripherals::take() {
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
        let gpiob = dp.GPIOB.split();

        let pins = (
            gpiob.pb8.into_alternate_af4().set_open_drain(),
            gpiob.pb9.into_alternate_af4().set_open_drain()
            );
        let mut i2c = I2c::new(dp.I2C1, pins, 400.khz(), clocks);

        let _ = i2c.write(0, &[0]);
    }

    loop {}
}
