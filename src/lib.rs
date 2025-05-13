mod braille;
use crate::braille::braille_char;

const RESET: &str   = "\x1b[0m";
const LIGHT_GREY: &str = "\x1b[38;5;242m";
const GREEN:   &str = "\x1b[32m";
const YELLOW:  &str = "\x1b[33m";
const MAGENTA: &str = "\x1b[35m";
const CYAN:    &str = "\x1b[36m";

/// Produce a colored hexdump with borders
pub fn hexdump(bytes: &[u8]) -> String {
    hexyl(bytes, BrailleMode::Mixed)
}

/// Produce a colored hexdump in the style of xxd
pub fn xxd(bytes: &[u8]) -> String {
    xxd_braille(bytes, BrailleMode::Mixed)
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrailleMode {
//    None,
    Mixed,
    All,
}

pub fn hexyl(bytes: &[u8], braille: BrailleMode) -> String {
    let lines = bytes.len() / 16;

    let mut output = String::from("┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n");
    
    let mut index = 0;
    
    for line in 0..lines+1 {
        let mut ascii_line = String::from("│");
        
        // address
        output.push_str(&format!("│{}{:08x}{}│ ", LIGHT_GREY, line * 0x10, RESET));
        
        for i in 0..0x10 {
            // print the colored byte in hexadecimal
            if index < bytes.len() {
                output.push_str(&colorize_byte(&bytes[index]));
                ascii_line.push_str(&colorize_ascii(&bytes[index], braille));

            // fill with whitespace if there are no more bytes
            } else { 
                output.push_str("  ");
                ascii_line.push(' ');
            }
            
            output.push(' ');
            index += 1;
            
            // middle line separator
            if i == 7 {
                output.push_str("│ ");
                ascii_line.push('│');
            }
            
        }
        
        output.push_str(&ascii_line);
        output.push_str("│\n");

        if index >= bytes.len() {
            break;
        }
    }
    
    output.push_str("└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘");
    output
}

pub fn xxd_braille(bytes: &[u8], braille: BrailleMode) -> String {
    let lines = bytes.len() / 16;

    let mut output = String::new();
    let mut index = 0;
    
    for line in 0..lines+1 {
        let mut ascii_line = String::new();
        
        // address
        output.push_str(&format!("{}{:08x}: {}", LIGHT_GREY, line * 0x10, RESET));
        
        for i in 0..0x10 {
            // print the colored byte in hexadecimal
            if index < bytes.len() {
                output.push_str(&colorize_byte(&bytes[index]));
                ascii_line.push_str(&colorize_ascii(&bytes[index], braille));

            // fill with whitespace if there are no more bytes
            } else { 
                output.push_str("  ");
                ascii_line.push(' ');
            }
            
            if i % 2 == 1 {
                output.push(' ');
            }

            index += 1;
        }
        
        output.push_str(&ascii_line);
        output.push_str("\n");

        if index >= bytes.len() {
            break;
        }
    }
    output
}


fn color(byte: &u8) -> &str {
    match byte {
        0x00 => LIGHT_GREY, // null bytes
        b'\t' | b'\n' | 0x0c | b'\r' | b' ' => GREEN, // ascii whitespace
        0x21..0x7f => CYAN, // printable ascii
        ..0x80 => MAGENTA,  // non-printable ascii
        _ => YELLOW,        // other
    }
}

fn colorize_byte(byte: &u8) -> String {
    format!("{}{:02x}{}", color(byte), byte, RESET)
}

fn colorize_ascii(byte: &u8, braille: BrailleMode) -> String {
    let ascii_char = match braille {
        BrailleMode::All => braille_char(*byte),
        BrailleMode::Mixed => mixed_braille(*byte),
    };
    format!("{}{}{}", color(byte), ascii_char, RESET)
}

/// Take a u8, return classic chars for value bellow 0x80, and a Braille ascii for other values
/// It's a pretty Ok compromise in readability
fn mixed_braille(val: u8) -> char {
	match val {
		val if val == 0x00 => {'0'},
		val if val == 0x20 => {' '},
		val if val.is_ascii_whitespace() => {'_'},
		val if val > 0x20 && val < 0x7f => {val as char},
		val if val.is_ascii() => {'•'},
		val => {braille_char(val)} // 0x80 and above
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let result = dump(&[0x00]);
        println!("{}", result);
        assert_eq!(result, "00");
    }
}
