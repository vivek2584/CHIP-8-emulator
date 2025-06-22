pub mod emulator_data;
pub mod sprite_data;

pub fn write_sprite_data(ram: &mut [u8]) {
    let mut current_pos: usize = emulator_data::SPRITE_DATA_START;
    for sprite in sprite_data::SPRITE_DATA.iter() {
        ram[current_pos..current_pos + emulator_data::SPRITE_FONT_SIZE].copy_from_slice(sprite);
        current_pos += sprite_data::SPRITE_FONT_SIZE;
    }
}
