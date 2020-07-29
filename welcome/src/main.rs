use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans from HIT!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

// -------------------------------------
// | Hello fellow Rustaceans from HIT! |
// -------------------------------------
//               \
//                \
//                   _~^~^~_
//               \) /  o o  \ (/
//                 '_   -   _'
//                 / '-----' \
