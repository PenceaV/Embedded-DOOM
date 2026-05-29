use embassy_rp::gpio::Output;
use crate::game::state::player::Player;

pub fn update_leds(player: &Player, red: &mut Output, green: &mut Output, blue: &mut Output, frames: u32) {
    match player.hp {
        h if h >= 3 => set_led(red, green, blue, false, true, false),
        2 => set_led(red, green, blue, false, false, true),
        1 => set_led(red, green, blue, true, false, false),
        _ => {
            let flash = (frames / 5) % 2 == 0;
            set_led(red, green, blue, flash, false, false);
        }
    }
}

fn set_led(red: &mut Output, green: &mut Output, blue: &mut Output, r: bool, g: bool, b: bool) {
    if r { red.set_high(); } else { red.set_low(); }
    if g { green.set_high(); } else { green.set_low(); }
    if b { blue.set_high(); } else { blue.set_low(); }
}
