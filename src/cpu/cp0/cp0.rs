use super::reg_config;
use super::reg_status;

#[derive(Debug, Default)]
pub struct Cp0
{
    // cp0 registers (see datasheet: p46)
    reg_config: reg_config::RegConfig,
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
                self.write_status_reg(data);
            }
            _ => panic!("Unrecognized cp0 reg write: {:#?}, {:#?}", index, data)
        }
    }

    fn write_status_reg(&mut self, data: u64)
    {
        // see datasheet p166 for details
    }
}
