use std::env;
use std::fs;
use std::io::Read;
use std::path::Path;

const NUM_GPR: usize = 32; // number of general purpose registers

// a NEC VR-4300 MIPS CPU
struct Cpu
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

    cp0: Cp0; // cpu contains cp0
}

impl Cpu
{
    fn new() -> Cpu
    {
        Cpu
        {
            // TODO
        }
    }

    // also 'hard_reset'
    fn power_on_reset (&mut self)
    {
        // TODO
        self.cp0.power_on_reset();
    }
}

struct Cp0
{
    // cp0 registers (see datasheet: p46)
    reg_index: u64,
    reg_random: u64,
    reg_entry_lo0: u64,
    reg_entry_lo1: u64,
    reg_context: u64,
    reg_page_mask: u64,
    reg_wired: u64,
    reg_bad_v_addr: u64,
    reg_count: u64,
    reg_entry_hi: u64,
    reg_compare: u64,
    reg_status: u64,
    reg_cause: u64,
    reg_epc: u64,
    reg_pr_id: u64,
    reg_config: u64,
    reg_ll_addr: u64,
    reg_watch_lo: u64,
    reg_watch_hi: u64,
    reg_x_context: u64,
    reg_parity_error: u64,
    reg_tag_lo: u64,
    reg_tag_hi: u64,
    reg_error_epc: u64,
}

impl Cp0
{
    // construct a zero'd cp0, must call power_on_reset to init
    fn new() -> Cp0
    {
        Cp0
        {
            reg_index: 0,
            reg_random: 0,
            reg_entry_lo0: 0,
            reg_entry_lo1: 0,
            reg_context: 0,
            reg_page_mask: 0,
            reg_wired: 0,
            reg_bad_v_addr: 0,
            reg_count: 0,
            reg_entry_hi: 0,
            reg_compare: 0,
            reg_status: 0,
            reg_cause: 0,
            reg_epc: 0,
            reg_pr_id: 0,
            reg_config: 0,
            reg_ll_addr: 0,
            reg_watch_lo: 0,
            reg_watch_hi: 0,
            reg_x_context: 0,
            reg_parity_error: 0,
            reg_tag_lo: 0,
            reg_tag_hi: 0,
            reg_error_epc: 0,
        }
        // note: could do a power-on reset here instead of making client do it
    }

    // also 'hard_reset'
    fn power_on_reset(&mut self)
    {
        // see datasheet: 9.2.1 p249

    }
}

fn main()
{
    let pif_file_name = env::args().nth(1).unwrap();
    let rom_file_name = env::args().nth(2).unwrap(); // TODO custom error handling not unwrap()

    let pif_rom = read_bin(pif_file_name);
    let rom = read_bin(rom_file_name);

    let mut cpu = Cpu::new();

}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8>
{
    let mut file = fs::File::open(path).unwrap();
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();
    file_buf
}
