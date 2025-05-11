pub fn hexdump(bytes: &[u8]) -> String {
    let mut output = String::new();
    for byte in bytes {
        output.push_str(&format!("{:02x}", byte));
    }

    output
}

#[allow(unused)]
fn colorize(byte: u8) {

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
