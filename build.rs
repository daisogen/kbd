use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::Command;

const KBD_REPO: &str = "https://git.kernel.org/pub/scm/linux/kernel/git/legion/kbd.git";
const KBD2CSV_REPO: &str = "https://github.com/jlxip/kbd2csv";

const KBD_CLONE: &str = "clones/kbd";
const KBD2CSV_CLONE: &str = "clones/kbd2csv";
const KBD2CSV_BIN: &str = "clones/kbd2csv/target/release/kbd2csv";

const DEST: &str = "keymaps";

fn main() {
    println!("cargo:rerun-if-changed=steal.txt");

    if !Path::new("clones").is_dir() {
        fs::create_dir("clones").unwrap();
    }

    // Get kbd
    if !Path::new(KBD_CLONE).is_dir() {
        Command::new("git")
            .args(["clone", KBD_REPO, KBD_CLONE])
            .status()
            .unwrap();
    }

    // Get kbd2csv
    if !Path::new(KBD2CSV_CLONE).is_dir() {
        Command::new("git")
            .args(["clone", KBD2CSV_REPO, KBD2CSV_CLONE])
            .status()
            .unwrap();
    }

    // Compile kbd2csv
    if !Path::new(KBD2CSV_BIN).is_file() {
        Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(Path::new(KBD2CSV_CLONE))
            .env("CARGO_ENCODED_RUSTFLAGS", "")
            .status()
            .unwrap();
    }

    if !Path::new(DEST).is_dir() {
        fs::create_dir(DEST).unwrap();
    }

    // What should I steal?
    let file = fs::File::open("steal.txt").unwrap();
    let lines = BufReader::new(file).lines();
    let mut files: Vec<String> = vec![];
    for line in lines.flatten() {
        if line.is_empty() {
            continue;
        }

        // Get the name
        let path = format!("{KBD_CLONE}/data/keymaps/{line}.map");
        let name = Path::new(&path).file_name().unwrap().to_str().unwrap();
        let newpath = format!("{DEST}/{name}");
        files.push(name.to_string());

        // Generate newpath from path
        Command::new(KBD2CSV_BIN)
            .args([path, newpath])
            .status()
            .unwrap();
    }

    // Finally, archive all the layouts in a tar
    files.insert(0, "acvf".to_string());
    files.insert(1, "../src/keymaps.tar".to_string());
    Command::new("tar")
        .args(files)
        .current_dir(Path::new(DEST))
        .status()
        .unwrap();
}
