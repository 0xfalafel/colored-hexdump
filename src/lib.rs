use colored::Colorize;

pub fn hexdump(bytes: &[u8]) -> String {
    let mut output = String::new();
    for byte in bytes {
        output.push_str(&format!("{:02x}", byte));
    }

    output
}

fn colorize_byte(byte: u8) -> &'static str {
    match byte {
        0x00 => "0x00",
        _ => todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let result = hexdump(&[0x00]);
        assert_eq!(result, "00");
    }
}
