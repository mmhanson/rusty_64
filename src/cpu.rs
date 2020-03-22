use super::interconnect;

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

    cp0: Cp0, // cpu contains cp0
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
            cp0: Cp0::default(),
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
        match opcode
        {
            0b001111 =>
            {
                // LUI
                println!("We got LUI!");
                let imm = instruction & 0xffff;
                let rt = (instruction >> 16) & 0x11111; // index
                // TODO check if in 32 or 64b mode for sign extension
                // NOTE 32b mode is assumed below
                self.write_reg_gpr(rt as usize, (imm << 16) as u64);
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
}

// The 'EP' area of the config register in the cp0
// See datasheet p152
// TODO better name?
#[derive(Debug)]
enum RegConfigEp
{
    D,
    DxxDxx,
    RFU,
}

impl Default for RegConfigEp
{
    fn default() -> RegConfigEp
    {
        RegConfigEp::D
    }
}

// TODO better name?
#[derive(Debug)]
enum RegConfigBe
{
    LittleEndian,
    BigEndian,
}

impl Default for RegConfigBe
{
    fn default() -> RegConfigBe
    {
        RegConfigBe::BigEndian
    }
}

#[derive(Debug, Default)]
struct RegConfig
{
    // 'areas' of cp0 registers are split out to separate structures for ergonomics
    reg_config_ep: RegConfigEp,
    reg_config_be: RegConfigBe,
}

impl RegConfig
{
    fn power_on_reset(&mut self)
    {
        // see datasheet: 9.2.1 p249
        self.reg_config_ep = RegConfigEp::D;
        self.reg_config_be = RegConfigBe::BigEndian;
    }
}

#[derive(Debug, Default)]
struct Cp0
{
    // cp0 registers (see datasheet: p46)
    reg_config: RegConfig,
}

impl Cp0
{
    // also 'hard_reset'
    fn power_on_reset(&mut self)
    {
        self.reg_config.power_on_reset();
    }
}
