// THIS MODULE DEFINES THE LOCATIONS IN RAM FOR ALL CHIP-8 REGISTERS

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;

pub const RAM_SIZE: usize = 4096;

pub const SPRITE_DATA_START: usize = 0x050;
pub const SPRITE_DATA_END: usize = 0x09F;
pub const SPRITE_FONT_SIZE: usize = 5;

pub const GPR_START_V0: usize = 0x200;
pub const GPR_END_VF: usize = 0x20F;
pub const GPR_SIZE: usize = 1;
pub const GPR_COUNT: usize = 16;

pub const STACK_START: usize = 0x210;
pub const STACK_END: usize = 0x21F;
pub const STACK_SIZE: usize = 16;

pub const STACK_PTR_LOC: usize = 0x224;
pub const STACK_PTR_SIZE: usize = 1;

pub const I_START: usize = 0x220;
pub const I_END: usize = 0x221;
pub const I_SIZE: usize = 2;

pub const PC_START: usize = 0x222;
pub const PC_END: usize = 0x223;
pub const PC_SIZE: usize = 2;

pub const DELAY_TIMER_LOC: usize = 0x225;
pub const DELAY_TIMER_SIZE: usize = 1;

pub const SOUND_TIMER_LOC: usize = 0x226;
pub const SOUND_TIMER_SIZE: usize = 1;

pub const FREE_MEM_START: usize = 0x227;
pub const FREE_MEM_END: usize = RAM_SIZE - 1;

pub const INSTRUCTION_SIZE: usize = 2;
