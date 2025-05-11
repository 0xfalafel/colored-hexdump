use colored_hexdump::{hexdump, hexyl};

fn main() {
    let mut buf: Vec<u8> = vec![];

    for i in 0..u8::MAX {
        buf.push(i);
    }

    let res = hexyl(&buf);
    println!("{res}");
}