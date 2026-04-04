#[derive(Debug, Default, PartialEq, Eq)]
pub struct RegVM {
    pc: usize,
    r0: usize,
    r1: usize,
    r2: usize,
    r3: usize,
}

pub trait CounterMachine {
    fn run(&mut self, instructions: Vec<Instruction>);
}

impl CounterMachine for RegVM {
    fn run(&mut self, instructions: Vec<Instruction>) {
        loop {
            let mut instr = &instructions[self.pc];
            let mut op = instr.r;
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /// # Decrement instruction
    ///
    /// もしも1つ目のレジスタが0よりも大きかったら2番目に指定した行へ行く。0以下だったら三番目に指定した行へ行く。
    Dec {
        /// Register number
        r: usize,

        /// Line number
        if_big_line: usize,

        /// Line number
        if_small_line: usize,
    },

    /// # Increment instruction
    ///
    /// 指定したレジスタに1を足し、指定した行へと移動。
    ///
    /// 1. レジスタ番号
    /// 1. 行
    Inc(usize, usize),

    /// # Halt instruction
    ///
    /// マシンを停止する。
    Halt,
}
