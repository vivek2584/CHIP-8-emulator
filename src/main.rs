use std::{
    thread::current,
    time::{Duration, Instant},
};

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
    //TODO DELAY AND SOUND TIMERS delays

    loop {
        if last_execution_time.elapsed() > instruction_delay {
            let current_pc = &ram[emulator_data::PC_START..=emulator_data::PC_END];
            let instruction_idx = u16::from_le_bytes(current_pc.try_into().unwrap());
            // TODO fetch from instruction_idx the instruction and execute
            update_pc(&mut ram);
            last_execution_time = Instant::now();
        }
    }
}
