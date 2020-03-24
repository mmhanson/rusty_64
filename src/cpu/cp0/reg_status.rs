#[derive(Debug, Default)]
pub struct RegStatus
{
    // see datasheet p166 for field description
    coprocessor_usability: [bool; 4], // CU
    low_power: bool, // RP
    additional_fp_regs: bool, // FR
    reverse_endian: bool, // RE
    diagnostic_status: DiagnosticStatus, // DS (TODO)
    interrupt_mask: InterruptMask, // IM(7:0)
    kernel_mode_64bit_addressing: bool, // KX
    supervisor_mode_64bit_addressing: bool, // SX
    user_mode_64bit_addressing: bool, // UX
    mode: Mode, // KSU
    error_level: bool, // ERL
    exception_level: bool, // EXL
    interrupts_enabled: bool, // IE
}

impl RegStatus
{
    pub fn write(&mut self, data: u32)
    {
        // see datasheet p166
        self.coprocessor_usability[0] = (data & (1 << 31)) != 0;
        self.coprocessor_usability[1] = (data & (1 << 30)) != 0;
        self.coprocessor_usability[2] = (data & (1 << 29)) != 0;
        self.coprocessor_usability[3] = (data & (1 << 28)) != 0;
        self.low_power                = (data & (1 << 27)) != 0;
        self.additional_fp_regs       = (data & (1 << 26)) != 0;
        self.reverse_endian           = (data & (1 << 25)) != 0;

        self.diagnostic_status = data.into();
        self.interrupt_mask = data.into();

        self.kernel_mode_64bit_addressing     = (data & (1 << 7)) != 0;
        self.supervisor_mode_64bit_addressing = (data & (1 << 6)) != 0;
        self.user_mode_64bit_addressing       = (data & (1 << 5)) != 0;

        self.mode = data.into();

        self.error_level        = (data & (1 << 2)) != 0;
        self.exception_level    = (data & (1 << 1)) != 0;
        self.interrupts_enabled = (data & (1 << 0)) != 0;
    }
}

#[derive(Debug, Default)]
struct DiagnosticStatus
{
    instruction_trace_support: bool, // ITS
    // TODO names
    tlb_general_exception_vector_location: TLBGeneralExceptionVectorLocation, // BEV
    tlb_shutdown: bool, // TS
    soft_reset_or_nmi_occurrec: bool, // SR
    condition_bit: bool, // CH

}

impl From<u32> for DiagnosticStatus
{
    fn from(value: u32) -> Self
    {
        DiagnosticStatus
        {
            instruction_trace_support:  (value & (1 << 24)) != 0,
            tlb_general_exception_vector_location: value.into(),
            tlb_shutdown:               (value & (1 << 21)) != 0,
            soft_reset_or_nmi_occurrec: (value & (1 << 20)) != 0,
            condition_bit:              (value & (1 << 18)) != 0,
        }
    }
}

// TODO better name
#[derive(Debug)]
enum TLBGeneralExceptionVectorLocation
{
    Normal,
    Bootstrap,
}

impl Default for TLBGeneralExceptionVectorLocation
{
    fn default() -> TLBGeneralExceptionVectorLocation
    {
        TLBGeneralExceptionVectorLocation::Normal
    }
}

impl From<u32> for TLBGeneralExceptionVectorLocation
{
    fn from(value: u32) -> Self
    {
        match (value >> 22) & 0b1
        {
            0 => TLBGeneralExceptionVectorLocation::Normal,
            1 => TLBGeneralExceptionVectorLocation::Bootstrap,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
struct InterruptMask
{
    timer_interrupt: bool, // IM(7)
    external_interrupt_write_req: [bool; 5], // IM(6:2)
    software_interrupt_cause_reg: [bool; 2], // IM(1:0)
}

impl From<u32> for InterruptMask
{
    fn from(value: u32) -> Self
    {
        InterruptMask
        {
            timer_interrupt: (value & (1 << 15)) != 0,
            external_interrupt_write_req: [
                (value & (1 << 10)) != 0,
                (value & (1 << 11)) != 0,
                (value & (1 << 12)) != 0,
                (value & (1 << 13)) != 0,
                (value & (1 << 14)) != 0],
            software_interrupt_cause_reg: [
                (value & (1 << 8)) != 0,
                (value & (1 << 9)) != 0],
        }
    }
}

#[derive(Debug)]
enum Mode
{
    Kernel,     // bits 00
    Supervisor, // bits 01
    User,       // bits 10
}

impl Default for Mode
{
    fn default() -> Self
    {
        Mode::Kernel
    }
}

impl From<u32> for Mode
{
    fn from(value: u32) -> Self
    {
        match (value >> 3) & 0b11
        {
            0b00 => Mode::Kernel,
            0b01 => Mode::Supervisor,
            0b10 => Mode::User,
            _ => panic!("Invalid cp0 KSU bits: {:#b}", value)
        }
    }
}
