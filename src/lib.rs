const RESET: &str   = "\x1b[0m";
const LIGHT_GREY: &str = "\x1b[38;5;242m";
const GREEN:   &str = "\x1b[32m";
const YELLOW:  &str = "\x1b[33m";
const MAGENTA: &str = "\x1b[35m";
const CYAN:    &str = "\x1b[36m";

pub fn hexdump(bytes: &[u8]) -> String {
    let mut output = String::new();
    for byte in bytes {
        output.push_str(&colorize(byte));
        output.push_str(" ");
    }

    output
}

pub fn hexyl(bytes: &[u8]) -> String {
    let lines = bytes.len() / 16;

    let mut output = String::from("┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n");

    let mut index = 0;

    for line in 0..lines {

        output.push_str(&format!("│{}{:08x}{}│ ", LIGHT_GREY, line * 0x10, RESET));

        for _ in 0..8 {
            output.push_str(&colorize(&bytes[index]));
            output.push(' ');
            index += 1;

            if index > bytes.len() {
                return output
            }
        }

        output.push_str("│ ");

        for _ in 0..8 {
            output.push_str(&colorize(&bytes[index]));
            output.push(' ');
            index += 1;

            if index > bytes.len() {
                return output
            }
        }

        output.push_str("│\n");
    }

    // for (index, byte) in bytes.iter().enumerate() {
        
    //     match index % 16 {
    //         7 => output.push_str("│ "),
    //         16 => output.push_str("│\n│ "),
    //         _ => {},
    //     }
    //     output.push_str(&colorize(byte));
    //     output.push(' ');
    // }
    output
}

fn colorize(byte: &u8) -> String {
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
