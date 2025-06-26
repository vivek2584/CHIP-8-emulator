# CHIP-8 Emulator

A CHIP-8 emulator for the COSMAC VIP computer written in Rust.

## Dependencies:-

- `minifb` - for display and keyboard input management


## How to build :-
```bash
cargo build --release
```

## Usage:-


```bash
cargo run roms/"rom_name"
```

## Key mapping:-

CHIP-8 keypad is mapped to the following keyboard keys:


```
CHIP-8          Keyboard
1 2 3 C          1 2 3 4
4 5 6 D   ->     Q W E R
7 8 9 E          A S D F
A 0 B F          Z X C V
```

- esc: stop emulator

## Memory Layout:-

everything is stored inside a 4KB buffer (even registers, ironic isn't it)

- `0x000-0x04F`: Empty
- `0x050-0x09F`: Font data (5 bytes per character, 16 characters)
- `0x0A0-0x1D8`: Unused
- `0x1D9-0x1E8`: V0-VF general purpose registers
- `0x1E9-0x1F8`: Stack (16 bytes)
- `0x1F9`: Stack pointer
- `0x1FA-0x1FB`: I register
- `0x1FC-0x1FD`: Program counter
- `0x1FE`: Delay timer
- `0x1FF`: Sound timer
- `0x200-0xFFF`: Program/ROM area

## Display:-

- 64x32 pixels scaled to 640x320 with 60hz refresh rate
- Color : monochrome (white on black)

## ROMS:- 

Some ROMS already in /roms/ dir . You can find more Chip-8 ROMs online (e.g., from https://github.com/dmatlack/chip8)

## File Structure :-

```
src/
├── main.rs              # main emulator loop
├── lib.rs               # helper functions
├── emulator_data.rs     # memory layout constants
├── key_mappings.rs      # keyboard mapping functions
└── sprite_data.rs       # font sprite data
```
