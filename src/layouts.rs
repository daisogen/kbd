// This is for translating keycodes (5, 32) to keys ("Escape", "a", "bracketleft")

use std::collections::HashMap;
use std::string::String;
use std::string::ToString;
use std::sync::Mutex;
use std::sync::OnceLock;
use tar_no_std::TarArchiveRef;

static RAW_LAYOUTS: &[u8] = include_bytes!("keymaps.tar");
static TAR: OnceLock<TarArchiveRef> = OnceLock::new();

type Layout = HashMap<u8, [String; 5]>;
static LAYOUTS: OnceLock<HashMap<String, Layout>> = OnceLock::new();
static SELECTED: Mutex<String> = Mutex::new(String::new());

pub fn init() {
    TAR.get_or_init(|| TarArchiveRef::new(RAW_LAYOUTS));
    let mut layouts: HashMap<String, Layout> = HashMap::new();
    for i in TAR.get().unwrap().entries() {
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

        layouts.insert(name.to_string(), layout);
    }

    // Select "us" if available
    *SELECTED.lock().unwrap() = if layouts.contains_key("us") {
        "us".to_string()
    } else {
        // Too bad! Pick one at random
        let sel = layouts.keys().next();
        let sel = sel.expect("no valid keymaps");
        sel.clone()
    };

    LAYOUTS.get_or_init(|| layouts);
}

pub fn get(keycode: u8, variant: usize) -> Option<&'static String> {
    let selected = &*SELECTED.lock().unwrap();
    let opts = LAYOUTS.get().unwrap()[selected].get(&keycode)?;
    let ret = &opts[variant];

    if ret != "" {
        Some(ret)
    } else {
        None
    }
}
