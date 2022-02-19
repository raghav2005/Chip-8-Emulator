# CHIP-8-EMULATOR

## INFO
This is a chip-8 emulator that I have coded in Rust by myself.

## REQUIREMENTS
### RUST
The Rust programming language is the only thing required, along with cargo.

## Usage
1. In order to run this, you must first go into the desktop_frontend folder via the terminal e.g.
```
$ cd desktop_frontend
```

2. Then, you must obtain a copy of a chip-8 rom file.

3. After doing this, in order to actually run the file, you need to run the following:
```
$ cargo run <path to rom file>
```
for example, if PONG2 was located in the root directory, this would look like:
```
$ cargo run ../PONG2
```
