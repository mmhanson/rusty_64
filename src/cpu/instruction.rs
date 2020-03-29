use super::opcode::Opcode;

pub struct Instruction(pub u32);

extern crate num;
use num::FromPrimitive;

impl Instruction
{
    pub fn opcode(&self) -> Opcode
    {
        Opcode::from_u32((self.0 >> 26) & 0b111111).unwrap_or_else(
            || panic!("Unrecognized instruction: {:#x}", self.0))
    }

    // source register index
    #[inline(always)]
    pub fn rs(&self) -> u32
    {
        (self.0 >> 21) & 0b11111
    }

    // target register index
    #[inline(always)]
    pub fn rt(&self) -> u32
    {
        (self.0 >> 16) & 0b11111
    }

    #[inline(always)]
    pub fn rd(&self) -> u32
    {
        (self.0 >> 11) & 0b11111
    }

    #[inline(always)]
    pub fn imm(&self) -> u32
    {
        self.0 & 0xffff
    }

    #[inline(always)]
    pub fn offset(&self) -> u32
    {
        self.imm()
    }
}
