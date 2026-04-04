#[derive(Debug, PartialEq, Eq)]
pub struct RegVM {
    r0: isize,
    r1: isize,
    r2: isize,
    r3: isize,
}

pub trait CounterMachine {
    fn run(&mut self, instructions: Vec<Instruction>);
}

impl CounterMachine for RegVM {
    fn run(&mut self, instructions: Vec<Instruction>) {}
}

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    /// Decrement instruction
    ///
    /// 1. レジスタ番号
    /// 1. 行
    /// 1. 行
    /// もしも1つ目のレジスタが0よりも大きかったら2番目に指定した行へ行く。0以下だったら三番目に指定した行へ行く。
    Dec(usize, usize, usize),

    /// Increment instruction
    ///
    /// 1. レジスタ番号
    /// 1. 行
    ///
    /// 指定したレジスタに1を足し、指定した行へと移動。
    Inc(usize, usize),

    /// Halt instruction
    ///
    /// マシンを停止する。
    Halt,
}
