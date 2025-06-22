use chip_8_emulator::*;

fn main() {
    let mut ram: [u8; emulator_data::RAM_SIZE] = [0; emulator_data::RAM_SIZE];

    write_sprite_data(&mut ram);
}
