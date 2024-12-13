pub const MEM_SIZE: usize = 65536;
pub const NUM_REG: usize = 8;

#[derive(Debug)]
pub struct Register(usize);

impl Register {
    pub fn from_str(src: &str) -> Self {
        let u = src.parse().unwrap();
        if u >= NUM_REG {
            unreachable!("Register too big");
        }
        Register(u)
    }
}

#[derive(Debug)]
pub enum Address {
    Literal(i32),
    Symbol(String),
}

impl Address {
    pub fn from_str(src: &str) -> Self {
        if src.chars().next().unwrap().is_alphabetic() {
            Self::Symbol(src.to_string())
        }
        else {
            Self::Literal(src.parse().unwrap())
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Add(Register, Register, Register),
    Nor(Register, Register, Register),

    Lw(Register, Register, Address),
    Sw(Register, Register, Address),
    Beq(Register, Register, Address),

    Jalr(Register, Register),

    Halt,
    Noop,

    Fill(i32),
}

pub fn parse(src: &str) -> Vec<(Option<String>, Instruction)> {
    src.lines().map(|line| {
        let mut words = line.split_whitespace();

        let label = if line.chars().next().unwrap().is_whitespace() {
            None
        }
        else {
            Some(words.next().unwrap().to_string())
        };

        let instr = match words.next().unwrap() {
            "add" => Instruction::Add(
                    Register::from_str(words.next().unwrap()),
                    Register::from_str(words.next().unwrap()),
                    Register::from_str(words.next().unwrap())
                ),
            "nor" => Instruction::Nor(
                    Register::from_str(words.next().unwrap()),
                    Register::from_str(words.next().unwrap()),
                    Register::from_str(words.next().unwrap())
                ),
            "lw" => Instruction::Lw(
                    Register::from_str(words.next().unwrap()),
                    Register::from_str(words.next().unwrap()),
                    Address::from_str(words.next().unwrap())
                ),
            "sw" => Instruction::Sw(
                    Register::from_str(words.next().unwrap()),
                    Register::from_str(words.next().unwrap()),
                    Address::from_str(words.next().unwrap())
                ),
            "beq" => Instruction::Beq(
                    Register::from_str(words.next().unwrap()),
                    Register::from_str(words.next().unwrap()),
                    Address::from_str(words.next().unwrap())
                ),
            "jalr" => Instruction::Jalr(
                    Register::from_str(words.next().unwrap()), 
                    Register::from_str(words.next().unwrap())
                ),
            "halt" => Instruction::Halt,
            "fill" => Instruction::Fill(words.next().unwrap().parse().unwrap()),
            "noop" => Instruction::Noop,
            word => unreachable!("{word} is not a valid instruction")
        };
        (label, instr)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn p() {
        let i = parse("label lw 0 1 5\n");
        print!("{:?}", i);
    }
}
