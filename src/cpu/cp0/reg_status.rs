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

#[derive(Debug, Default)]
struct InterruptMask
{
    timer_interrupt: bool, // IM(7)
    external_interrupt_write_req: [bool; 5], // IM(6:2)
    software_interrupt_cause_reg: [bool; 2], // IM(1:0)
}

#[derive(Debug)]
enum Mode
{
    Kernel,     // bits 00
    Supervisor, // bits 01
    User,       // bits 10
    Invalid,    // bits 11; TODO panic on decode
}

impl Default for Mode
{
    fn default() -> Mode
    {
        Mode::Kernel
    }
}
