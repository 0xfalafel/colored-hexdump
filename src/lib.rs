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
    fn test_hexdump() {
        let all_bytes: Vec<u8> = (0..=u8::MAX).collect();
        let result = hexdump(&all_bytes);
        // println!("{}", result);
        assert_eq!(result, "┌────────┬─────────────────────────┬─────────────────────────┬────────┬────────┐\n│\u{1b}[38;5;242m00000000\u{1b}[0m│ \u{1b}[38;5;242m00\u{1b}[0m \u{1b}[35m01\u{1b}[0m \u{1b}[35m02\u{1b}[0m \u{1b}[35m03\u{1b}[0m \u{1b}[35m04\u{1b}[0m \u{1b}[35m05\u{1b}[0m \u{1b}[35m06\u{1b}[0m \u{1b}[35m07\u{1b}[0m │ \u{1b}[35m08\u{1b}[0m \u{1b}[32m09\u{1b}[0m \u{1b}[32m0a\u{1b}[0m \u{1b}[35m0b\u{1b}[0m \u{1b}[32m0c\u{1b}[0m \u{1b}[32m0d\u{1b}[0m \u{1b}[35m0e\u{1b}[0m \u{1b}[35m0f\u{1b}[0m │\u{1b}[38;5;242m0\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m│\u{1b}[35m•\u{1b}[0m\u{1b}[32m_\u{1b}[0m\u{1b}[32m_\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[32m_\u{1b}[0m\u{1b}[32m_\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m│\n│\u{1b}[38;5;242m00000010\u{1b}[0m│ \u{1b}[35m10\u{1b}[0m \u{1b}[35m11\u{1b}[0m \u{1b}[35m12\u{1b}[0m \u{1b}[35m13\u{1b}[0m \u{1b}[35m14\u{1b}[0m \u{1b}[35m15\u{1b}[0m \u{1b}[35m16\u{1b}[0m \u{1b}[35m17\u{1b}[0m │ \u{1b}[35m18\u{1b}[0m \u{1b}[35m19\u{1b}[0m \u{1b}[35m1a\u{1b}[0m \u{1b}[35m1b\u{1b}[0m \u{1b}[35m1c\u{1b}[0m \u{1b}[35m1d\u{1b}[0m \u{1b}[35m1e\u{1b}[0m \u{1b}[35m1f\u{1b}[0m │\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m│\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m│\n│\u{1b}[38;5;242m00000020\u{1b}[0m│ \u{1b}[32m20\u{1b}[0m \u{1b}[36m21\u{1b}[0m \u{1b}[36m22\u{1b}[0m \u{1b}[36m23\u{1b}[0m \u{1b}[36m24\u{1b}[0m \u{1b}[36m25\u{1b}[0m \u{1b}[36m26\u{1b}[0m \u{1b}[36m27\u{1b}[0m │ \u{1b}[36m28\u{1b}[0m \u{1b}[36m29\u{1b}[0m \u{1b}[36m2a\u{1b}[0m \u{1b}[36m2b\u{1b}[0m \u{1b}[36m2c\u{1b}[0m \u{1b}[36m2d\u{1b}[0m \u{1b}[36m2e\u{1b}[0m \u{1b}[36m2f\u{1b}[0m │\u{1b}[32m \u{1b}[0m\u{1b}[36m!\u{1b}[0m\u{1b}[36m\"\u{1b}[0m\u{1b}[36m#\u{1b}[0m\u{1b}[36m$\u{1b}[0m\u{1b}[36m%\u{1b}[0m\u{1b}[36m&\u{1b}[0m\u{1b}[36m'\u{1b}[0m│\u{1b}[36m(\u{1b}[0m\u{1b}[36m)\u{1b}[0m\u{1b}[36m*\u{1b}[0m\u{1b}[36m+\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36m-\u{1b}[0m\u{1b}[36m.\u{1b}[0m\u{1b}[36m/\u{1b}[0m│\n│\u{1b}[38;5;242m00000030\u{1b}[0m│ \u{1b}[36m30\u{1b}[0m \u{1b}[36m31\u{1b}[0m \u{1b}[36m32\u{1b}[0m \u{1b}[36m33\u{1b}[0m \u{1b}[36m34\u{1b}[0m \u{1b}[36m35\u{1b}[0m \u{1b}[36m36\u{1b}[0m \u{1b}[36m37\u{1b}[0m │ \u{1b}[36m38\u{1b}[0m \u{1b}[36m39\u{1b}[0m \u{1b}[36m3a\u{1b}[0m \u{1b}[36m3b\u{1b}[0m \u{1b}[36m3c\u{1b}[0m \u{1b}[36m3d\u{1b}[0m \u{1b}[36m3e\u{1b}[0m \u{1b}[36m3f\u{1b}[0m │\u{1b}[36m0\u{1b}[0m\u{1b}[36m1\u{1b}[0m\u{1b}[36m2\u{1b}[0m\u{1b}[36m3\u{1b}[0m\u{1b}[36m4\u{1b}[0m\u{1b}[36m5\u{1b}[0m\u{1b}[36m6\u{1b}[0m\u{1b}[36m7\u{1b}[0m│\u{1b}[36m8\u{1b}[0m\u{1b}[36m9\u{1b}[0m\u{1b}[36m:\u{1b}[0m\u{1b}[36m;\u{1b}[0m\u{1b}[36m<\u{1b}[0m\u{1b}[36m=\u{1b}[0m\u{1b}[36m>\u{1b}[0m\u{1b}[36m?\u{1b}[0m│\n│\u{1b}[38;5;242m00000040\u{1b}[0m│ \u{1b}[36m40\u{1b}[0m \u{1b}[36m41\u{1b}[0m \u{1b}[36m42\u{1b}[0m \u{1b}[36m43\u{1b}[0m \u{1b}[36m44\u{1b}[0m \u{1b}[36m45\u{1b}[0m \u{1b}[36m46\u{1b}[0m \u{1b}[36m47\u{1b}[0m │ \u{1b}[36m48\u{1b}[0m \u{1b}[36m49\u{1b}[0m \u{1b}[36m4a\u{1b}[0m \u{1b}[36m4b\u{1b}[0m \u{1b}[36m4c\u{1b}[0m \u{1b}[36m4d\u{1b}[0m \u{1b}[36m4e\u{1b}[0m \u{1b}[36m4f\u{1b}[0m │\u{1b}[36m@\u{1b}[0m\u{1b}[36mA\u{1b}[0m\u{1b}[36mB\u{1b}[0m\u{1b}[36mC\u{1b}[0m\u{1b}[36mD\u{1b}[0m\u{1b}[36mE\u{1b}[0m\u{1b}[36mF\u{1b}[0m\u{1b}[36mG\u{1b}[0m│\u{1b}[36mH\u{1b}[0m\u{1b}[36mI\u{1b}[0m\u{1b}[36mJ\u{1b}[0m\u{1b}[36mK\u{1b}[0m\u{1b}[36mL\u{1b}[0m\u{1b}[36mM\u{1b}[0m\u{1b}[36mN\u{1b}[0m\u{1b}[36mO\u{1b}[0m│\n│\u{1b}[38;5;242m00000050\u{1b}[0m│ \u{1b}[36m50\u{1b}[0m \u{1b}[36m51\u{1b}[0m \u{1b}[36m52\u{1b}[0m \u{1b}[36m53\u{1b}[0m \u{1b}[36m54\u{1b}[0m \u{1b}[36m55\u{1b}[0m \u{1b}[36m56\u{1b}[0m \u{1b}[36m57\u{1b}[0m │ \u{1b}[36m58\u{1b}[0m \u{1b}[36m59\u{1b}[0m \u{1b}[36m5a\u{1b}[0m \u{1b}[36m5b\u{1b}[0m \u{1b}[36m5c\u{1b}[0m \u{1b}[36m5d\u{1b}[0m \u{1b}[36m5e\u{1b}[0m \u{1b}[36m5f\u{1b}[0m │\u{1b}[36mP\u{1b}[0m\u{1b}[36mQ\u{1b}[0m\u{1b}[36mR\u{1b}[0m\u{1b}[36mS\u{1b}[0m\u{1b}[36mT\u{1b}[0m\u{1b}[36mU\u{1b}[0m\u{1b}[36mV\u{1b}[0m\u{1b}[36mW\u{1b}[0m│\u{1b}[36mX\u{1b}[0m\u{1b}[36mY\u{1b}[0m\u{1b}[36mZ\u{1b}[0m\u{1b}[36m[\u{1b}[0m\u{1b}[36m\\\u{1b}[0m\u{1b}[36m]\u{1b}[0m\u{1b}[36m^\u{1b}[0m\u{1b}[36m_\u{1b}[0m│\n│\u{1b}[38;5;242m00000060\u{1b}[0m│ \u{1b}[36m60\u{1b}[0m \u{1b}[36m61\u{1b}[0m \u{1b}[36m62\u{1b}[0m \u{1b}[36m63\u{1b}[0m \u{1b}[36m64\u{1b}[0m \u{1b}[36m65\u{1b}[0m \u{1b}[36m66\u{1b}[0m \u{1b}[36m67\u{1b}[0m │ \u{1b}[36m68\u{1b}[0m \u{1b}[36m69\u{1b}[0m \u{1b}[36m6a\u{1b}[0m \u{1b}[36m6b\u{1b}[0m \u{1b}[36m6c\u{1b}[0m \u{1b}[36m6d\u{1b}[0m \u{1b}[36m6e\u{1b}[0m \u{1b}[36m6f\u{1b}[0m │\u{1b}[36m`\u{1b}[0m\u{1b}[36ma\u{1b}[0m\u{1b}[36mb\u{1b}[0m\u{1b}[36mc\u{1b}[0m\u{1b}[36md\u{1b}[0m\u{1b}[36me\u{1b}[0m\u{1b}[36mf\u{1b}[0m\u{1b}[36mg\u{1b}[0m│\u{1b}[36mh\u{1b}[0m\u{1b}[36mi\u{1b}[0m\u{1b}[36mj\u{1b}[0m\u{1b}[36mk\u{1b}[0m\u{1b}[36ml\u{1b}[0m\u{1b}[36mm\u{1b}[0m\u{1b}[36mn\u{1b}[0m\u{1b}[36mo\u{1b}[0m│\n│\u{1b}[38;5;242m00000070\u{1b}[0m│ \u{1b}[36m70\u{1b}[0m \u{1b}[36m71\u{1b}[0m \u{1b}[36m72\u{1b}[0m \u{1b}[36m73\u{1b}[0m \u{1b}[36m74\u{1b}[0m \u{1b}[36m75\u{1b}[0m \u{1b}[36m76\u{1b}[0m \u{1b}[36m77\u{1b}[0m │ \u{1b}[36m78\u{1b}[0m \u{1b}[36m79\u{1b}[0m \u{1b}[36m7a\u{1b}[0m \u{1b}[36m7b\u{1b}[0m \u{1b}[36m7c\u{1b}[0m \u{1b}[36m7d\u{1b}[0m \u{1b}[36m7e\u{1b}[0m \u{1b}[35m7f\u{1b}[0m │\u{1b}[36mp\u{1b}[0m\u{1b}[36mq\u{1b}[0m\u{1b}[36mr\u{1b}[0m\u{1b}[36ms\u{1b}[0m\u{1b}[36mt\u{1b}[0m\u{1b}[36mu\u{1b}[0m\u{1b}[36mv\u{1b}[0m\u{1b}[36mw\u{1b}[0m│\u{1b}[36mx\u{1b}[0m\u{1b}[36my\u{1b}[0m\u{1b}[36mz\u{1b}[0m\u{1b}[36m{\u{1b}[0m\u{1b}[36m|\u{1b}[0m\u{1b}[36m}\u{1b}[0m\u{1b}[36m~\u{1b}[0m\u{1b}[35m•\u{1b}[0m│\n│\u{1b}[38;5;242m00000080\u{1b}[0m│ \u{1b}[33m80\u{1b}[0m \u{1b}[33m81\u{1b}[0m \u{1b}[33m82\u{1b}[0m \u{1b}[33m83\u{1b}[0m \u{1b}[33m84\u{1b}[0m \u{1b}[33m85\u{1b}[0m \u{1b}[33m86\u{1b}[0m \u{1b}[33m87\u{1b}[0m │ \u{1b}[33m88\u{1b}[0m \u{1b}[33m89\u{1b}[0m \u{1b}[33m8a\u{1b}[0m \u{1b}[33m8b\u{1b}[0m \u{1b}[33m8c\u{1b}[0m \u{1b}[33m8d\u{1b}[0m \u{1b}[33m8e\u{1b}[0m \u{1b}[33m8f\u{1b}[0m │\u{1b}[33m⠠\u{1b}[0m\u{1b}[33m⡀\u{1b}[0m\u{1b}[33m⢀\u{1b}[0m\u{1b}[33m⣀\u{1b}[0m\u{1b}[33m⠠\u{1b}[0m\u{1b}[33m⡠\u{1b}[0m\u{1b}[33m⢠\u{1b}[0m\u{1b}[33m⣠\u{1b}[0m│\u{1b}[33m⠄\u{1b}[0m\u{1b}[33m⡄\u{1b}[0m\u{1b}[33m⢄\u{1b}[0m\u{1b}[33m⣄\u{1b}[0m\u{1b}[33m⠤\u{1b}[0m\u{1b}[33m⡤\u{1b}[0m\u{1b}[33m⢤\u{1b}[0m\u{1b}[33m⣤\u{1b}[0m│\n│\u{1b}[38;5;242m00000090\u{1b}[0m│ \u{1b}[33m90\u{1b}[0m \u{1b}[33m91\u{1b}[0m \u{1b}[33m92\u{1b}[0m \u{1b}[33m93\u{1b}[0m \u{1b}[33m94\u{1b}[0m \u{1b}[33m95\u{1b}[0m \u{1b}[33m96\u{1b}[0m \u{1b}[33m97\u{1b}[0m │ \u{1b}[33m98\u{1b}[0m \u{1b}[33m99\u{1b}[0m \u{1b}[33m9a\u{1b}[0m \u{1b}[33m9b\u{1b}[0m \u{1b}[33m9c\u{1b}[0m \u{1b}[33m9d\u{1b}[0m \u{1b}[33m9e\u{1b}[0m \u{1b}[33m9f\u{1b}[0m │\u{1b}[33m⠁\u{1b}[0m\u{1b}[33m⡁\u{1b}[0m\u{1b}[33m⢁\u{1b}[0m\u{1b}[33m⣁\u{1b}[0m\u{1b}[33m⠡\u{1b}[0m\u{1b}[33m⡡\u{1b}[0m\u{1b}[33m⢡\u{1b}[0m\u{1b}[33m⣡\u{1b}[0m│\u{1b}[33m⠅\u{1b}[0m\u{1b}[33m⡅\u{1b}[0m\u{1b}[33m⢅\u{1b}[0m\u{1b}[33m⣅\u{1b}[0m\u{1b}[33m⠥\u{1b}[0m\u{1b}[33m⡥\u{1b}[0m\u{1b}[33m⢥\u{1b}[0m\u{1b}[33m⣥\u{1b}[0m│\n│\u{1b}[38;5;242m000000a0\u{1b}[0m│ \u{1b}[33ma0\u{1b}[0m \u{1b}[33ma1\u{1b}[0m \u{1b}[33ma2\u{1b}[0m \u{1b}[33ma3\u{1b}[0m \u{1b}[33ma4\u{1b}[0m \u{1b}[33ma5\u{1b}[0m \u{1b}[33ma6\u{1b}[0m \u{1b}[33ma7\u{1b}[0m │ \u{1b}[33ma8\u{1b}[0m \u{1b}[33ma9\u{1b}[0m \u{1b}[33maa\u{1b}[0m \u{1b}[33mab\u{1b}[0m \u{1b}[33mac\u{1b}[0m \u{1b}[33mad\u{1b}[0m \u{1b}[33mae\u{1b}[0m \u{1b}[33maf\u{1b}[0m │\u{1b}[33m⠃\u{1b}[0m\u{1b}[33m⡃\u{1b}[0m\u{1b}[33m⢃\u{1b}[0m\u{1b}[33m⣃\u{1b}[0m\u{1b}[33m⠣\u{1b}[0m\u{1b}[33m⡣\u{1b}[0m\u{1b}[33m⢣\u{1b}[0m\u{1b}[33m⣣\u{1b}[0m│\u{1b}[33m⠇\u{1b}[0m\u{1b}[33m⡇\u{1b}[0m\u{1b}[33m⢇\u{1b}[0m\u{1b}[33m⣇\u{1b}[0m\u{1b}[33m⠧\u{1b}[0m\u{1b}[33m⡧\u{1b}[0m\u{1b}[33m⢧\u{1b}[0m\u{1b}[33m⣧\u{1b}[0m│\n│\u{1b}[38;5;242m000000b0\u{1b}[0m│ \u{1b}[33mb0\u{1b}[0m \u{1b}[33mb1\u{1b}[0m \u{1b}[33mb2\u{1b}[0m \u{1b}[33mb3\u{1b}[0m \u{1b}[33mb4\u{1b}[0m \u{1b}[33mb5\u{1b}[0m \u{1b}[33mb6\u{1b}[0m \u{1b}[33mb7\u{1b}[0m │ \u{1b}[33mb8\u{1b}[0m \u{1b}[33mb9\u{1b}[0m \u{1b}[33mba\u{1b}[0m \u{1b}[33mbb\u{1b}[0m \u{1b}[33mbc\u{1b}[0m \u{1b}[33mbd\u{1b}[0m \u{1b}[33mbe\u{1b}[0m \u{1b}[33mbf\u{1b}[0m │\u{1b}[33m⠉\u{1b}[0m\u{1b}[33m⡉\u{1b}[0m\u{1b}[33m⢉\u{1b}[0m\u{1b}[33m⣉\u{1b}[0m\u{1b}[33m⠩\u{1b}[0m\u{1b}[33m⡩\u{1b}[0m\u{1b}[33m⢩\u{1b}[0m\u{1b}[33m⣩\u{1b}[0m│\u{1b}[33m⠍\u{1b}[0m\u{1b}[33m⡍\u{1b}[0m\u{1b}[33m⢍\u{1b}[0m\u{1b}[33m⣍\u{1b}[0m\u{1b}[33m⠭\u{1b}[0m\u{1b}[33m⡭\u{1b}[0m\u{1b}[33m⢭\u{1b}[0m\u{1b}[33m⣭\u{1b}[0m│\n│\u{1b}[38;5;242m000000c0\u{1b}[0m│ \u{1b}[33mc0\u{1b}[0m \u{1b}[33mc1\u{1b}[0m \u{1b}[33mc2\u{1b}[0m \u{1b}[33mc3\u{1b}[0m \u{1b}[33mc4\u{1b}[0m \u{1b}[33mc5\u{1b}[0m \u{1b}[33mc6\u{1b}[0m \u{1b}[33mc7\u{1b}[0m │ \u{1b}[33mc8\u{1b}[0m \u{1b}[33mc9\u{1b}[0m \u{1b}[33mca\u{1b}[0m \u{1b}[33mcb\u{1b}[0m \u{1b}[33mcc\u{1b}[0m \u{1b}[33mcd\u{1b}[0m \u{1b}[33mce\u{1b}[0m \u{1b}[33mcf\u{1b}[0m │\u{1b}[33m⠊\u{1b}[0m\u{1b}[33m⡊\u{1b}[0m\u{1b}[33m⢊\u{1b}[0m\u{1b}[33m⣊\u{1b}[0m\u{1b}[33m⠪\u{1b}[0m\u{1b}[33m⡪\u{1b}[0m\u{1b}[33m⢪\u{1b}[0m\u{1b}[33m⣪\u{1b}[0m│\u{1b}[33m⠎\u{1b}[0m\u{1b}[33m⡎\u{1b}[0m\u{1b}[33m⢎\u{1b}[0m\u{1b}[33m⣎\u{1b}[0m\u{1b}[33m⠮\u{1b}[0m\u{1b}[33m⡮\u{1b}[0m\u{1b}[33m⢮\u{1b}[0m\u{1b}[33m⣮\u{1b}[0m│\n│\u{1b}[38;5;242m000000d0\u{1b}[0m│ \u{1b}[33md0\u{1b}[0m \u{1b}[33md1\u{1b}[0m \u{1b}[33md2\u{1b}[0m \u{1b}[33md3\u{1b}[0m \u{1b}[33md4\u{1b}[0m \u{1b}[33md5\u{1b}[0m \u{1b}[33md6\u{1b}[0m \u{1b}[33md7\u{1b}[0m │ \u{1b}[33md8\u{1b}[0m \u{1b}[33md9\u{1b}[0m \u{1b}[33mda\u{1b}[0m \u{1b}[33mdb\u{1b}[0m \u{1b}[33mdc\u{1b}[0m \u{1b}[33mdd\u{1b}[0m \u{1b}[33mde\u{1b}[0m \u{1b}[33mdf\u{1b}[0m │\u{1b}[33m⠑\u{1b}[0m\u{1b}[33m⡑\u{1b}[0m\u{1b}[33m⢑\u{1b}[0m\u{1b}[33m⣑\u{1b}[0m\u{1b}[33m⠱\u{1b}[0m\u{1b}[33m⡱\u{1b}[0m\u{1b}[33m⢱\u{1b}[0m\u{1b}[33m⣱\u{1b}[0m│\u{1b}[33m⠕\u{1b}[0m\u{1b}[33m⡕\u{1b}[0m\u{1b}[33m⢕\u{1b}[0m\u{1b}[33m⣕\u{1b}[0m\u{1b}[33m⠵\u{1b}[0m\u{1b}[33m⡵\u{1b}[0m\u{1b}[33m⢵\u{1b}[0m\u{1b}[33m⣵\u{1b}[0m│\n│\u{1b}[38;5;242m000000e0\u{1b}[0m│ \u{1b}[33me0\u{1b}[0m \u{1b}[33me1\u{1b}[0m \u{1b}[33me2\u{1b}[0m \u{1b}[33me3\u{1b}[0m \u{1b}[33me4\u{1b}[0m \u{1b}[33me5\u{1b}[0m \u{1b}[33me6\u{1b}[0m \u{1b}[33me7\u{1b}[0m │ \u{1b}[33me8\u{1b}[0m \u{1b}[33me9\u{1b}[0m \u{1b}[33mea\u{1b}[0m \u{1b}[33meb\u{1b}[0m \u{1b}[33mec\u{1b}[0m \u{1b}[33med\u{1b}[0m \u{1b}[33mee\u{1b}[0m \u{1b}[33mef\u{1b}[0m │\u{1b}[33m⠚\u{1b}[0m\u{1b}[33m⡚\u{1b}[0m\u{1b}[33m⢚\u{1b}[0m\u{1b}[33m⣚\u{1b}[0m\u{1b}[33m⠺\u{1b}[0m\u{1b}[33m⡺\u{1b}[0m\u{1b}[33m⢺\u{1b}[0m\u{1b}[33m⣺\u{1b}[0m│\u{1b}[33m⠞\u{1b}[0m\u{1b}[33m⡞\u{1b}[0m\u{1b}[33m⢞\u{1b}[0m\u{1b}[33m⣞\u{1b}[0m\u{1b}[33m⠾\u{1b}[0m\u{1b}[33m⡾\u{1b}[0m\u{1b}[33m⢾\u{1b}[0m\u{1b}[33m⣾\u{1b}[0m│\n│\u{1b}[38;5;242m000000f0\u{1b}[0m│ \u{1b}[33mf0\u{1b}[0m \u{1b}[33mf1\u{1b}[0m \u{1b}[33mf2\u{1b}[0m \u{1b}[33mf3\u{1b}[0m \u{1b}[33mf4\u{1b}[0m \u{1b}[33mf5\u{1b}[0m \u{1b}[33mf6\u{1b}[0m \u{1b}[33mf7\u{1b}[0m │ \u{1b}[33mf8\u{1b}[0m \u{1b}[33mf9\u{1b}[0m \u{1b}[33mfa\u{1b}[0m \u{1b}[33mfb\u{1b}[0m \u{1b}[33mfc\u{1b}[0m \u{1b}[33mfd\u{1b}[0m \u{1b}[33mfe\u{1b}[0m \u{1b}[33mff\u{1b}[0m │\u{1b}[33m⠛\u{1b}[0m\u{1b}[33m⡛\u{1b}[0m\u{1b}[33m⢛\u{1b}[0m\u{1b}[33m⣛\u{1b}[0m\u{1b}[33m⠻\u{1b}[0m\u{1b}[33m⡻\u{1b}[0m\u{1b}[33m⢻\u{1b}[0m\u{1b}[33m⣻\u{1b}[0m│\u{1b}[33m⠟\u{1b}[0m\u{1b}[33m⡟\u{1b}[0m\u{1b}[33m⢟\u{1b}[0m\u{1b}[33m⣟\u{1b}[0m\u{1b}[33m⠿\u{1b}[0m\u{1b}[33m⡿\u{1b}[0m\u{1b}[33m⢿\u{1b}[0m\u{1b}[33m⣿\u{1b}[0m│\n└────────┴─────────────────────────┴─────────────────────────┴────────┴────────┘");
    }

    #[test]
    fn test_xxd() {
        let all_bytes: Vec<u8> = (0..=u8::MAX).collect();
        let result = xxd(&all_bytes);
        // println!("{}", result);
        assert_eq!(result, "\u{1b}[38;5;242m00000000: \u{1b}[0m\u{1b}[38;5;242m00\u{1b}[0m\u{1b}[35m01\u{1b}[0m \u{1b}[35m02\u{1b}[0m\u{1b}[35m03\u{1b}[0m \u{1b}[35m04\u{1b}[0m\u{1b}[35m05\u{1b}[0m \u{1b}[35m06\u{1b}[0m\u{1b}[35m07\u{1b}[0m \u{1b}[35m08\u{1b}[0m\u{1b}[32m09\u{1b}[0m \u{1b}[32m0a\u{1b}[0m\u{1b}[35m0b\u{1b}[0m \u{1b}[32m0c\u{1b}[0m\u{1b}[32m0d\u{1b}[0m \u{1b}[35m0e\u{1b}[0m\u{1b}[35m0f\u{1b}[0m \u{1b}[38;5;242m0\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[32m_\u{1b}[0m\u{1b}[32m_\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[32m_\u{1b}[0m\u{1b}[32m_\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\n\u{1b}[38;5;242m00000010: \u{1b}[0m\u{1b}[35m10\u{1b}[0m\u{1b}[35m11\u{1b}[0m \u{1b}[35m12\u{1b}[0m\u{1b}[35m13\u{1b}[0m \u{1b}[35m14\u{1b}[0m\u{1b}[35m15\u{1b}[0m \u{1b}[35m16\u{1b}[0m\u{1b}[35m17\u{1b}[0m \u{1b}[35m18\u{1b}[0m\u{1b}[35m19\u{1b}[0m \u{1b}[35m1a\u{1b}[0m\u{1b}[35m1b\u{1b}[0m \u{1b}[35m1c\u{1b}[0m\u{1b}[35m1d\u{1b}[0m \u{1b}[35m1e\u{1b}[0m\u{1b}[35m1f\u{1b}[0m \u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\u{1b}[35m•\u{1b}[0m\n\u{1b}[38;5;242m00000020: \u{1b}[0m\u{1b}[32m20\u{1b}[0m\u{1b}[36m21\u{1b}[0m \u{1b}[36m22\u{1b}[0m\u{1b}[36m23\u{1b}[0m \u{1b}[36m24\u{1b}[0m\u{1b}[36m25\u{1b}[0m \u{1b}[36m26\u{1b}[0m\u{1b}[36m27\u{1b}[0m \u{1b}[36m28\u{1b}[0m\u{1b}[36m29\u{1b}[0m \u{1b}[36m2a\u{1b}[0m\u{1b}[36m2b\u{1b}[0m \u{1b}[36m2c\u{1b}[0m\u{1b}[36m2d\u{1b}[0m \u{1b}[36m2e\u{1b}[0m\u{1b}[36m2f\u{1b}[0m \u{1b}[32m \u{1b}[0m\u{1b}[36m!\u{1b}[0m\u{1b}[36m\"\u{1b}[0m\u{1b}[36m#\u{1b}[0m\u{1b}[36m$\u{1b}[0m\u{1b}[36m%\u{1b}[0m\u{1b}[36m&\u{1b}[0m\u{1b}[36m'\u{1b}[0m\u{1b}[36m(\u{1b}[0m\u{1b}[36m)\u{1b}[0m\u{1b}[36m*\u{1b}[0m\u{1b}[36m+\u{1b}[0m\u{1b}[36m,\u{1b}[0m\u{1b}[36m-\u{1b}[0m\u{1b}[36m.\u{1b}[0m\u{1b}[36m/\u{1b}[0m\n\u{1b}[38;5;242m00000030: \u{1b}[0m\u{1b}[36m30\u{1b}[0m\u{1b}[36m31\u{1b}[0m \u{1b}[36m32\u{1b}[0m\u{1b}[36m33\u{1b}[0m \u{1b}[36m34\u{1b}[0m\u{1b}[36m35\u{1b}[0m \u{1b}[36m36\u{1b}[0m\u{1b}[36m37\u{1b}[0m \u{1b}[36m38\u{1b}[0m\u{1b}[36m39\u{1b}[0m \u{1b}[36m3a\u{1b}[0m\u{1b}[36m3b\u{1b}[0m \u{1b}[36m3c\u{1b}[0m\u{1b}[36m3d\u{1b}[0m \u{1b}[36m3e\u{1b}[0m\u{1b}[36m3f\u{1b}[0m \u{1b}[36m0\u{1b}[0m\u{1b}[36m1\u{1b}[0m\u{1b}[36m2\u{1b}[0m\u{1b}[36m3\u{1b}[0m\u{1b}[36m4\u{1b}[0m\u{1b}[36m5\u{1b}[0m\u{1b}[36m6\u{1b}[0m\u{1b}[36m7\u{1b}[0m\u{1b}[36m8\u{1b}[0m\u{1b}[36m9\u{1b}[0m\u{1b}[36m:\u{1b}[0m\u{1b}[36m;\u{1b}[0m\u{1b}[36m<\u{1b}[0m\u{1b}[36m=\u{1b}[0m\u{1b}[36m>\u{1b}[0m\u{1b}[36m?\u{1b}[0m\n\u{1b}[38;5;242m00000040: \u{1b}[0m\u{1b}[36m40\u{1b}[0m\u{1b}[36m41\u{1b}[0m \u{1b}[36m42\u{1b}[0m\u{1b}[36m43\u{1b}[0m \u{1b}[36m44\u{1b}[0m\u{1b}[36m45\u{1b}[0m \u{1b}[36m46\u{1b}[0m\u{1b}[36m47\u{1b}[0m \u{1b}[36m48\u{1b}[0m\u{1b}[36m49\u{1b}[0m \u{1b}[36m4a\u{1b}[0m\u{1b}[36m4b\u{1b}[0m \u{1b}[36m4c\u{1b}[0m\u{1b}[36m4d\u{1b}[0m \u{1b}[36m4e\u{1b}[0m\u{1b}[36m4f\u{1b}[0m \u{1b}[36m@\u{1b}[0m\u{1b}[36mA\u{1b}[0m\u{1b}[36mB\u{1b}[0m\u{1b}[36mC\u{1b}[0m\u{1b}[36mD\u{1b}[0m\u{1b}[36mE\u{1b}[0m\u{1b}[36mF\u{1b}[0m\u{1b}[36mG\u{1b}[0m\u{1b}[36mH\u{1b}[0m\u{1b}[36mI\u{1b}[0m\u{1b}[36mJ\u{1b}[0m\u{1b}[36mK\u{1b}[0m\u{1b}[36mL\u{1b}[0m\u{1b}[36mM\u{1b}[0m\u{1b}[36mN\u{1b}[0m\u{1b}[36mO\u{1b}[0m\n\u{1b}[38;5;242m00000050: \u{1b}[0m\u{1b}[36m50\u{1b}[0m\u{1b}[36m51\u{1b}[0m \u{1b}[36m52\u{1b}[0m\u{1b}[36m53\u{1b}[0m \u{1b}[36m54\u{1b}[0m\u{1b}[36m55\u{1b}[0m \u{1b}[36m56\u{1b}[0m\u{1b}[36m57\u{1b}[0m \u{1b}[36m58\u{1b}[0m\u{1b}[36m59\u{1b}[0m \u{1b}[36m5a\u{1b}[0m\u{1b}[36m5b\u{1b}[0m \u{1b}[36m5c\u{1b}[0m\u{1b}[36m5d\u{1b}[0m \u{1b}[36m5e\u{1b}[0m\u{1b}[36m5f\u{1b}[0m \u{1b}[36mP\u{1b}[0m\u{1b}[36mQ\u{1b}[0m\u{1b}[36mR\u{1b}[0m\u{1b}[36mS\u{1b}[0m\u{1b}[36mT\u{1b}[0m\u{1b}[36mU\u{1b}[0m\u{1b}[36mV\u{1b}[0m\u{1b}[36mW\u{1b}[0m\u{1b}[36mX\u{1b}[0m\u{1b}[36mY\u{1b}[0m\u{1b}[36mZ\u{1b}[0m\u{1b}[36m[\u{1b}[0m\u{1b}[36m\\\u{1b}[0m\u{1b}[36m]\u{1b}[0m\u{1b}[36m^\u{1b}[0m\u{1b}[36m_\u{1b}[0m\n\u{1b}[38;5;242m00000060: \u{1b}[0m\u{1b}[36m60\u{1b}[0m\u{1b}[36m61\u{1b}[0m \u{1b}[36m62\u{1b}[0m\u{1b}[36m63\u{1b}[0m \u{1b}[36m64\u{1b}[0m\u{1b}[36m65\u{1b}[0m \u{1b}[36m66\u{1b}[0m\u{1b}[36m67\u{1b}[0m \u{1b}[36m68\u{1b}[0m\u{1b}[36m69\u{1b}[0m \u{1b}[36m6a\u{1b}[0m\u{1b}[36m6b\u{1b}[0m \u{1b}[36m6c\u{1b}[0m\u{1b}[36m6d\u{1b}[0m \u{1b}[36m6e\u{1b}[0m\u{1b}[36m6f\u{1b}[0m \u{1b}[36m`\u{1b}[0m\u{1b}[36ma\u{1b}[0m\u{1b}[36mb\u{1b}[0m\u{1b}[36mc\u{1b}[0m\u{1b}[36md\u{1b}[0m\u{1b}[36me\u{1b}[0m\u{1b}[36mf\u{1b}[0m\u{1b}[36mg\u{1b}[0m\u{1b}[36mh\u{1b}[0m\u{1b}[36mi\u{1b}[0m\u{1b}[36mj\u{1b}[0m\u{1b}[36mk\u{1b}[0m\u{1b}[36ml\u{1b}[0m\u{1b}[36mm\u{1b}[0m\u{1b}[36mn\u{1b}[0m\u{1b}[36mo\u{1b}[0m\n\u{1b}[38;5;242m00000070: \u{1b}[0m\u{1b}[36m70\u{1b}[0m\u{1b}[36m71\u{1b}[0m \u{1b}[36m72\u{1b}[0m\u{1b}[36m73\u{1b}[0m \u{1b}[36m74\u{1b}[0m\u{1b}[36m75\u{1b}[0m \u{1b}[36m76\u{1b}[0m\u{1b}[36m77\u{1b}[0m \u{1b}[36m78\u{1b}[0m\u{1b}[36m79\u{1b}[0m \u{1b}[36m7a\u{1b}[0m\u{1b}[36m7b\u{1b}[0m \u{1b}[36m7c\u{1b}[0m\u{1b}[36m7d\u{1b}[0m \u{1b}[36m7e\u{1b}[0m\u{1b}[35m7f\u{1b}[0m \u{1b}[36mp\u{1b}[0m\u{1b}[36mq\u{1b}[0m\u{1b}[36mr\u{1b}[0m\u{1b}[36ms\u{1b}[0m\u{1b}[36mt\u{1b}[0m\u{1b}[36mu\u{1b}[0m\u{1b}[36mv\u{1b}[0m\u{1b}[36mw\u{1b}[0m\u{1b}[36mx\u{1b}[0m\u{1b}[36my\u{1b}[0m\u{1b}[36mz\u{1b}[0m\u{1b}[36m{\u{1b}[0m\u{1b}[36m|\u{1b}[0m\u{1b}[36m}\u{1b}[0m\u{1b}[36m~\u{1b}[0m\u{1b}[35m•\u{1b}[0m\n\u{1b}[38;5;242m00000080: \u{1b}[0m\u{1b}[33m80\u{1b}[0m\u{1b}[33m81\u{1b}[0m \u{1b}[33m82\u{1b}[0m\u{1b}[33m83\u{1b}[0m \u{1b}[33m84\u{1b}[0m\u{1b}[33m85\u{1b}[0m \u{1b}[33m86\u{1b}[0m\u{1b}[33m87\u{1b}[0m \u{1b}[33m88\u{1b}[0m\u{1b}[33m89\u{1b}[0m \u{1b}[33m8a\u{1b}[0m\u{1b}[33m8b\u{1b}[0m \u{1b}[33m8c\u{1b}[0m\u{1b}[33m8d\u{1b}[0m \u{1b}[33m8e\u{1b}[0m\u{1b}[33m8f\u{1b}[0m \u{1b}[33m⠠\u{1b}[0m\u{1b}[33m⡀\u{1b}[0m\u{1b}[33m⢀\u{1b}[0m\u{1b}[33m⣀\u{1b}[0m\u{1b}[33m⠠\u{1b}[0m\u{1b}[33m⡠\u{1b}[0m\u{1b}[33m⢠\u{1b}[0m\u{1b}[33m⣠\u{1b}[0m\u{1b}[33m⠄\u{1b}[0m\u{1b}[33m⡄\u{1b}[0m\u{1b}[33m⢄\u{1b}[0m\u{1b}[33m⣄\u{1b}[0m\u{1b}[33m⠤\u{1b}[0m\u{1b}[33m⡤\u{1b}[0m\u{1b}[33m⢤\u{1b}[0m\u{1b}[33m⣤\u{1b}[0m\n\u{1b}[38;5;242m00000090: \u{1b}[0m\u{1b}[33m90\u{1b}[0m\u{1b}[33m91\u{1b}[0m \u{1b}[33m92\u{1b}[0m\u{1b}[33m93\u{1b}[0m \u{1b}[33m94\u{1b}[0m\u{1b}[33m95\u{1b}[0m \u{1b}[33m96\u{1b}[0m\u{1b}[33m97\u{1b}[0m \u{1b}[33m98\u{1b}[0m\u{1b}[33m99\u{1b}[0m \u{1b}[33m9a\u{1b}[0m\u{1b}[33m9b\u{1b}[0m \u{1b}[33m9c\u{1b}[0m\u{1b}[33m9d\u{1b}[0m \u{1b}[33m9e\u{1b}[0m\u{1b}[33m9f\u{1b}[0m \u{1b}[33m⠁\u{1b}[0m\u{1b}[33m⡁\u{1b}[0m\u{1b}[33m⢁\u{1b}[0m\u{1b}[33m⣁\u{1b}[0m\u{1b}[33m⠡\u{1b}[0m\u{1b}[33m⡡\u{1b}[0m\u{1b}[33m⢡\u{1b}[0m\u{1b}[33m⣡\u{1b}[0m\u{1b}[33m⠅\u{1b}[0m\u{1b}[33m⡅\u{1b}[0m\u{1b}[33m⢅\u{1b}[0m\u{1b}[33m⣅\u{1b}[0m\u{1b}[33m⠥\u{1b}[0m\u{1b}[33m⡥\u{1b}[0m\u{1b}[33m⢥\u{1b}[0m\u{1b}[33m⣥\u{1b}[0m\n\u{1b}[38;5;242m000000a0: \u{1b}[0m\u{1b}[33ma0\u{1b}[0m\u{1b}[33ma1\u{1b}[0m \u{1b}[33ma2\u{1b}[0m\u{1b}[33ma3\u{1b}[0m \u{1b}[33ma4\u{1b}[0m\u{1b}[33ma5\u{1b}[0m \u{1b}[33ma6\u{1b}[0m\u{1b}[33ma7\u{1b}[0m \u{1b}[33ma8\u{1b}[0m\u{1b}[33ma9\u{1b}[0m \u{1b}[33maa\u{1b}[0m\u{1b}[33mab\u{1b}[0m \u{1b}[33mac\u{1b}[0m\u{1b}[33mad\u{1b}[0m \u{1b}[33mae\u{1b}[0m\u{1b}[33maf\u{1b}[0m \u{1b}[33m⠃\u{1b}[0m\u{1b}[33m⡃\u{1b}[0m\u{1b}[33m⢃\u{1b}[0m\u{1b}[33m⣃\u{1b}[0m\u{1b}[33m⠣\u{1b}[0m\u{1b}[33m⡣\u{1b}[0m\u{1b}[33m⢣\u{1b}[0m\u{1b}[33m⣣\u{1b}[0m\u{1b}[33m⠇\u{1b}[0m\u{1b}[33m⡇\u{1b}[0m\u{1b}[33m⢇\u{1b}[0m\u{1b}[33m⣇\u{1b}[0m\u{1b}[33m⠧\u{1b}[0m\u{1b}[33m⡧\u{1b}[0m\u{1b}[33m⢧\u{1b}[0m\u{1b}[33m⣧\u{1b}[0m\n\u{1b}[38;5;242m000000b0: \u{1b}[0m\u{1b}[33mb0\u{1b}[0m\u{1b}[33mb1\u{1b}[0m \u{1b}[33mb2\u{1b}[0m\u{1b}[33mb3\u{1b}[0m \u{1b}[33mb4\u{1b}[0m\u{1b}[33mb5\u{1b}[0m \u{1b}[33mb6\u{1b}[0m\u{1b}[33mb7\u{1b}[0m \u{1b}[33mb8\u{1b}[0m\u{1b}[33mb9\u{1b}[0m \u{1b}[33mba\u{1b}[0m\u{1b}[33mbb\u{1b}[0m \u{1b}[33mbc\u{1b}[0m\u{1b}[33mbd\u{1b}[0m \u{1b}[33mbe\u{1b}[0m\u{1b}[33mbf\u{1b}[0m \u{1b}[33m⠉\u{1b}[0m\u{1b}[33m⡉\u{1b}[0m\u{1b}[33m⢉\u{1b}[0m\u{1b}[33m⣉\u{1b}[0m\u{1b}[33m⠩\u{1b}[0m\u{1b}[33m⡩\u{1b}[0m\u{1b}[33m⢩\u{1b}[0m\u{1b}[33m⣩\u{1b}[0m\u{1b}[33m⠍\u{1b}[0m\u{1b}[33m⡍\u{1b}[0m\u{1b}[33m⢍\u{1b}[0m\u{1b}[33m⣍\u{1b}[0m\u{1b}[33m⠭\u{1b}[0m\u{1b}[33m⡭\u{1b}[0m\u{1b}[33m⢭\u{1b}[0m\u{1b}[33m⣭\u{1b}[0m\n\u{1b}[38;5;242m000000c0: \u{1b}[0m\u{1b}[33mc0\u{1b}[0m\u{1b}[33mc1\u{1b}[0m \u{1b}[33mc2\u{1b}[0m\u{1b}[33mc3\u{1b}[0m \u{1b}[33mc4\u{1b}[0m\u{1b}[33mc5\u{1b}[0m \u{1b}[33mc6\u{1b}[0m\u{1b}[33mc7\u{1b}[0m \u{1b}[33mc8\u{1b}[0m\u{1b}[33mc9\u{1b}[0m \u{1b}[33mca\u{1b}[0m\u{1b}[33mcb\u{1b}[0m \u{1b}[33mcc\u{1b}[0m\u{1b}[33mcd\u{1b}[0m \u{1b}[33mce\u{1b}[0m\u{1b}[33mcf\u{1b}[0m \u{1b}[33m⠊\u{1b}[0m\u{1b}[33m⡊\u{1b}[0m\u{1b}[33m⢊\u{1b}[0m\u{1b}[33m⣊\u{1b}[0m\u{1b}[33m⠪\u{1b}[0m\u{1b}[33m⡪\u{1b}[0m\u{1b}[33m⢪\u{1b}[0m\u{1b}[33m⣪\u{1b}[0m\u{1b}[33m⠎\u{1b}[0m\u{1b}[33m⡎\u{1b}[0m\u{1b}[33m⢎\u{1b}[0m\u{1b}[33m⣎\u{1b}[0m\u{1b}[33m⠮\u{1b}[0m\u{1b}[33m⡮\u{1b}[0m\u{1b}[33m⢮\u{1b}[0m\u{1b}[33m⣮\u{1b}[0m\n\u{1b}[38;5;242m000000d0: \u{1b}[0m\u{1b}[33md0\u{1b}[0m\u{1b}[33md1\u{1b}[0m \u{1b}[33md2\u{1b}[0m\u{1b}[33md3\u{1b}[0m \u{1b}[33md4\u{1b}[0m\u{1b}[33md5\u{1b}[0m \u{1b}[33md6\u{1b}[0m\u{1b}[33md7\u{1b}[0m \u{1b}[33md8\u{1b}[0m\u{1b}[33md9\u{1b}[0m \u{1b}[33mda\u{1b}[0m\u{1b}[33mdb\u{1b}[0m \u{1b}[33mdc\u{1b}[0m\u{1b}[33mdd\u{1b}[0m \u{1b}[33mde\u{1b}[0m\u{1b}[33mdf\u{1b}[0m \u{1b}[33m⠑\u{1b}[0m\u{1b}[33m⡑\u{1b}[0m\u{1b}[33m⢑\u{1b}[0m\u{1b}[33m⣑\u{1b}[0m\u{1b}[33m⠱\u{1b}[0m\u{1b}[33m⡱\u{1b}[0m\u{1b}[33m⢱\u{1b}[0m\u{1b}[33m⣱\u{1b}[0m\u{1b}[33m⠕\u{1b}[0m\u{1b}[33m⡕\u{1b}[0m\u{1b}[33m⢕\u{1b}[0m\u{1b}[33m⣕\u{1b}[0m\u{1b}[33m⠵\u{1b}[0m\u{1b}[33m⡵\u{1b}[0m\u{1b}[33m⢵\u{1b}[0m\u{1b}[33m⣵\u{1b}[0m\n\u{1b}[38;5;242m000000e0: \u{1b}[0m\u{1b}[33me0\u{1b}[0m\u{1b}[33me1\u{1b}[0m \u{1b}[33me2\u{1b}[0m\u{1b}[33me3\u{1b}[0m \u{1b}[33me4\u{1b}[0m\u{1b}[33me5\u{1b}[0m \u{1b}[33me6\u{1b}[0m\u{1b}[33me7\u{1b}[0m \u{1b}[33me8\u{1b}[0m\u{1b}[33me9\u{1b}[0m \u{1b}[33mea\u{1b}[0m\u{1b}[33meb\u{1b}[0m \u{1b}[33mec\u{1b}[0m\u{1b}[33med\u{1b}[0m \u{1b}[33mee\u{1b}[0m\u{1b}[33mef\u{1b}[0m \u{1b}[33m⠚\u{1b}[0m\u{1b}[33m⡚\u{1b}[0m\u{1b}[33m⢚\u{1b}[0m\u{1b}[33m⣚\u{1b}[0m\u{1b}[33m⠺\u{1b}[0m\u{1b}[33m⡺\u{1b}[0m\u{1b}[33m⢺\u{1b}[0m\u{1b}[33m⣺\u{1b}[0m\u{1b}[33m⠞\u{1b}[0m\u{1b}[33m⡞\u{1b}[0m\u{1b}[33m⢞\u{1b}[0m\u{1b}[33m⣞\u{1b}[0m\u{1b}[33m⠾\u{1b}[0m\u{1b}[33m⡾\u{1b}[0m\u{1b}[33m⢾\u{1b}[0m\u{1b}[33m⣾\u{1b}[0m\n\u{1b}[38;5;242m000000f0: \u{1b}[0m\u{1b}[33mf0\u{1b}[0m\u{1b}[33mf1\u{1b}[0m \u{1b}[33mf2\u{1b}[0m\u{1b}[33mf3\u{1b}[0m \u{1b}[33mf4\u{1b}[0m\u{1b}[33mf5\u{1b}[0m \u{1b}[33mf6\u{1b}[0m\u{1b}[33mf7\u{1b}[0m \u{1b}[33mf8\u{1b}[0m\u{1b}[33mf9\u{1b}[0m \u{1b}[33mfa\u{1b}[0m\u{1b}[33mfb\u{1b}[0m \u{1b}[33mfc\u{1b}[0m\u{1b}[33mfd\u{1b}[0m \u{1b}[33mfe\u{1b}[0m\u{1b}[33mff\u{1b}[0m \u{1b}[33m⠛\u{1b}[0m\u{1b}[33m⡛\u{1b}[0m\u{1b}[33m⢛\u{1b}[0m\u{1b}[33m⣛\u{1b}[0m\u{1b}[33m⠻\u{1b}[0m\u{1b}[33m⡻\u{1b}[0m\u{1b}[33m⢻\u{1b}[0m\u{1b}[33m⣻\u{1b}[0m\u{1b}[33m⠟\u{1b}[0m\u{1b}[33m⡟\u{1b}[0m\u{1b}[33m⢟\u{1b}[0m\u{1b}[33m⣟\u{1b}[0m\u{1b}[33m⠿\u{1b}[0m\u{1b}[33m⡿\u{1b}[0m\u{1b}[33m⢿\u{1b}[0m\u{1b}[33m⣿\u{1b}[0m\n");
    }
}
