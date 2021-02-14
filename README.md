# Niccalc
![Crates.io](https://img.shields.io/crates/v/niccalc.svg)
![Rust](https://github.com/geri1701/niccalc/workflows/Rust/badge.svg)

Niccalc is a tool that helps to determine the necessary amount of nicotine for an e-cigarette liquid,
all you have to do is enter the corresponding values in the input fields.
The amount of flavor is optional.

This software is written in Rust using the Rust bindings for the FLTK Graphical User Interface library [fltk-rs](https://crates.io/crates/fltk)
and [comfy-table](https://crates.io/crates/comfy-table).

![A screenshot](https://raw.githubusercontent.com/geri1701/niccalc/master/screenshots/niccalc_scrsh.png)

## Installation

### Linux and other

First install cargo.

Now, compile the niccalc-crate:

```
cargo install niccalc
```
### Windows

For all Windows users who are not able to compile something themselves or who are already too lazy to do so, I have prepared a binary:

Download:
[win-executable](https://github.com/geri1701/niccalc/blob/master/binary/niccalc.exe)
