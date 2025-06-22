use chip_8_emulator::*;

fn main() {
    let mut ram: [u8; data::RAM_SIZE] = [0; data::RAM_SIZE];

    write_sprite_data(&mut ram);

    for element in &ram[data::SPRITE_DATA_START..=data::SPRITE_DATA_END] {
        print!("{} ", element);
    }
}
