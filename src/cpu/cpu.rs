use super::super::interconnect;
use super::cp0::cp0;

const NUM_GPR: usize = 32; // number of general purpose registers

// a NEC VR-4300 MIPS CPU
#[derive(Debug)]
pub struct Cpu
{
    // cpu registers (see datasheet: 1.4.2)
    reg_gpr: [u64; NUM_GPR],
    reg_fpr: [f64; NUM_GPR], // floating point
    reg_pc: u64,
    reg_hi: u64,
    reg_low: u64,
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
            reg_low: 0,
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
        let instruction = self.read_word(self.reg_pc);

        let opcode = (instruction >> 26) & 0b111111;
        let rt = (instruction >> 16) & 0b11111; // target register index
        match opcode
        {
            0b001111 =>
            {
                // LUI
                let imm = instruction & 0xffff;
                // TODO sign extend upper 32 bits
                self.write_reg_gpr(rt as usize, (imm << 16) as u64);
            }
            0b010000 =>
            {
                // MTC0
                let rd = (instruction >> 11) & 0b11111;
                let data = self.read_reg_gpr(rt as usize);
                self.cp0.write_reg(rd, data);
            }
            _ =>
            {
                panic!("Unrecognized opword: {:#x}", instruction)
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
