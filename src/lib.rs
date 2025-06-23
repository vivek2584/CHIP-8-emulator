pub mod emulator_data;
pub mod sprite_data;

pub fn write_sprite_data(ram: &mut [u8]) {
    let mut current_pos: usize = emulator_data::SPRITE_DATA_START;
    for sprite in sprite_data::SPRITE_DATA.iter() {
        ram[current_pos..current_pos + emulator_data::SPRITE_FONT_SIZE].copy_from_slice(sprite);
        current_pos += sprite_data::SPRITE_FONT_SIZE;
    }
}

pub fn update_pc(ram: &mut [u8]) {
    let current_ptr = &ram[emulator_data::PC_START..=emulator_data::PC_END];
    let mut current_as_u16 = u16::from_le_bytes(current_ptr.try_into().unwrap());
    current_as_u16 += emulator_data::INSTRUCTION_SIZE as u16;
    let new_ptr = current_as_u16.to_ne_bytes();
    ram[emulator_data::PC_START..=emulator_data::PC_END].copy_from_slice(&new_ptr);
}
