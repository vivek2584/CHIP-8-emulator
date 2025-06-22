use std::time::{Duration, Instant};

use chip_8_emulator::*;

fn main() {
    let mut ram: [u8; emulator_data::RAM_SIZE] = [0; emulator_data::RAM_SIZE];

    write_sprite_data(&mut ram);

    let instruction_delay = Duration::from_micros(1_000_000 / 700);
    let timer_interval = Duration::from_millis(1000 / 60);

    let mut last_execution_time = Instant::now();
    //TODO DELAY AND SOUND TIMERS delays

    loop {
        if last_execution_time.elapsed() > instruction_delay {
            //TODO DECODE AND EXECUTE
        }
        last_execution_time = Instant::now();
    }
}
