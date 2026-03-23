#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Pull, Output, Input};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut led = Output::new(p.PIN_15, Level::Low);
    let delay = Duration::from_secs(1);
    loop {
        led.set_high();
        Timer::after(delay).await;

        led.set_low();
        Timer::after(delay).await;
    }
}
