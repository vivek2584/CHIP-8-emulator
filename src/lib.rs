pub mod data;

pub fn write_sprite_data(ram: &mut [u8]) {
    let mut current_pos: usize = data::SPRITE_DATA_START;
    for sprite in data::SPRITE_DATA.iter() {
        ram[current_pos..current_pos + data::SPRITE_FONT_SIZE].copy_from_slice(sprite);
        current_pos += data::SPRITE_FONT_SIZE;
    }
}
