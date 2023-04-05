// This is for translating keys ("Escape", "bracketleft") to symbols ("ñ", "[")

use std::collections::HashMap;
use std::string::String;
use std::string::ToString;
use std::sync::OnceLock;

static RAW_SYMBOLS: &str = include_str!("symbols.txt");
static SYMBOLS: OnceLock<HashMap<String, [char; 2]>> = OnceLock::new();

pub fn init() {
    let mut symbols: HashMap<String, [char; 2]> = HashMap::new();
    for i in RAW_SYMBOLS.lines() {
        let mut parts = i.split(" ");
        let name = parts.next().unwrap().to_string();
        let chars: [char; 2] = [
            parts.next().unwrap().chars().next().unwrap(),
            parts.next().unwrap_or("\0").chars().next().unwrap(),
        ];

        symbols.insert(name, chars);
    }

    SYMBOLS.get_or_init(|| symbols);
}

pub fn get(key: &str, mayus: bool) -> Option<char> {
    let opts = SYMBOLS.get().unwrap().get(key);
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
