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

// This function does NOT include state
// It can return "A", "Shift", "Caps_Lock"
// But not "dollar", "numbersign"
pub fn get_low_key(keycode: u8) -> Option<&'static String> {
    // Ignore releases
    let keycode = keycode & !(1 << 7);
    // What key does this keycode map to?
    layouts::get(keycode, 0)
}

// This function DOES include state
// It can return "A", "dollar", "numbersign"
// But not "Shift", "Caps_Lock", "Alt"
pub fn get_high_key(keycode: u8) -> Option<&'static String> {
    // Split the actual keycode and whether it's released
    let release = keycode & (1 << 7) != 0;
    let keycode = keycode & !(1 << 7);

    // Get low key
    let Some(val) = layouts::get(keycode, 0) else { return None; };

    // Cases (for state)
    let mut state = STATE.lock().unwrap();
    match (*val).as_str() {
        "Shift" => {
            state.shift = !release;
            None
        }
        "Alt" => {
            state.alt = !release;
            None
        }
        "Caps_Lock" => {
            if !release {
                state.mayus = !state.mayus;
            }
            None
        }
        "space" => {
            if !release {
                Some(val)
            } else {
                None
            }
        }
        _ => {
            if release {
                return None;
            }

            if state.shift {
                return Some(layouts::get(keycode, 1).unwrap_or(val));
            } else if state.alt {
                return Some(layouts::get(keycode, 2).unwrap_or(val));
            }

            // No shift and no alt, so regular
            Some(val)
        }
    }
}

// This function DOES include state
// It can return "a", "A", "$"
// But not "dollar", "numbersign", "Shift"
pub fn key_to_char(key: &'static String) -> Option<char> {
    // The few that have to be hardcoded
    match (*key).as_str() {
        "space" => {
            return Some(' ');
        }
        "Return" => {
            return Some('\n');
        }
        _ => {}
    }

    // Handle mayus
    let state = STATE.lock().unwrap();
    let mut mayus = state.mayus;
    if state.shift {
        mayus = !mayus;
    }

    if let Some(s) = symbols::get(key, mayus) {
        // This key did have a mayus symbol, nice
        Some(s)
    } else {
        // It did not, force lowercase
        symbols::get(key, false)
    }
}

pub fn get_char(keycode: u8) -> Option<char> {
    key_to_char(get_high_key(keycode)?)
}
