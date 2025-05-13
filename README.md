# Colored-Hexdump

Create beautifuly colored hexdump in Rust.

## Hexdump

![`colored_hexdump::hexdump()`](./images/hexdump.png)

Use `colored_hexdump::hexdump()` to create an hexdump with borders.

```Rust
let all_bytes: Vec<u8> = (0..=u8::MAX).collect();
let hex_colors = hexdump(&all_bytes);
println!("{}", hex_colors);
```

## xxd

You can also go with a more classic `xxd` style with

```Rust
colored_hexdump::xxr()
```

![`colored_hexdump::xxr()`](./images/xxr.png)

```Rust
let all_bytes: Vec<u8> = (0..=u8::MAX).collect();
let hex_colors = xxd(&all_bytes);
println!("{}", hex_colors);
```