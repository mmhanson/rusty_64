use super::reg_config;
use super::reg_status;

#[derive(Debug, Default)]
pub struct Cp0
{
    // cp0 registers (see datasheet: p46)
    reg_config: reg_config::RegConfig,
    reg_status: reg_status::RegStatus,
}

impl Cp0
{
    // also 'hard_reset'
    pub fn power_on_reset(&mut self)
    {
        self.reg_config.power_on_reset();
    }

    pub fn write_reg(&mut self, index: u32, data: u64)
    {
        match index
        {
            12 =>
            {
                // status register (see datasheet p166)
                self.reg_status = (data as u32).into();
            },
            16 =>
            {
                self.reg_config = (data as u32).into();
            },
            _ => panic!("Unrecognized cp0 reg write: {}, {:#x}", index, data)
        }
    }
}
