#![feature(let_chains)]
#![feature(once_cell)]
#![feature(daisogen_api)]

mod chars;
mod layouts;
mod mpsc;
mod symbols;

use std::io::Write;
use std::sync::{Arc, OnceLock};

static QUEUE: OnceLock<Arc<mpsc::Mpsc<u8>>> = OnceLock::new();

fn main() {
    layouts::init();
    symbols::init();

    QUEUE.get_or_init(|| Arc::new(mpsc::Mpsc::<u8>::new()));

    std::daisogen::pd_set("kbd_buffer_keycode", buffer_keycode as u64);
    std::daisogen::pd_set("kbd_get_keycode", get_keycode as u64);
    std::daisogen::pd_set("kbd_get_char", get_char as u64);
    std::daisogen::yld();
}

extern "C" fn buffer_keycode(keycode: usize) {
    QUEUE.get().unwrap().send(keycode as u8);
}

extern "C" fn get_keycode() -> usize {
    QUEUE.get().unwrap().recv() as usize
}

// --- ↓ Layout abstraction ↓ ---
extern "C" fn get_char() -> u64 {
    // This is temporal. Returing a char is not trivial, it's pretty much
    // like returning a Vec. For this reason, I need first to finish remote
    // allocations. In the meantime, I just print the letter to check if
    // everything works.
    if let Some(c) = chars::get(get_keycode() as u8) {
        std::print!("{}", c);
        std::io::stdout().flush().unwrap();
    }

    0
}
