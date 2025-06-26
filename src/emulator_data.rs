// THIS MODULE DEFINES THE LOCATIONS IN RAM FOR ALL CHIP-8 REGISTERS
pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const SCALE: usize = 10;

pub const RAM_SIZE: usize = 4096;

pub const SPRITE_DATA_START: usize = 0x050;
pub const SPRITE_DATA_END: usize = 0x09F;
pub const SPRITE_FONT_SIZE: usize = 5;

pub const GPR_START_V0: usize = 0x1D9;
pub const GPR_END_VF: usize = 0x1E8;
pub const GPR_SIZE: usize = 1;
pub const GPR_COUNT: usize = 16;

pub const STACK_START: usize = 0x1E9;
pub const STACK_END: usize = 0x1F8;
pub const STACK_SIZE: usize = 16;

pub const STACK_PTR_LOC: usize = 0x1F9;
pub const STACK_PTR_SIZE: usize = 1;

pub const I_START: usize = 0x1FA;
pub const I_END: usize = 0x1FB;
pub const I_SIZE: usize = 2;

pub const PC_START: usize = 0x1FC;
pub const PC_END: usize = 0x1FD;
pub const PC_SIZE: usize = 2;

pub const DELAY_TIMER_LOC: usize = 0x1FE;
pub const DELAY_TIMER_SIZE: usize = 1;

pub const SOUND_TIMER_LOC: usize = 0x1FF;
pub const SOUND_TIMER_SIZE: usize = 1;

pub const FREE_MEM_START: usize = 0x200;
pub const FREE_MEM_END: usize = RAM_SIZE - 1;

pub const INSTRUCTION_SIZE: usize = 2;
