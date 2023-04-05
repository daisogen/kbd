// This handles all the keyboard state

use crate::{layouts, symbols};
use std::sync::Mutex;

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

pub fn get(keycode: u8) -> Option<char> {
    // Split the actual keycode and whether it's released
    let release = keycode & (1 << 7) != 0;
    let keycode = keycode & !(1 << 7);

    // What key does this keycode map to?
    let val = layouts::get(keycode, 0);
    let Some(val) = val else { return None; };

    // Cases (for state)
    let mut state = STATE.lock().unwrap();
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
                return Some(' ');
            }
        }
        key => {
            if release {
                return None;
            }

            if state.shift && let Some(shiftvariant) = layouts::get(keycode, 1) {
                return symbols::get(&shiftvariant, false);
            } else if state.alt && let Some(altvariant) = layouts::get(keycode, 2) {
                return symbols::get(&altvariant, false);
            }

            let mut mayus = false;
            if state.mayus {
                mayus = true;
            }
            if state.shift {
                mayus = !mayus;
            }

            return symbols::get(key, mayus);
        }
    }

    return None;
}
