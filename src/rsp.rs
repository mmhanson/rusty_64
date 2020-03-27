#[derive(Default)]
pub struct Rsp; // unit struct, no fields

impl Rsp
{
    // TODO read general regs
    pub fn read_status_reg(&self) -> u32
    {
        // TODO actually impl
        0
    }
}
