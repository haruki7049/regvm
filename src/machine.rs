use thiserror::Error;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct RegVM {
    pc: ProgramCounter,
    registers: Vec<Register>,
    instructions: Vec<Instruction>,
}

impl RegVM {
    pub fn new(registers: Vec<Register>, instructions: Vec<Instruction>) -> Self {
        RegVM {
            registers,
            instructions,
            ..Default::default()
        }
    }
}

pub trait CounterMachine {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

impl CounterMachine for RegVM {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let instr = &self.instructions[self.pc];

            match instr {
                Instruction::Dec { r, next, branch } => {
                    if self.registers[*r] > 0 {
                        // Decrements the r register
                        self.registers[*r] -= 1;
                        self.pc = *next; // Go to next address
                    } else {
                        self.pc = *branch; // Go to branch address
                    }
                }
                Instruction::Inc { r, next } => {
                    // Increments the r register
                    self.registers[*r] += 1;
                    self.pc = *next; // Go to next address
                }
                Instruction::Halt => break,
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /// # Decrement instruction
    Dec {
        /// Register number
        r: RegisterNumber,

        /// Line address if the decrement is success
        next: LineNumber,

        /// Line address if it is zero
        branch: LineNumber,
    },

    /// # Increment instruction
    ///
    /// 指定したレジスタに1を足し、指定した行へと移動。
    Inc {
        /// Register number
        r: RegisterNumber,

        /// Next line number
        next: LineNumber,
    },

    /// # Halt instruction
    ///
    /// マシンを停止する。
    Halt,
}

pub trait Runnable {
    fn r(&self) -> Result<RegisterNumber, Box<dyn std::error::Error>>;
    fn next(&self) -> Result<LineNumber, Box<dyn std::error::Error>>;
    fn branch(&self) -> Result<LineNumber, Box<dyn std::error::Error>>;
}

impl Runnable for Instruction {
    fn r(&self) -> Result<RegisterNumber, Box<dyn std::error::Error>> {
        match self {
            Self::Dec { r, .. } => Ok(*r),
            Self::Inc { r, .. } => Ok(*r),
            _ => Err(Box::new(RunnableError::RegisterNumberNotFound)),
        }
    }

    fn next(&self) -> Result<LineNumber, Box<dyn std::error::Error>> {
        match self {
            Self::Dec { next, .. } => Ok(*next),
            Self::Inc { next, .. } => Ok(*next),
            _ => Err(Box::new(RunnableError::NextNotFound)),
        }
    }

    fn branch(&self) -> Result<LineNumber, Box<dyn std::error::Error>> {
        match self {
            Self::Dec { branch, .. } => Ok(*branch),
            _ => Err(Box::new(RunnableError::BranchNotFound)),
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
enum RunnableError {
    #[error("the runnable value does not have a register number")]
    RegisterNumberNotFound,

    #[error("the runnable value does not have a next line address")]
    NextNotFound,

    #[error("the runnable value does not have a branch line address")]
    BranchNotFound,
}

type Register = usize;
type ProgramCounter = usize;
type RegisterNumber = usize;
type LineNumber = usize;
