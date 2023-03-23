// This is for translating keys ("Escape", "bracketleft") to symbols ("ñ", "[")

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::string::String;
use std::string::ToString;
use std::sync::Mutex;

static RAW_SYMBOLS: &str = include_str!("symbols.txt");

// This should totally be a OnceCell
lazy_static! {
    static ref SYMBOLS: Mutex<HashMap<String, [char; 2]>> = Mutex::new(HashMap::new());
}

pub fn init() {
    let mut locked = SYMBOLS.lock();
    for i in RAW_SYMBOLS.lines() {
        let mut parts = i.split(" ");
        let name = parts.next().unwrap().to_string();
        let chars: [char; 2] = [
            parts.next().unwrap().chars().next().unwrap(),
            parts.next().unwrap_or("\0").chars().next().unwrap(),
        ];

        locked.insert(name, chars);
    }
}

pub fn get(key: &str, mayus: bool) -> Option<char> {
    let locked = SYMBOLS.lock();
    let opts = locked.get(key);
    let Some(opts) = opts else {
        return None;
    };

    if !mayus {
        // Regular non-mayus symbol: first variant
        return Some(opts[0]);
    } else if opts[1] != '\0' {
        // Regular mayus symbol: second variant
        return Some(opts[1]);
    } else {
        // Wants mayus, but there's no second variant, so I return the first
        // (non-mayus)
        return Some(opts[0]);
    }
}
