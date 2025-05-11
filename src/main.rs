use std::u8;

use colored_hexdump::{hexdump, hexyl};

fn main() {
    let mut buf: Vec<u8> = vec![];

    for i in 0..u8::MAX {
        buf.push(i);
    }
    buf.push(u8::MAX);

    let res = hexyl(&buf);
    println!("{res}");
}