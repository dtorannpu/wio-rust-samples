#![no_std]
#![no_main]

use panic_halt as _;
use wio::hal::clock::GenericClockController;
use wio::hal::delay::Delay;
use wio::{
    entry,
    pac::{CorePeripherals, Peripherals},
    prelude::*,
};
use wio_terminal as wio;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let sets = wio::Pins::new(peripherals.PORT).split();

    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut user_led = sets.user_led.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &mut clocks);

    loop {
        user_led.toggle().ok();
        delay.delay_ms(1000u16);
    }
}
