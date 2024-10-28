#![no_main]
#![no_std]

use cortex_m_rt::entry;
// use embedded_hal::delay::DelayNs;
use microbit::{board::Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rprintln;
use rtt_target::rtt_init_print;

use microbit_playground::font::FONT5X5;


fn find_char(c: char) -> [[u8; 5]; 5] {
    for (font, ch) in FONT5X5.iter() {
        if *ch == c {
            return *font;
        }
    }
    FONT5X5[0].0 // default to space
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("hello world");

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let message = "HELLO WORLD";

    loop {
        for c in message.chars() {
            let font = find_char(c);
            display.show(&mut timer, font, 400);
            // timer.delay_ms(500);
        }
    }
}
