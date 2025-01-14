fn parse_input(input: &str) -> Vec<(&str, i32)> {
    input
        .trim()
        .lines()
        .map(|s| {
            let v = s.split_whitespace().collect::<Vec<_>>();
            (v[0], v[1].parse::<i32>().unwrap())
        })
        .collect()
}

fn execute(program: &[(&str, i32)]) -> Result<i32, i32> {
    let n = program.len();
    let mut visited: Vec<bool> = vec![false; n];
    let mut a = 0;
    let mut p = 0;
    loop {
        if p as usize >= n {
            break Ok(a);
        }
        if visited[p as usize] {
            break Err(a);
        }
        visited[p as usize] = true;
        let (operator, operand) = &program[p as usize];
        match *operator {
            "acc" => {
                a += operand;
                p += 1
            }
            "jmp" => p += operand,
            _ => p += 1,
        }
    }
}

pub fn part_one(input: &str) -> i32 {
    let program = parse_input(input);
    execute(&program).err().unwrap()
}

pub fn part_two(input: &str) -> i32 {
    const NOP: &str = "nop";
    const JMP: &str = "jmp";

    let mut program = parse_input(input);
    let candidates = program
        .iter()
        .enumerate()
        .filter(|(_, (operator, _))| [NOP, JMP].contains(operator))
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    for i in candidates {
        let (operator, operand) = program[i];
        let op = match operator {
            NOP => JMP,
            JMP => NOP,
            _ => panic!(),
        };
        program[i] = (op, operand);
        if let Ok(a) = execute(&program) {
            return a;
        }
        program[i] = (operator, operand);
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(8);
        assert_eq!(part_one(&input), 5);
        assert_eq!(part_two(&input), 8);
    }
}
