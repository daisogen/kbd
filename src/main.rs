#![no_std]
#![no_main]
#![feature(let_chains)]

mod layouts;
mod symbols;

use std::sync::Mutex;

#[no_mangle]
fn main() {
    layouts::init();
    symbols::init();

    std::daisogen::pd_set("kbd_buffer_keycode", buffer_keycode as u64);
    std::daisogen::yld();
}

struct State {
    mayus: bool,
    shift: bool,
    alt: bool,
}

static STATE: Mutex<State> = Mutex::new(State {
    mayus: false,
    shift: false,
    alt: false,
});

// Static producer-consumer queue for keycodes would be here
// TODO: for that I probably need to have a Semaphore and some sync
// primitives first

extern "C" fn buffer_keycode(keycode: u8) {
    // This is a temporal implementation, mainly for demonstration purposes
    let release = keycode & (1 << 7) != 0;
    let keycode = keycode & !(1 << 7);

    let val = layouts::get(keycode, 0);
    let Some(val) = val else { return; };

    let mut state = STATE.lock();
    match val.as_str() {
        "Shift" => {
            state.shift = !release;
        }
        "Alt" => {
            state.alt = !release;
        }
        "Caps_Lock" => {
            if !release {
                state.mayus = !state.mayus;
            }
        }
        "space" => {
            if !release {
                std::print!(" ");
            }
        }
        key => {
            if release {
                return;
            }

            if state.shift && let Some(shiftvariant) = layouts::get(keycode, 1) {
                if let Some(symbol) = symbols::get(&shiftvariant, false) {
                    std::print!("{}", symbol);
                }
                return;
            } else if state.alt && let Some(altvariant) = layouts::get(keycode, 2) {
                if let Some(symbol) = symbols::get(&altvariant, false) {
                    std::print!("{}", symbol);
                }
                return;
            }

            let mut mayus = false;
            if state.mayus {
                mayus = true;
            }
            if state.shift {
                mayus = !mayus;
            }

            let symbol = symbols::get(key, mayus);
            if let Some(symbol) = symbol {
                std::print!("{}", symbol);
            }
        }
    }
}
