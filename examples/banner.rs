#![no_main]
#![no_std]

use cortex_m_rt::entry;
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

// Get the LED matrix state when shift left from c0 to c1, with gap
fn get_led_transist(c0: char, c1: char, offset: usize, gap: usize) -> [[u8; 5]; 5] {
    let font0 = find_char(c0);
    let font1 = find_char(c1);
    assert!(offset <= 5 + gap);

    if offset == 0 {
        return font0;
    } else if offset >= 5 + gap {
        // when gap=1, if offset=6, it means c0 is empty, c1 is full
        return font1;
    } else {
        let mut result = [[0; 5]; 5];

        for i in 0..5 {
            for j in 0..5 {
                if j < 5 - offset {
                    result[i][j] = font0[i][j + offset];
                } else if j >= 5 - offset + gap {
                    result[i][j] = font1[i][j - (5 - offset + gap)];
                }
            }
        }

        result
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    rprintln!("hello world");

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    // extra space to make the message shift in and out
    let message = "  HELLO WORLD  ";

    loop {
        for (c0, c1) in message.chars().zip(message.chars().skip(1)) {
            let gap = 1;
            for offset in 0..5 + gap + 1 {
                let leds = get_led_transist(c0, c1, offset, gap);

                let duration: u32 = if offset == 0 { 500 } else { 40 };
                display.show(&mut timer, leds, duration);
            }
        }
    }
}
