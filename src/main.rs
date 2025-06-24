use std::time::{Duration, Instant};

use chip_8_emulator::*;

fn main() {
    let mut ram: [u8; emulator_data::RAM_SIZE] = [0; emulator_data::RAM_SIZE];

    write_sprite_data(&mut ram);

    let pc_init_idx = emulator_data::FREE_MEM_START as u16;
    let idx_as_bytes = pc_init_idx.to_le_bytes();
    ram[emulator_data::PC_START..=emulator_data::PC_END].copy_from_slice(&idx_as_bytes);

    let instruction_delay = Duration::from_micros(1_000_000 / 700); // limit to 700 instructions
                                                                    // per second
    let timer_interval = Duration::from_millis(1000 / 60); // decrease timer 60 times per
                                                           // second
    let mut last_execution_time = Instant::now();
    //TODO DELAY AND SOUND TIMERS decrement at 60hz

    loop {
        if last_execution_time.elapsed() > instruction_delay {
            let current_pc = &ram[emulator_data::PC_START..=emulator_data::PC_END];
            let instruction_idx = u16::from_le_bytes(current_pc.try_into().unwrap()) as usize;
            update_pc(&mut ram);

            let instruction_as_bytes =
                &ram[instruction_idx..instruction_idx + emulator_data::INSTRUCTION_SIZE];

            let instruction: u16 = u16::from_le_bytes(instruction_as_bytes.try_into().unwrap());

            match instruction & 0xF000 {
                0x0000 => {
                    match instruction & 0x00FF {
                        0x00E0 => todo!(), //TODO CLEAR SCREEN
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
                    todo!() //TODO 3XNN =>  CHECK IF VX IS EQUAL TO NN THEN SKIP ONE INSTRUCTION
                            //TODO MAKE A FUNCTION TO MAP THE EXTRACTED X VALUE TO REGISTER VX
                }

                0x4000 => {
                    todo!() //TODO 4XNN => SAME AS ABOVE BUT VX SHOULD NOT BE EQUAL TO NN
                }

                0x5000 => {
                    todo!() //TODO 5XY0 => CHECK IF VALUES IN VX AND VY ARE EQUAL AND SKIP ONE
                            //INSTRUCTION
                }

                0x6000 => {
                    todo!() //TODO 6XNN => SET VX TO NN
                }

                0x7000 => {
                    todo!() //TODO 7XNN => ADD NN TO VX and wrap around if overflow
                }

                0x8000 => {
                    // 0x8XY_
                    match instruction & 0x000F {
                        0x0000 => todo!(), //TODO SET VX TO VY
                        0x0001 => todo!(), //TODO VX SET TO VX | VY
                        0x0002 => todo!(), //TODO VX SET TO VX & VY
                        0x0003 => todo!(), //TODO VX SET TO VX ^ VY
                        0x0004 => todo!(), //TODO VX SET TO VX + VY, affects the VF CARRY FLAG
                        0x0005 => todo!(), //TODO VX SET TO VX - VY, affects the carry flag
                        0x0006 => todo!(), //TODO SHIFT VX IN PLACE TO RIGHT, SET VF TO SHIFTED BIT
                        0x0007 => todo!(), //TODO VX SET TO VY - VX, affects the carry flag
                        0x000E => todo!(), //TODO SHIFT VX IN PLACE TO LEFT, SET VF TO SHIFTED BIT
                        _ => (),
                    }
                }

                0x9000 => {
                    todo!() //TODO 9XY0 => CHECK IF VALUES IN VX AND VY ARE NOT EQUAL AND SKIP ONE
                            // INSTRUCTION
                }

                0xA000 => {
                    todo!() //TODO ANNN => SET 'I' REGISTER TO NNN
                }

                0xB000 => {
                    todo!() //TODO BNNN => SET PC TO NNN + value in V0
                }

                0xC000 => {
                    todo!() //TODO CXNN => SET VX TO A RANDOM NUMBER &  NN
                }

                0xD000 => {
                    todo!() //TODO DXYN => refer to guide, big ass instruction
                }

                0xE000 => {
                    //0xEX__
                    match instruction & 0x00FF {
                        0x009E => todo!(), //TODO EX9E => SKIP ONE INSTRUCTION IF KEY CORRESPONDING TO VALUE IN VX IS PRESSED
                        0x00A1 => todo!(), //TODO EXA1 => SKIP ONE INSTRUCTION IF KEY CORRESPONDING TO VALUE IN VX IS NOT PRESSED
                        _ => (),           // KEYS RANGE FROM 0 - F
                    }
                }

                0xF000 => {
                    //0xFX__
                    match instruction & 0x00FF {
                        0x0007 => todo!(), //TODO SET VX TO CURRENT VALUE OF DELAY TIMER
                        0x0015 => todo!(), //TODO SET DELAY TIMER TO VALUE IN VX
                        0x0018 => todo!(), //TODO SET SOUND TIMER TO VALUE IN VX
                        0x001E => todo!(), //TODO I = I + VALUE IN VX, overflow of I above 0x0FFF (4096) sets VF to 1
                        0x000A => todo!(), //TODO WAITS FOR KEY INPUT AND BLOCKS BUT TIMERS SHOULD STILL BE DECREASING, SET HEX VALUE OF KEY TO VX
                        0x0029 => todo!(), //TODO SET I TO POINT TO SPRITE DATA OF HEX STORED IN VX
                        0x0033 => todo!(), //TODO TAKE VAL IN VX, CONVERT TO 3 DIGIT DECIMAL, STORE THE DIGIT AT ADDRESS STORED IN I, I+1, I+2
                        0x0055 => todo!(), //TODO STORE V0 to VX in I to I + X (where I is actually pointing to) USE A TEMP VARIABLE INSTEAD OF CHANGING I
                        0x0065 => todo!(), //TODO STORE I to I + X in V0 to VX (reverse of above)  USE A TEMP VARIABLE INSTEAD OF CHANGING I
                        _ => (),
                    }
                }

                _ => todo!(),
            }

            last_execution_time = Instant::now();
        }
    }
}
