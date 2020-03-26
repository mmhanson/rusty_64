use super::byteorder::{BigEndian, ByteOrder};
use std::fmt;

const PIF_ROM_SIZE: usize = 2048;
const RAM_SIZE: usize = 4 * 1024 * 1024;

pub struct Interconnect
{
    pif_rom: Vec<u8>,
    // TODO change to box. Used vec b/c box syntax unstable
    //   box is better because its not resizable
    ram: Vec<u16>, // b/c theres a 9th bit for rasterizer?
}

impl Interconnect
{
    pub fn new(pif_rom: Vec<u8>) -> Interconnect
    {
        Interconnect
        {
            pif_rom: pif_rom,
            ram: vec![0; RAM_SIZE],
        }
    }

    pub fn read_word(&self, addr: u32) -> u32
    {
        // TODO bring magic numbers to constants
        // pifrom range
        if (addr >= 0x1fc0_0000 && addr < 0x1fc0_07c0)
        {
            let rel_addr = addr - 0x1fc0_0000;

            /*
            // TODO verify endianness
            // TODO use byteorder crate?
            ((self.pif_rom[rel_addr as usize] as u32) << 24) |
            ((self.pif_rom[(rel_addr + 1) as usize] as u32) << 16) |
            ((self.pif_rom[(rel_addr + 2) as usize] as u32) << 8) |
            (self.pif_rom[(rel_addr + 3) as usize] as u32)
            */
            BigEndian::read_u32(&self.pif_rom[rel_addr as usize..])
        }
        else
        {
            panic!("Unrecognized physical address: {:#x}", addr);
        }
    }
}

impl fmt::Debug for Interconnect
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "TODO impl Debug for Interconnect")
    }
}
