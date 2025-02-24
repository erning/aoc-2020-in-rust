fn parse_input(input: &str) -> Vec<(u8, i32)> {
    input
        .trim()
        .lines()
        .map(|s| (s.as_bytes()[0], s[1..].parse().unwrap()))
        .collect()
}

pub fn part_one(input: &str) -> usize {
    const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)]; // ESWN
    let instructions = parse_input(input);
    let mut x = 0;
    let mut y = 0;
    let mut d = 0;
    for inst in instructions {
        match inst {
            (b'R', v) => d = (d + (v / 90) as usize) % 4,
            (b'L', v) => d = (d + 4 - (v / 90) as usize) % 4,
            (b'F', v) => {
                x += DIRS[d].0 * v;
                y += DIRS[d].1 * v;
            }
            (b'E', v) => x += v,
            (b'S', v) => y += v,
            (b'W', v) => x -= v,
            (b'N', v) => y -= v,
            _ => panic!("unknown"),
        }
    }
    (x.abs() + y.abs()) as usize
}

pub fn part_two(input: &str) -> usize {
    let instructions = parse_input(input);
    let mut x = 0;
    let mut y = 0;
    let mut wpx = 10;
    let mut wpy = -1;
    for inst in instructions {
        match inst {
            (b'R', v) => {
                for _ in 0..(v / 90 % 4) {
                    let (dx, dy) = (x - wpx, y - wpy);
                    wpx = x + dy;
                    wpy = y - dx;
                }
            }
            (b'L', v) => {
                for _ in 0..(v / 90 % 4) {
                    let (dx, dy) = (x - wpx, y - wpy);
                    wpx = x - dy;
                    wpy = y + dx;
                }
            }
            (b'F', v) => {
                let (dx, dy) = (wpx - x, wpy - y);
                x += dx * v;
                y += dy * v;
                wpx = x + dx;
                wpy = y + dy;
            }
            (b'E', v) => wpx += v,
            (b'S', v) => wpy += v,
            (b'W', v) => wpx -= v,
            (b'N', v) => wpy -= v,
            _ => panic!("unknown"),
        }
    }
    (x.abs() + y.abs()) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(12);
        assert_eq!(part_one(&input), 25);
        assert_eq!(part_two(&input), 286);
    }
}
