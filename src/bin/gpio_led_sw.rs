#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Pull, Output, Input};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut led = Output::new(p.PIN_14, Level::Low);
    let sw = Input::new(p.PIN_16, Pull::Down);

    loop {
        if sw.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
