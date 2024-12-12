use error_chain::example_generated::{Result, ResultExt};

pub const MEM_SIZE: usize = 65536;
pub const NUM_REG: usize = 8;

#[derive(Debug)]
pub struct Register(usize);

impl Register {
    pub fn from_str(src: &str) -> Result<Self> {
        let r = src.parse::<usize>().chain_err(|| "Failed to parse register")?;

        if (0..NUM_REG).contains(&r) {
            Ok(Register(r))
        }
        else {
            Err("Register number out of range".into())
        }
    }
}

#[derive(Debug)]
pub struct Label(String);

#[derive(Debug)]
pub enum Address {
    Literal(i32),
    Label(Label),
}

#[derive(Debug)]
pub enum Instruction {
    Add(Register, Register, Register),
    Nor(Register, Register, Register),

    // Relocation
    Lw(Register, Register, Address),
    Sw(Register, Register, Address),
    Beq(Register, Register, Address),

    Jalr(Register, Register),
    Halt,
    Noop,

    Fill(i32),
}
