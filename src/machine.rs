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
    fn run(&mut self, instructions: Vec<Instruction>) {}
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /// # Decrement instruction
    ///
    /// もしも1つ目のレジスタが0よりも大きかったら2番目に指定した行へ行く。0以下だったら三番目に指定した行へ行く。
    ///
    /// 1. レジスタ番号
    /// 1. 行
    /// 1. 行
    Dec(usize, usize, usize),

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
