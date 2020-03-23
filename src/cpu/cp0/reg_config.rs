// The 'EP' area of the config register in the cp0
// See datasheet p152
// TODO better name?
#[derive(Debug)]
enum Ep
{
    D,
    DxxDxx,
    RFU,
}

impl Default for Ep
{
    fn default() -> Ep
    {
        Ep::D
    }
}

// TODO better name?
#[derive(Debug)]
enum Be
{
    LittleEndian,
    BigEndian,
}

impl Default for Be
{
    fn default() -> Be
    {
        Be::BigEndian
    }
}

#[derive(Debug, Default)]
pub struct RegConfig
{
    // 'areas' of cp0 registers are split out to separate structures for ergonomics
    ep: Ep,
    be: Be,
}

impl RegConfig
{
    pub fn power_on_reset(&mut self)
    {
        // see datasheet: 9.2.1 p249
        self.ep = Ep::D;
        self.be = Be::BigEndian;
    }
}
