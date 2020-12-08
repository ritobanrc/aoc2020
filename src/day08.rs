use itertools::Itertools;

enum Instruction {
    Nop,
    Acc,
    Jmp,
}

type Program = Vec<(Instruction, isize)>;

fn parse(input: &str) -> Program {
    input
        .lines()
        .map(|ins| {
            let (ins, arg) = ins.split(' ').collect_tuple().unwrap();
            let ins = match ins {
                "nop" => Instruction::Nop,
                "acc" => Instruction::Acc,
                "jmp" => Instruction::Jmp,
                _ => panic!("Unrecognized instruction: {:?}", ins),
            };
            let arg = arg.parse::<isize>().unwrap();
            (ins, arg)
        })
        .collect()
}

// If Ok, the program terminates. If Err, the program enterred an infinite loop, and the value right before that is returned.
fn run_program(instructions: &Program) -> Result<isize, isize> {
    let mut acc = 0;
    let mut ins = 0;
    let mut visited: Vec<_> = instructions.iter().map(|_| false).collect();

    while visited[ins] == false {
        visited[ins] = true;
        match instructions[ins] {
            (Instruction::Nop, _) => ins += 1,
            (Instruction::Acc, n) => {
                ins += 1;
                acc += n;
            }
            (Instruction::Jmp, n) => {
                let next_ins = ins as isize + n;
                ins = next_ins as usize;
            }
        }

        if ins >= instructions.len() {
            return Ok(acc);
        }
    }

    Err(acc)
}

pub fn part1(input: String) -> isize {
    let instructions = parse(&input);
    if let Err(acc) = run_program(&instructions) {
        return acc;
    } else {
        panic!("Day 08 -- Input terminates successfully, when it shouldn't.")
    }
}

pub fn part2(input: String) -> isize {
    let mut instructions = parse(&input);
    for idx in 0..instructions.len() {
        let ins = &mut instructions[idx];
        match ins {
            (Instruction::Nop, n) => *ins = (Instruction::Jmp, *n),
            (Instruction::Jmp, n) => *ins = (Instruction::Nop, *n),
            _ => {}
        }

        if let Ok(acc) = run_program(&instructions) {
            return acc;
        } else {
            // Undo the swap, if it didn't work
            let ins = &mut instructions[idx];
            match ins {
                (Instruction::Nop, n) => *ins = (Instruction::Jmp, *n),
                (Instruction::Jmp, n) => *ins = (Instruction::Nop, *n),
                _ => {}
            }
        }
    }

    0
}
