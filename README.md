# Colored-Hexdump

Create beautifuly colored hexdump in Rust.

## Add to your project

```bash
cargo add colored-hexdump
```

## Hexdump

![`colored_hexdump::hexdump()`](./images/hexdump.png)

Use `colored_hexdump::hexdump()` to create an hexdump with borders.

```Rust
use colored_hexdump::hexdump;

fn main() {
    // All possible bytes
    let all_bytes: Vec<u8> = (0..=u8::MAX).collect();

    // Create hexdump, and print it to stdout
    let hexdump = hexdump(&all_bytes);
    println!("{}", hexdump);
}
```

## xxd

You can also go with a more classic `xxd` style with `colored_hexdump::xxd()`.

![`colored_hexdump::xxd()`](./images/xxr.png)

```Rust
use colored_hexdump::xxd;

fn main() {
    // All possible bytes
    let all_bytes: Vec<u8> = (0..=u8::MAX).collect();

    // Create hexdump, and print it to stdout
    let hexdump = xxd(&all_bytes);
    println!("{}", hexdump);
}
```