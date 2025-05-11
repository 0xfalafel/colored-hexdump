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
    
    for line in 0..lines+1 {
        
        // address
        output.push_str(&format!("│{}{:08x}{}│ ", LIGHT_GREY, line * 0x10, RESET));
        
        for i in 0..0x10 {
            // print the colored byte in hexadecimal
            if index < bytes.len() {
                output.push_str(&colorize(&bytes[index]));
                
            // fill with whitespace if there are no more bytes
            } else { 
                output.push_str("  ");
            }
            
            output.push(' ');
            index += 1;
            
            // middle line separator
            if i == 7 {
                output.push_str("│ ");        
            }
            
        }
        
        output.push_str("│\n");

        if index >= bytes.len() {
            break;
        }
    }
    
    output.push_str("└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘");
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
