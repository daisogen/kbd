#![feature(let_chains)]
#![feature(once_cell)]
#![feature(daisogen_api)]

mod chars;
mod layouts;
mod mpsc;
mod symbols;

use std::sync::{Arc, OnceLock};

static QUEUE: OnceLock<Arc<mpsc::Mpsc<u8>>> = OnceLock::new();

fn main() {
    layouts::init();
    symbols::init();

    QUEUE.get_or_init(|| Arc::new(mpsc::Mpsc::<u8>::new()));

    std::daisogen::pd_set("kbd_buffer_keycode", buffer_keycode as u64);
    std::daisogen::pd_set("kbd_get_keycode", get_keycode as u64);
    std::daisogen::pd_set("kbd_get_low_key", get_low_key as u64);
    std::daisogen::pd_set("kbd_get_high_key", get_high_key as u64);
    std::daisogen::pd_set("kbd_key_to_char", key_to_char as u64);
    std::daisogen::pd_set("kbd_get_char", get_char as u64);
    std::daisogen::yld();
}

extern "C" fn buffer_keycode(keycode: usize) {
    QUEUE.get().unwrap().send(keycode as u8);
}

extern "C" fn get_keycode() -> usize {
    QUEUE.get().unwrap().recv() as usize
}

extern "C" fn get_low_key() -> usize {
    if let Some(k) = chars::get_low_key(get_keycode() as u8) {
        k as *const String as usize
    } else {
        0
    }
}

extern "C" fn get_high_key() -> usize {
    if let Some(k) = chars::get_high_key(get_keycode() as u8) {
        k as *const String as usize
    } else {
        0
    }
}

extern "C" fn key_to_char(key: usize) -> usize {
    let key = unsafe { &*(key as *const String) };
    chars::key_to_char(key).unwrap_or_default() as usize
}

extern "C" fn get_char() -> usize {
    chars::get_char(get_keycode() as u8).unwrap_or_default() as usize
}
