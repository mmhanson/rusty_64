use super::super::interconnect;
use super::cp0::cp0;
use super::opcode::Opcode::*;
use super::instruction::Instruction;

use std::fmt;

const NUM_GPR: usize = 32; // number of general purpose registers

// a NEC VR-4300 MIPS CPU
pub struct Cpu
{
    // cpu registers (see datasheet: 1.4.2)
    reg_gpr: [u64; NUM_GPR],
    reg_fpr: [f64; NUM_GPR], // floating point
    reg_pc: u64,
    reg_hi: u64,
    reg_lo: u64,
    reg_llbit: bool, // TODO enum type?
    reg_fcr0: u32,
    reg_fcr31: u32,

    cp0: cp0::Cp0, // cpu contains cp0
    interconnect: interconnect::Interconnect,
}

impl Cpu
{
    pub fn new(interconnect: interconnect::Interconnect) -> Cpu
    {
        Cpu
        {
            reg_gpr: [0; NUM_GPR],
            reg_fpr: [0.0; NUM_GPR],
            reg_pc: 0,
            reg_hi: 0,
            reg_lo: 0,
            reg_llbit: false,
            reg_fcr0: 0,
            reg_fcr31: 0,
            cp0: cp0::Cp0::default(),
            interconnect: interconnect,
        }
    }
    // also 'hard_reset'
    pub fn power_on_reset (&mut self)
    {
        // TODO
        self.cp0.power_on_reset();

        // see datasheet p136
        self.reg_pc = 0xffff_ffff_bfc0_0000; // TODO move to const later
    }

    pub fn run(&mut self)
    {
        loop
        {
            self.run_instruction();
        }
    }

    pub fn run_instruction(&mut self)
    {
        let instr = Instruction(self.read_word(self.reg_pc));

        match instr.opcode()
        {
            Andi =>
            {
                // ANDI
                let res = self.read_reg_gpr(instr.rs() as usize) & (instr.imm() as u64);
                self.write_reg_gpr(instr.rt() as usize, res);
            }
            Ori =>
            {
                // ORI
                let res = self.read_reg_gpr(instr.rs() as usize) | (instr.imm() as u64);
                self.write_reg_gpr(instr.rt() as usize, res);
            }
            Lui =>
            {
                // LUI
                let value = (((instr.imm() << 16) as i32) as u64); // sign extend upper 32 bits
                self.write_reg_gpr(instr.rt() as usize, value);
            }
            Mtc0 =>
            {
                // MTC0
                let data = self.read_reg_gpr(instr.rt() as usize);
                self.cp0.write_reg(instr.rd(), data);
            }
            Beql =>
            {
                // BEQL (and BEQZL)
                if (self.read_reg_gpr(instr.rs() as usize) == self.read_reg_gpr(instr.rt() as usize))
                {
                    let sign_extended_offset =
                        ((instr.offset() as i16) as u64).wrapping_shl(2);
                    self.reg_pc = self.reg_pc.wrapping_add(sign_extended_offset);

                    // TODO split into own fxn
                    // TODO refactor delay slot stuff, this could stack overflow
                    self.run_instruction();
                }
            }
            Lw =>
            {
                // LW
                let base = instr.rs();

                // sign extend upper 32 bits
                let signed_offset = ((instr.offset() as i16) as u64); // TODO refactor
                let virt_addr =
                    signed_offset + self.read_reg_gpr(base as usize);
                let word = self.read_word(virt_addr);
                let mem = (word as i32) as u64;

                self.write_reg_gpr(instr.rt() as usize, mem);
            }
        }

        self.reg_pc += 4;
    }

    // take an address (64b) and return a word (32b)
    fn read_word(&self, virt_addr: u64) -> u32
    {
        let phys_addr = self.virt_addr_to_phys_addr(virt_addr);
        self.interconnect.read_word(phys_addr as u32)
    }

    fn virt_addr_to_phys_addr(&self, virt_addr: u64) -> u64
    {
        // see table 5-3 datasheet p136
        let addr_bit_values = (virt_addr >> 29) & 0b111;

        if (addr_bit_values == 0b101)
        {
            // kseg1
            virt_addr - 0xffff_ffff_a000_0000
        }
        else
        {
            // TODO
            panic!("Unrecognized virtual address: {:#x}", virt_addr);
        }
    }

    fn write_reg_gpr(&mut self, index: usize, value: u64)
    {
        // b/c first register is always zero
        if (index != 0)
        {
            self.reg_gpr[index] = value;
        }
    }

    fn read_reg_gpr(&self, index: usize) -> u64
    {
        match index
        {
            0 => 0,
            _ => self.reg_gpr[index]
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const REGS_PER_LINE: usize = 2;
        const REG_NAMES: [&'static str; NUM_GPR] = [
        "r0", "at", "v0", "v1", "a0", "a1", "a2", "a3",
        "t0", "t1", "t2", "t3", "t4", "t5", "t6", "t7",
        "s0", "s1", "s2", "s3", "s4", "s5", "s6", "s7",
        "t8", "t9", "k0", "k1", "gp", "sp", "s8", "ra",
        ];

        // must put 'r#' in front of all 'try!'s because rust has a bug
        // where 'try' keyword interferes with 'try!' macro...
        r#try!(write!(f,"\nCPU General Purpose Registers:"));
        for reg_num in 0..NUM_GPR {
            if (reg_num % REGS_PER_LINE) == 0 {
                r#try!(writeln!(f,""));
            }
            r#try!(write!(f,
                "{reg_name}/gpr{num:02}: {value:#018X} ",
                num = reg_num,
                reg_name = REG_NAMES[reg_num],
                value = self.reg_gpr[reg_num],
            ));
        }

        r#try!(write!(f,"\n\nCPU Floating Point Registers:"));
        for reg_num in 0..NUM_GPR {
            if (reg_num % REGS_PER_LINE) == 0 {
                r#try!(writeln!(f,""));
            }
            r#try!(write!(f,
                "fpr{num:02}: {value:21} ",
                num = reg_num,
                value = self.reg_fpr[reg_num],)
            );
        }

        r#try!(writeln!(f,"\n\nCPU Special Registers:"));
        r#try!(writeln!(f,
            "\
            reg_pc: {:#018X}\n\
            reg_hi: {:#018X}\n\
            reg_lo: {:#018X}\n\
            reg_llbit: {}\n\
            reg_fcr0:  {:#010X}\n\
            reg_fcr31: {:#010X}\n\
            ",
            self.reg_pc,
            self.reg_hi,
            self.reg_lo,
            self.reg_llbit,
            self.reg_fcr0,
            self.reg_fcr31
        ));

        r#try!(writeln!(f, "{:#?}", self.cp0));
        writeln!(f, "{:#?}", self.interconnect)
    }
}
