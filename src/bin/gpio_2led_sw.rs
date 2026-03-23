/*
 * スイッチでLED点灯
 */
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Pull, Output, Input};
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());

    let mut led1 = Output::new(p.PIN_14, Level::Low);
    let mut led2 = Output::new(p.PIN_15, Level::Low);
    let sw = Input::new(p.PIN_16, Pull::Down);

    let mut state: u8 = 0;  // 0-3の範囲
    let mut is_trans: bool = false;  // 遷移フラグ

    loop {
        if sw.is_high() && is_trans == false {
            // 状態遷移 0->1->2->3->0->...
            state += 1;
            if state > 3 {
                state = 0;
            }

            // LEDをセット
            if state == 1 {
                led1.set_high();
                led2.set_low();
            }
            else if state == 2 {
                led1.set_low();
                led2.set_high();
            }
            else if state == 3 {
                led1.set_high();
                led2.set_high();
            }
            else {
                led1.set_low();
                led2.set_low();
            }

            // 状態遷移フラグ
            is_trans = true;
        } else if sw.is_low() {
            is_trans = false;
        }
    }
}
