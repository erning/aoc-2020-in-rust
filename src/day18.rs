#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    N(u64),
    O(char),
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().map(|s| s.trim()).collect()
}

fn tokenize(s: &str) -> Vec<Token> {
    s.chars()
        .filter(|c| c.is_numeric() || ['(', ')', '+', '*'].contains(c))
        .map(|c| {
            if c.is_numeric() {
                Token::N(c.to_digit(10).unwrap() as u64)
            } else {
                Token::O(c)
            }
        })
        .collect()
}

fn evaluate(expr: &str, rpn: &dyn Fn(Vec<Token>) -> Vec<Token>) -> u64 {
    let tokens = tokenize(expr);
    let tokens = rpn(tokens);
    let mut stack = Vec::new();
    for token in tokens.into_iter() {
        match token {
            Token::N(n) => stack.push(n),
            Token::O(op) => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(match op {
                    '+' => lhs + rhs,
                    '*' => lhs * rhs,
                    _ => unreachable!(),
                });
            }
        }
    }
    stack.pop().unwrap()
}

pub fn part_one(input: &str) -> u64 {
    fn rpn(tokens: Vec<Token>) -> Vec<Token> {
        let mut s1 = Vec::new();
        let mut s2 = Vec::new();
        for token in tokens {
            match token {
                Token::N(_) => s2.push(token),
                Token::O(_) => {
                    if s1.is_empty() || token == Token::O('(') {
                        s1.push(token);
                        continue;
                    }
                    if token == Token::O(')') {
                        while let Some(last) = s1.pop() {
                            if last == Token::O('(') {
                                break;
                            }
                            s2.push(last);
                        }
                        continue;
                    }
                    let last = s1.pop().unwrap();
                    if last == Token::O('(') {
                        s1.push(last);
                        s1.push(token);
                        continue;
                    }
                    s2.push(last);
                    s1.push(token);
                }
            }
        }
        while let Some(last) = s1.pop() {
            s2.push(last);
        }
        s2
    }

    let expressions = parse_input(input);
    expressions.iter().map(|expr| evaluate(expr, &rpn)).sum()
}

pub fn part_two(input: &str) -> u64 {
    fn rpn(tokens: Vec<Token>) -> Vec<Token> {
        let mut s1 = Vec::new();
        let mut s2 = Vec::new();
        for token in tokens {
            match token {
                Token::N(_) => s2.push(token),
                Token::O(_) => {
                    if s1.is_empty() || token == Token::O('(') {
                        s1.push(token);
                        continue;
                    }
                    if token == Token::O(')') {
                        while let Some(last) = s1.pop() {
                            if last == Token::O('(') {
                                break;
                            }
                            s2.push(last);
                        }
                        continue;
                    }
                    let last = s1.pop().unwrap();
                    if last == Token::O('(') {
                        s1.push(last);
                        s1.push(token);
                        continue;
                    }
                    if last == Token::O('*') && token == Token::O('+') {
                        s1.push(last);
                        s1.push(token);
                        continue;
                    }
                    s2.push(last);
                    s1.push(token);
                }
            }
        }
        while let Some(last) = s1.pop() {
            s2.push(last);
        }
        s2
    }
    let expressions = parse_input(input);
    expressions.iter().map(|expr| evaluate(expr, &rpn)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(18);
        assert_eq!(part_one(&input), 71 + 51 + 26 + 437 + 12240 + 13632);
        assert_eq!(part_two(&input), 231 + 51 + 46 + 1445 + 669060 + 23340);
    }
}
