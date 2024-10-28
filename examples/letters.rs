#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::digital::InputPin;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rprintln;
use rtt_target::rtt_init_print;

use microbit_playground::font::FONT5X5;


const FONT_LENGTH: usize = FONT5X5.len();

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("hello world");

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut buttons = board.buttons;

    // if not using bsp
    // let peripherals = pac::Peripherals::take().unwrap();
    // let mut rng = hal::rng::Rng::new(board.RNG);

    let mut idx = 10;

    loop {
        if buttons.button_b.is_low().unwrap() {
            rprintln!("button b pressed");

            idx += 1;
            if idx == FONT_LENGTH {
                idx = 0;
            }
        } else if buttons.button_a.is_low().unwrap() {
            rprintln!("button a pressed");
            idx -= 1;

            if idx == 0 {
                idx = FONT_LENGTH - 1;
            }
        }

        display.show(&mut timer, FONT5X5[idx].0, 150);
        //  timer.delay_ms(100);
    }
}
