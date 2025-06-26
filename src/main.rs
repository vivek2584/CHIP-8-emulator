use chip_8_emulator::*;
use minifb::*;
use rand::*;
use std::time::{Duration, Instant};

fn main() {
    let mut ram: [u8; emulator_data::RAM_SIZE] = [0; emulator_data::RAM_SIZE];
    write_sprite_data(&mut ram);

    let pc_init_idx = emulator_data::FREE_MEM_START as u16;
    let idx_as_bytes = pc_init_idx.to_le_bytes();
    ram[emulator_data::PC_START..=emulator_data::PC_END].copy_from_slice(&idx_as_bytes);

    let mut display_buffer: Vec<u32> =
        vec![0; emulator_data::DISPLAY_WIDTH * emulator_data::DISPLAY_HEIGHT];

    let mut window = Window::new(
        "CHIP-8",
        emulator_data::DISPLAY_WIDTH,
        emulator_data::DISPLAY_HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    //window.set_target_fps(60);   dont use this as it throttles the instruction execution speed
    window
        .update_with_buffer(
            &display_buffer,
            emulator_data::DISPLAY_WIDTH,
            emulator_data::DISPLAY_HEIGHT,
        )
        .unwrap();

    let instruction_delay = Duration::from_micros(1000000 / 700);
    let delay_60hz = Duration::from_micros(1000000 / 60);
    let mut last_execution_time = Instant::now();
    let mut last_delay_decr = Instant::now();
    let mut last_sound_decr = Instant::now();
    let mut last_display_update = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if last_execution_time.elapsed() > instruction_delay {
            let current_pc = &ram[emulator_data::PC_START..=emulator_data::PC_END];
            let instruction_idx = u16::from_le_bytes(current_pc.try_into().unwrap()) as usize;
            println!("{}", instruction_idx);
            increment_pc(&mut ram);
            let instruction_as_bytes =
                &ram[instruction_idx..instruction_idx + emulator_data::INSTRUCTION_SIZE];
            let instruction: u16 = u16::from_le_bytes(instruction_as_bytes.try_into().unwrap());

            match instruction & 0xF000 {
                0x0000 => {
                    match instruction & 0x00FF {
                        0x00E0 => {
                            display_buffer.fill(0);
                        }
                        0x00EE => {
                            let mut sp = ram[emulator_data::STACK_PTR_LOC] as usize;
                            let addr = &ram[(emulator_data::STACK_START + sp - 2)
                                ..(emulator_data::STACK_START + sp)];
                            let owned: [u8; 2] = addr.try_into().unwrap();
                            ram[emulator_data::PC_START..=emulator_data::PC_END]
                                .copy_from_slice(&owned);
                            sp -= 2;
                            ram[emulator_data::STACK_PTR_LOC] = sp as u8;
                        }
                        _ => (), //IGNORE 0x0NNN,
                    }
                }

                0x1000 => {
                    let jump_to: u16 = instruction & 0x0FFF;
                    set_pc(&mut ram, jump_to);
                }

                0x2000 => {
                    let pc_ref = &ram[emulator_data::PC_START..=emulator_data::PC_END];
                    let owned: [u8; 2] = pc_ref.try_into().unwrap();
                    let mut sp = ram[emulator_data::STACK_PTR_LOC] as usize;
                    ram[(emulator_data::STACK_START + sp)..=(emulator_data::STACK_START + sp + 1)]
                        .copy_from_slice(&owned);
                    sp += 2;
                    ram[emulator_data::STACK_PTR_LOC] = sp as u8;
                    let jump_to: u16 = instruction & 0x0FFF;
                    set_pc(&mut ram, jump_to);
                }

                0x3000 => {
                    let X = ((instruction & 0x0F00) >> 8) as usize;
                    let VX = ram[emulator_data::GPR_START_V0 + X];
                    let NN = (instruction & 0x00FF) as u8;
                    if VX == NN {
                        increment_pc(&mut ram);
                    }
                }

                0x4000 => {
                    let X = ((instruction & 0x0F00) >> 8) as usize;
                    let VX = ram[emulator_data::GPR_START_V0 + X];
                    let NN = (instruction & 0x00FF) as u8;
                    if VX != NN {
                        increment_pc(&mut ram);
                    }
                }

                0x5000 => {
                    let X = ((instruction & 0x0F00) >> 8) as usize;
                    let VX = ram[emulator_data::GPR_START_V0 + X];
                    let Y = ((instruction & 0x00F0) >> 4) as usize;
                    let VY = ram[emulator_data::GPR_START_V0 + Y];
                    if VX == VY {
                        increment_pc(&mut ram);
                    }
                }

                0x6000 => {
                    let X = ((instruction & 0x0F00) >> 8) as usize;
                    let NN = (instruction & 0x00FF) as u8;
                    ram[emulator_data::GPR_START_V0 + X] = NN;
                }

                0x7000 => {
                    let X = ((instruction & 0x0F00) >> 8) as usize;
                    let NN = (instruction & 0x00FF) as u8;
                    let mut VX = ram[emulator_data::GPR_START_V0 + X];
                    VX = VX.wrapping_add(NN);
                    ram[emulator_data::GPR_START_V0 + X] = VX;
                }

                0x8000 => {
                    // 0x8XY_
                    match instruction & 0x000F {
                        0x0000 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let Y = ((instruction & 0x00F0) >> 4) as usize;
                            let VY = ram[emulator_data::GPR_START_V0 + Y];
                            ram[emulator_data::GPR_START_V0 + X] = VY;
                        }
                        0x0001 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let VX = ram[emulator_data::GPR_START_V0 + X];
                            let Y = ((instruction & 0x00F0) >> 4) as usize;
                            let VY = ram[emulator_data::GPR_START_V0 + Y];
                            ram[emulator_data::GPR_START_V0 + X] = VX | VY;
                        }
                        0x0002 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let VX = ram[emulator_data::GPR_START_V0 + X];
                            let Y = ((instruction & 0x00F0) >> 4) as usize;
                            let VY = ram[emulator_data::GPR_START_V0 + Y];
                            ram[emulator_data::GPR_START_V0 + X] = VX & VY;
                        }
                        0x0003 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let VX = ram[emulator_data::GPR_START_V0 + X];
                            let Y = ((instruction & 0x00F0) >> 4) as usize;
                            let VY = ram[emulator_data::GPR_START_V0 + Y];
                            ram[emulator_data::GPR_START_V0 + X] = VX ^ VY;
                        }
                        0x0004 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let VX = ram[emulator_data::GPR_START_V0 + X];
                            let Y = ((instruction & 0x00F0) >> 4) as usize;
                            let VY = ram[emulator_data::GPR_START_V0 + Y];
                            let (res, carry) = VX.overflowing_add(VY);
                            ram[emulator_data::GPR_START_V0 + X] = res;
                            ram[emulator_data::GPR_END_VF] = carry as u8;
                        }
                        0x0005 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let VX = ram[emulator_data::GPR_START_V0 + X];
                            let Y = ((instruction & 0x00F0) >> 4) as usize;
                            let VY = ram[emulator_data::GPR_START_V0 + Y];
                            let (res, borrow) = VX.overflowing_sub(VY);
                            ram[emulator_data::GPR_START_V0 + X] = res;
                            ram[emulator_data::GPR_END_VF] = (!borrow) as u8;
                        }
                        0x0006 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let mut VX = ram[emulator_data::GPR_START_V0 + X];
                            ram[emulator_data::GPR_END_VF] = VX & 0x1;
                            VX >>= 1;
                            ram[emulator_data::GPR_START_V0 + X] = VX;
                        }
                        0x0007 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let VX = ram[emulator_data::GPR_START_V0 + X];
                            let Y = ((instruction & 0x00F0) >> 4) as usize;
                            let VY = ram[emulator_data::GPR_START_V0 + Y];
                            let (res, borrow) = VY.overflowing_sub(VX);
                            ram[emulator_data::GPR_START_V0 + X] = res;
                            ram[emulator_data::GPR_END_VF] = (!borrow) as u8;
                        }
                        0x000E => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let mut VX = ram[emulator_data::GPR_START_V0 + X];
                            ram[emulator_data::GPR_END_VF] = VX >> 7;
                            VX <<= 1;
                            ram[emulator_data::GPR_START_V0 + X] = VX;
                        }
                        _ => (),
                    }
                }

                0x9000 => {
                    let X = ((instruction & 0x0F00) >> 8) as usize;
                    let VX = ram[emulator_data::GPR_START_V0 + X];
                    let Y = ((instruction & 0x00F0) >> 4) as usize;
                    let VY = ram[emulator_data::GPR_START_V0 + Y];
                    if VX != VY {
                        increment_pc(&mut ram);
                    }
                }

                0xA000 => {
                    let NNN = instruction & 0x0FFF;
                    let NNN_as_bytes = NNN.to_le_bytes();
                    ram[emulator_data::I_START..=emulator_data::I_END]
                        .copy_from_slice(&NNN_as_bytes);
                }

                0xB000 => {
                    let V0 = ram[emulator_data::GPR_START_V0] as u16;
                    let NNN = instruction & 0x0FFF;
                    let PC = NNN + V0;
                    let PC_as_bytes = PC.to_le_bytes();
                    ram[emulator_data::PC_START..=emulator_data::PC_END]
                        .copy_from_slice(&PC_as_bytes);
                }

                0xC000 => {
                    let NN = (instruction & 0x00FF) as u8;
                    let X = ((instruction & 0x0F00) >> 8) as usize;
                    let mut rng = rand::rng();
                    let rand_u8 = rng.random_range(0..=u8::MAX);
                    ram[emulator_data::GPR_START_V0 + X] = NN & rand_u8;
                }

                0xD000 => {
                    todo!() //TODO DXYN => refer to guide, big ass instruction
                }

                0xE000 => {
                    //0xEX__
                    match instruction & 0x00FF {
                        0x009E => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let key_hex = ram[emulator_data::GPR_START_V0 + X];
                            if let Some(key) = key_mappings::get_key(key_hex) {
                                if window.is_key_down(key) {
                                    increment_pc(&mut ram);
                                }
                            }
                        }
                        0x00A1 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let key_hex = ram[emulator_data::GPR_START_V0 + X];
                            if let Some(key) = key_mappings::get_key(key_hex) {
                                if !window.is_key_down(key) {
                                    increment_pc(&mut ram);
                                }
                            }
                        }
                        _ => (),
                    }
                }

                0xF000 => {
                    //0xFX__
                    match instruction & 0x00FF {
                        0x0007 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            ram[emulator_data::GPR_START_V0 + X] =
                                ram[emulator_data::DELAY_TIMER_LOC];
                        }
                        0x0015 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            ram[emulator_data::DELAY_TIMER_LOC] =
                                ram[emulator_data::GPR_START_V0 + X];
                        }
                        0x0018 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            ram[emulator_data::SOUND_TIMER_LOC] =
                                ram[emulator_data::GPR_START_V0 + X];
                        }
                        0x001E => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let VX = ram[emulator_data::GPR_START_V0 + X] as u16;
                            let I = &ram[emulator_data::I_START..=emulator_data::I_END];
                            let owned: [u8; 2] = I.try_into().unwrap();
                            let mut I_as_u16 = u16::from_le_bytes(owned);
                            I_as_u16 += VX;
                            let new_I_as_bytes = I_as_u16.to_le_bytes();
                            ram[emulator_data::I_START..=emulator_data::I_END]
                                .copy_from_slice(&new_I_as_bytes);
                            ram[emulator_data::GPR_END_VF] = (I_as_u16 > 0x0FFF) as u8;
                            // ^ AMBIGUOUS
                        }
                        0x000A => todo!(), //TODO WAITS FOR KEY INPUT AND BLOCKS BUT TIMERS SHOULD STILL BE DECREASING, SET HEX VALUE OF KEY TO VX
                        0x0029 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let sprite = ram[emulator_data::GPR_START_V0 + X] as usize;
                            let sprite_loc = (emulator_data::SPRITE_DATA_START
                                + sprite * emulator_data::SPRITE_FONT_SIZE)
                                as u16;
                            let sprite_loc_as_bytes = sprite_loc.to_le_bytes();
                            ram[emulator_data::I_START..=emulator_data::I_END]
                                .copy_from_slice(&sprite_loc_as_bytes);
                        }
                        0x0033 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let VX = ram[emulator_data::GPR_START_V0 + X];
                            let ref_I = &ram[emulator_data::I_START..=emulator_data::I_END];
                            let owned: [u8; 2] = ref_I.try_into().unwrap();
                            let I_usize = u16::from_le_bytes(owned) as usize;
                            let BCD: [u8; 3] = [VX / 100, (VX % 100) / 10, VX % 10];
                            ram[I_usize..=(I_usize + 2)].copy_from_slice(&BCD);
                        }
                        0x0055 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let ref_I = &ram[emulator_data::I_START..=emulator_data::I_END];
                            let owned: [u8; 2] = ref_I.try_into().unwrap();
                            let I_usize = u16::from_le_bytes(owned) as usize;
                            let register_bytes = &ram
                                [emulator_data::GPR_START_V0..=(emulator_data::GPR_START_V0 + X)];
                            let owned = register_bytes.to_vec();
                            ram[I_usize..=(I_usize + X)].copy_from_slice(&owned[..]);
                        }
                        0x0065 => {
                            let X = ((instruction & 0x0F00) >> 8) as usize;
                            let ref_I = &ram[emulator_data::I_START..=emulator_data::I_END];
                            let owned: [u8; 2] = ref_I.try_into().unwrap();
                            let I_usize = u16::from_le_bytes(owned) as usize;
                            let I_bytes = &ram[I_usize..=(I_usize + X)];
                            let owned = I_bytes.to_vec();
                            ram[emulator_data::GPR_START_V0..=(emulator_data::GPR_START_V0 + X)]
                                .copy_from_slice(&owned[..]);
                        }
                        _ => (),
                    }
                }

                _ => (),
            }

            last_execution_time = Instant::now();
        }

        if (ram[emulator_data::DELAY_TIMER_LOC] > 0) && (last_delay_decr.elapsed() > delay_60hz) {
            ram[emulator_data::DELAY_TIMER_LOC] -= 1;
            last_delay_decr = Instant::now();
        }

        if (ram[emulator_data::SOUND_TIMER_LOC] > 0) && (last_sound_decr.elapsed() > delay_60hz) {
            ram[emulator_data::SOUND_TIMER_LOC] -= 1;
            last_sound_decr = Instant::now();
        }

        if last_display_update.elapsed() > delay_60hz {
            window
                .update_with_buffer(
                    &display_buffer,
                    emulator_data::DISPLAY_WIDTH,
                    emulator_data::DISPLAY_HEIGHT,
                )
                .unwrap();
            last_display_update = Instant::now();
        }
    }
}
