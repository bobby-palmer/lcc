use std::str::SplitWhitespace;

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

impl Label {
    pub fn from_str(src: &str) -> Option<Self> {
        match src.chars().next() {
            Some(ch) if ch.is_alphabetic() => Some(Label(src.into())),
            _ => None
        }
    }
}

#[derive(Debug)]
pub enum Address {
    Literal(i32),
    Label(Label),
}

impl Address {
    pub fn from_str(src: &str) -> Result<Self> {
        let label = Label::from_str(src);

        match label {
            Some(l) => Ok(Address::Label(l)),
            _ => Ok(Self::Literal(src.parse().chain_err(|| "Cannot parse address")?))
        }
    }
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

impl Instruction {
    pub fn from_str(mut src: SplitWhitespace) -> Result<Instruction> {
        match src.next() {
            Some("add") => Ok(
                Self::Add(Register::from_str(src.next().chain_err(|| "Expected register 1")?)?,
                          Register::from_str(src.next().chain_err(|| "Expected register 2")?)?, 
                          Register::from_str(src.next().chain_err(|| "Expected register 3")?)?
                )),
            Some("nor") => Ok(
                Self::Nor(Register::from_str(src.next().chain_err(|| "Expected register 1")?)?,
                          Register::from_str(src.next().chain_err(|| "Expected register 2")?)?, 
                          Register::from_str(src.next().chain_err(|| "Expected register 3")?)?
                )),
            Some("lw") => Ok(
                Self::Lw(Register::from_str(src.next().chain_err(|| "Expected register 1")?)?,
                          Register::from_str(src.next().chain_err(|| "Expected register 2")?)?, 
                          Address::from_str(src.next().chain_err(|| "Expected address")?)?
                )),
            Some("sw") => Ok(
                Self::Lw(Register::from_str(src.next().chain_err(|| "Expected register 1")?)?,
                          Register::from_str(src.next().chain_err(|| "Expected register 2")?)?, 
                          Address::from_str(src.next().chain_err(|| "Expected address")?)?
                )),
            Some("beq") => Ok(
                Self::Lw(Register::from_str(src.next().chain_err(|| "Expected register 1")?)?,
                          Register::from_str(src.next().chain_err(|| "Expected register 2")?)?, 
                          Address::from_str(src.next().chain_err(|| "Expected address")?)?
                )),
            Some("jalr") => Ok(
                Self::Jalr(Register::from_str(src.next().chain_err(|| "Expected register 1")?)?,
                          Register::from_str(src.next().chain_err(|| "Expected register 2")?)?, 
                )),
            Some("halt") => Ok(Self::Halt),
            Some("noop") => Ok(Self::Noop),
            Some(".fill") => Ok(Self::Fill(
                    src.next().chain_err(|| "Expected value")?.parse()
                    .chain_err(|| "Fill value must be an integer")?
                    )),
            None => Err("Empty instruction".into()),
            _ => Err("Invlid instruction".into())
        }
    }
}

pub fn parse_line(src: &str) -> Result<(Option<Label>, Instruction)> {
    let mut src = src.split_whitespace(); 
        Ok((
        Label::from_str(src.next().chain_err(|| "Unexpected end of line")?),
        Instruction::from_str(src)?
        ))
}

#[cfg(test)]
mod tests {
    use super::{parse_line, Instruction};

    #[test]
    fn parse() {
        let i = parse_line("main lw 1 1 label");
        print!("{:?}", i)
    }
}
