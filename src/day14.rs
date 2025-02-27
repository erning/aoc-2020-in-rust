use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<Vec<(u64, u64)>> {
    input
        .split("mask = ")
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|section| {
            section
                .lines()
                .map(|s| {
                    if let Some(s) = s.strip_prefix("mem[") {
                        let parts: Vec<&str> = s.splitn(2, "] = ").collect();
                        (parts[0].parse().unwrap(), parts[1].parse().unwrap())
                    } else {
                        let (mut bm0, mut bm1) = (0, 0);
                        for c in s.chars() {
                            bm0 <<= 1;
                            bm1 <<= 1;
                            match c {
                                '0' => bm0 |= 1,
                                '1' => bm1 |= 1,
                                'X' => {}
                                _ => unreachable!(),
                            }
                        }
                        (bm0, bm1)
                    }
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> u64 {
    let program = parse_input(input);
    let mut memory = HashMap::<u64, u64>::new();
    for section in program.iter() {
        let (bm0, bm1) = section[0];
        for &(address, value) in section[1..].iter() {
            memory.insert(address, (value | bm1) & !bm0);
        }
    }
    memory.values().sum()
}

pub fn part_two(input: &str) -> u64 {
    let program = parse_input(input);
    let mut memory = HashMap::<u64, u64>::new();
    for section in program.iter() {
        let (bm0, bm1) = section[0];
        let bmx = !bm0 & !bm1 & 0b111111111111111111111111111111111111;
        let bits: Vec<u8> = (0..36).filter(|i| bmx & (1 << i) != 0).collect();
        fn setbmx(
            memory: &mut HashMap<u64, u64>,
            address: u64,
            value: u64,
            bits: &[u8],
        ) {
            if let Some(shift) = bits.first() {
                let mask = 1 << shift;
                for addr in [address & !mask, address | mask] {
                    memory.insert(addr, value);
                    setbmx(memory, addr, value, &bits[1..]);
                }
            }
        }
        for &(address, value) in section[1..].iter() {
            setbmx(&mut memory, address | bm1 & !bmx, value, &bits);
        }
    }
    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(14);
        assert_eq!(part_one(&input), 165);
    }

    #[test]
    fn example_part_two() {
        let input = concat!(
            "mask = 000000000000000000000000000000X1001X\n",
            "mem[42] = 100\n",
            "mask = 00000000000000000000000000000000X0XX\n",
            "mem[26] = 1\n"
        );
        assert_eq!(part_two(input), 208);

        let input = concat!(
            "mask = 000000000000000000000000000000X1001X\n",
            "mem[42] = 100\n",
            "mask = 00000000000000000000000000000000X0XX\n",
            "mem[30] = 1\n"
        );
        assert_eq!(part_two(input), 1 * 8 + 100 * 4);
    }
}
