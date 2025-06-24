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
                        0x00EE => todo!(), //TODO RETURN FROM SUBROUTINE
                        _ => (),           //IGNORE 0x0NNN,
                    }
                }

                0x1000 => {
                    todo!() // TODO EXTRACT NNN FROM 1NNN AND SET PC TO NNN, DO NOT INCREMENT PC
                }

                0x2000 => {
                    todo!() // TODO SAME AS 1NNN but first push current pc value to stack to return
                            // with 00EE
                }

                0x3000 => {
                    todo!() //TODO 3XNN =>  CHECK IF VX IS EQUAL TO NN THEN SKIP ONE INSTRUCTION
                            //TODO MAKE A FUNCTION TO MAP THE EXTRACTED X VALUE TO REGISTER VX
                }

                0x4000 => {
                    todo!() //TODO 4XNN => SAME AS ABOVE BUT VX SHOULD NOT BE EQUAL TO NN
                }

                0x5000 => {
                    todo!() // 5XY0 => CHECK IF VALUES IN VX AND VY ARE EQUAL AND SKIP ONE
                            // INSTRUCTION
                }

                0x6000 => {
                    todo!() // 6XNN => SET VX TO NN
                }

                0x7000 => {
                    todo!() // 7XNN => ADD NN TO VX and wrap around if overflow
                }

                0x8000 => {}

                0x9000 => {
                    todo!() // 9XY0 => CHECK IF VALUES IN VX AND VY ARE NOT EQUAL AND SKIP ONE
                            // INSTRUCTION
                }

                _ => todo!(),
            }

            last_execution_time = Instant::now();
        }
    }
}
