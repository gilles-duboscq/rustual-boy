#![allow(dead_code)]

pub const VRAM_START: u32 = 0x00000000;
pub const VRAM_LENGTH: u32 = 0x00040000;
pub const VRAM_END: u32 = VRAM_START + VRAM_LENGTH - 1;

pub const LEFT_FRAME_BUFFER_0_START: u32 = 0x00000000;
pub const LEFT_FRAME_BUFFER_0_LENGTH: u32 = 0x00006000;
pub const LEFT_FRAME_BUFFER_0_END: u32 = LEFT_FRAME_BUFFER_0_START + LEFT_FRAME_BUFFER_0_LENGTH - 1;

pub const CHR_RAM_PATTERN_TABLE_0_START: u32 = 0x00006000;
pub const CHR_RAM_PATTERN_TABLE_0_LENGTH: u32 = 0x00002000;
pub const CHR_RAM_PATTERN_TABLE_0_END: u32 = CHR_RAM_PATTERN_TABLE_0_START + CHR_RAM_PATTERN_TABLE_0_LENGTH - 1;

pub const BG_SEGMENTS_AND_WINDOW_PARAM_TABLE_START: u32 = 0x00020000;
pub const BG_SEGMENTS_AND_WINDOW_PARAM_TABLE_LENGTH: u32 = 0x0001d800;
pub const BG_SEGMENTS_AND_WINDOW_PARAM_TABLE_END: u32 = BG_SEGMENTS_AND_WINDOW_PARAM_TABLE_START + BG_SEGMENTS_AND_WINDOW_PARAM_TABLE_LENGTH - 1;

pub const WINDOW_ATTRIBS_START: u32 = 0x0003d800;
pub const WINDOW_ATTRIBS_LENGTH: u32 = 0x00000400;
pub const WINDOW_ATTRIBS_END: u32 = WINDOW_ATTRIBS_START + WINDOW_ATTRIBS_LENGTH - 1;

pub const COLUMN_TABLE_START: u32 = 0x0003dc00;
pub const COLUMN_TABLE_LENGTH: u32 = 0x00000400;
pub const COLUMN_TABLE_END: u32 = COLUMN_TABLE_START + COLUMN_TABLE_LENGTH - 1;

pub const OAM_START: u32 = 0x0003e000;
pub const OAM_LENGTH: u32 = 0x00002000;
pub const OAM_END: u32 = OAM_START + OAM_LENGTH - 1;

pub enum MappedAddress {
    Vram(u32),
}

pub fn map_address(addr: u32) -> MappedAddress {
    let addr = addr & 0x07ffffff;
    match addr {
        VRAM_START ... VRAM_END => MappedAddress::Vram(addr - VRAM_START),

        _ => panic!("Unrecognized addr: 0x{:08x}", addr)
    }
}