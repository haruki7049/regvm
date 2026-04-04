use crate::machine::{CounterMachine, Instruction, RegVM};

pub mod machine;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut machine = RegVM::new(
        vec![22, 11],
        vec![
            Instruction::Dec {
                r: 1,
                next: 1,
                branch: 2,
            },
            Instruction::Inc { r: 0, next: 0 },
            Instruction::Halt,
        ],
    );

    machine.run()?;
    dbg!(machine);

    Ok(())
}
