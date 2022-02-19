# CHIP-8-EMULATOR

## INFO
This is a chip-8 emulator that I have coded in Rust by myself.

## REQUIREMENTS
### RUST
The Rust programming language is the only thing required, along with cargo.

## Usage
### CHANGING DIRECTORIES
In order to run this, you must first go into the desktop_frontend folder via the terminal e.g.
```
$ cd desktop_frontend
```
### OBTAINING A CHIP-8 ROM FILE
Then, you must obtain a copy of a chip-8 rom file from online - I would suggest getting PONG2.
This file can be saved anywhere in the project directory, as long as you know the path to it.
### RUNNING THE FILE
After doing this, in order to actually run the file, you need to run the following:
```
$ cargo run <path to rom file>
```
For example, if PONG2 was located in the root directory, this would look like:
```
$ cargo run ../PONG2
```
