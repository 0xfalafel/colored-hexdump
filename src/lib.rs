const RESET: &str   = "\x1b[0m";
const LIGHT_GREY: &str = "\x1b[242m";
const GREEN:   &str = "\x1b[32m";
const YELLOW:  &str = "\x1b[33m";
const MAGENTA: &str = "\x1b[35m";
const CYAN:    &str = "\x1b[36m";

pub fn hexdump(bytes: &[u8]) -> String {
    let mut output = String::new();
    for byte in bytes {
        output.push_str(&colorize_byte(byte));
        output.push_str(" ");
    }

    output
}

fn colorize_byte(byte: &u8) -> String {
    let color = match byte {
        0x00 => LIGHT_GREY, // null bytes
        b'\t' | b'\n' | 0x0c | b'\r' | b' ' => GREEN, // ascii whitespace
        0x21..0x7f => CYAN, // printable ascii
        ..0x80 => MAGENTA,  // non-printable ascii
        _ => YELLOW,        // other
    };
    format!("{}{:02x}{}", color, byte, RESET)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let result = hexdump(&[0x00]);
        println!("{}", result);
        assert_eq!(result, "00");
    }
}
