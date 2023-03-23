// This is for translating keycodes (5, 32) to keys ("Escape", "a", "bracketleft")

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::string::String;
use std::string::ToString;
use std::sync::Mutex;
use tar_no_std::TarArchiveRef;

static RAW_LAYOUTS: &[u8] = include_bytes!("keymaps.tar");

type Layout = HashMap<u8, [String; 5]>;
lazy_static! {
    static ref LAYOUTS: Mutex<HashMap<String, Layout>> = Mutex::new(HashMap::new());
}
static SELECTED: Mutex<String> = Mutex::new(String::new());

pub fn init() {
    let archive = TarArchiveRef::new(RAW_LAYOUTS);
    for i in archive.entries() {
        let name = i.filename();
        let name = name.split(".").next().unwrap();
        let data = i.data_as_str().unwrap();

        // Parse data (CSV)
        let mut layout: Layout = Layout::new();
        for line in data.lines() {
            let mut map = line.split(",");
            let mut opts: [String; 5] = Default::default();

            let key = map.next().unwrap().parse::<u8>().unwrap();
            for (idx, j) in map.enumerate() {
                opts[idx] = j.to_string();
            }

            layout.insert(key, opts);
        }

        LAYOUTS.lock().insert(name.to_string(), layout);
    }

    // Select "us" if available
    if LAYOUTS.lock().contains_key("us") {
        *SELECTED.lock() = "us".to_string();
    } else {
        // Too bad! Pick one at random
        let locked = LAYOUTS.lock();
        let sel = locked.keys().next();
        let sel = sel.expect("no valid keymaps");
        *SELECTED.lock() = sel.clone();
    }
}

pub fn get(keycode: u8, variant: usize) -> Option<String> {
    // TODO: this is clearly a temporal solution
    let locked = LAYOUTS.lock();
    let opts = locked[&*SELECTED.lock()].get(&keycode)?;
    let ret = opts[variant].clone();

    if ret != "" {
        Some(ret)
    } else {
        None
    }
}
