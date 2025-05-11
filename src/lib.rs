use colored::Colorize;

const RESET: &str = "\x1b[0m";
const LIGHT_GREY: &str = "\x1b[38┊;5;242m";

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
        0x00 => LIGHT_GREY,

        _ => todo!()
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
