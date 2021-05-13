// /// Flags mask
// pub const FI: u16 = 0b0000_0000_0000_0001;
// /// Jump mask
// pub const JP: u16 = 0b0000_0000_0000_0010;
// /// Program Counter Out mask
// pub const CO: u16 = 0b0000_0000_0000_0100;
// /// Program Counter Enable mask
// pub const CE: u16 = 0b0000_0000_0000_1000;
// /// Output Register In mask
// pub const OI: u16 = 0b0000_0000_0001_0000;
// /// Register B In mask
// pub const BI: u16 = 0b0000_0000_0010_0000;
// /// ALU Subtract mask
// pub const SU: u16 = 0b0000_0000_0100_0000;
// /// ALU Sum Out mask
// pub const EO: u16 = 0b0000_0000_1000_0000;
// /// Register A Out mask
// pub const AO: u16 = 0b0000_0001_0000_0000;
// /// Register A In mask
// pub const AI: u16 = 0b0000_0010_0000_0000;
// /// Instruction Register In mask
// pub const II: u16 = 0b0000_0100_0000_0000;
// /// Instruction Register Out mask
// pub const IO: u16 = 0b0000_1000_0000_0000;
// /// RAM Out mask
// pub const RO: u16 = 0b0001_0000_0000_0000;
// /// RAM In mask
// pub const RI: u16 = 0b0010_0000_0000_0000;
// /// Memory Access Register In mask
// pub const MI: u16 = 0b0100_0000_0000_0000;
// /// Halt mask
// pub const HLT: u16 = 0b1000_0000_0000_0000;

#[derive(Debug, Clone)]
pub struct EaterCpu {
    pub clk: bool,
    pub pc: u8,
    pub t: u8,
    pub a: u8,
    pub b: u8,
    pub ir: u8,
    pub mar: u8,
    pub out: u8,
    pub mc: [u16; 0x400],
    pub ram: [u8; 0x10],
}

impl Default for EaterCpu {
    fn default() -> Self {
        EaterCpu {
            clk: false,
            pc: 0,
            t: 0,
            a: 0,
            b: 0,
            ir: 0,
            mar: 0,
            out: 0,
            mc: [0; 0x400],
            ram: [0; 0x10],
        }
    }
}

// impl EaterCpu {
//     pub fn clock_down(&mut self, control: u16) {}
// }
