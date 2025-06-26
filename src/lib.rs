pub mod emulator_data;
pub mod key_mappings;
pub mod sprite_data;

pub fn write_sprite_data(ram: &mut [u8]) {
    let mut current_pos: usize = emulator_data::SPRITE_DATA_START;
    for sprite in sprite_data::SPRITE_DATA.iter() {
        ram[current_pos..current_pos + emulator_data::SPRITE_FONT_SIZE].copy_from_slice(sprite);
        current_pos += emulator_data::SPRITE_FONT_SIZE;
    }
}

pub fn increment_pc(ram: &mut [u8]) {
    let current_ptr = &ram[emulator_data::PC_START..=emulator_data::PC_END];
    let mut current_as_u16 = u16::from_le_bytes(current_ptr.try_into().unwrap());
    current_as_u16 += emulator_data::INSTRUCTION_SIZE as u16;
    let ptr_as_bytes = current_as_u16.to_le_bytes();
    ram[emulator_data::PC_START..=emulator_data::PC_END].copy_from_slice(&ptr_as_bytes);
}

pub fn decrement_pc(ram: &mut [u8]) {
    let current_ptr = &ram[emulator_data::PC_START..=emulator_data::PC_END];
    let mut current_as_u16 = u16::from_le_bytes(current_ptr.try_into().unwrap());
    current_as_u16 -= emulator_data::INSTRUCTION_SIZE as u16;
    let ptr_as_bytes = current_as_u16.to_le_bytes();
    ram[emulator_data::PC_START..=emulator_data::PC_END].copy_from_slice(&ptr_as_bytes);
}

pub fn set_pc(ram: &mut [u8], target: u16) {
    let target_as_bytes = target.to_le_bytes();
    ram[emulator_data::PC_START..=emulator_data::PC_END].copy_from_slice(&target_as_bytes);
}

pub fn upscale_display_buffer(display_buffer: &[u32], upscaled_buffer: &mut [u32], scale: usize) {
    let width = emulator_data::DISPLAY_WIDTH;
    let height = emulator_data::DISPLAY_HEIGHT;

    for y in 0..height {
        for x in 0..width {
            let color = display_buffer[y * width + x];
            for dy in 0..scale {
                for dx in 0..scale {
                    let up_x = x * scale + dx;
                    let up_y = y * scale + dy;
                    upscaled_buffer[up_y * width * scale + up_x] = color;
                }
            }
        }
    }
}
