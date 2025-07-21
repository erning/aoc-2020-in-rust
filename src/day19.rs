//! Day 19: Monster Messages
//!
//! ## Problem Description
//!
//! Part 1: Count how many messages completely match rule 0 using the given grammar rules.
//! Part 2: Modify rules 8 and 11 to handle loops, then count valid messages.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Splits input into:
//! - Rules: Grammar rules in format "id: rule" where rules can be literals or sequences
//! - Messages: Lines of text to validate against the grammar
//!
//! **Part 1 Strategy**: Recursive pattern matching
//! - Builds rules as a grammar tree with literals and sequences
//! - Uses recursive descent parsing to match messages against rule 0
//! - Returns all possible suffixes after matching a rule prefix
//! - Message is valid if any suffix is empty (complete match)
//!
//! **Part 2 Strategy**: Grammar modification with loops
//! - Rule 8: Replaced with "42 | 42 8" (one or more 42s)
//! - Rule 11: Replaced with "42 31 | 42 11 31" (n 42s followed by n 31s)
//! - Same recursive matching algorithm handles the modified grammar
//!
//! **Algorithm**: Recursive backtracking parser with memoization via function calls.
//!
//! ## Rule Types
//! - **L(char)**: Literal character match
//! - **S(sequences)**: Sequence of rule references with alternation (|)
//!
//! ## Grammar Format
//! - Literals: "a" or "b"
//! - Sequences: "1 2 3" or "1 2 | 3 4"

use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
enum Rule {
    L(char),            // Literal
    S(Vec<Vec<usize>>), // Sequence [Sequence, Sequence, ...]
}

type Rules = HashMap<usize, Rule>;

fn parse_input(input: &str) -> (Rules, Vec<&str>) {
    let (p1, p2) = input.trim().split_once("\n\n").unwrap();

    let rules: Vec<(usize, Rule)> = p1
        .lines()
        .map(|s| {
            let (s1, s2) = s.split_once(": ").unwrap();
            let idx = s1.parse().unwrap();
            let rule = if s2.starts_with('"') {
                Rule::L(s2.chars().nth(1).unwrap())
            } else {
                Rule::S(
                    s2.split('|')
                        .map(|s| {
                            s.split_whitespace()
                                .map(|v| v.parse().unwrap())
                                .collect()
                        })
                        .collect(),
                )
            };
            (idx, rule)
        })
        .collect();
    // rules.sort_unstable_by_key(|&(id, _)| id);
    let rules = rules.into_iter().collect();
    let messages = p2.lines().collect();
    (rules, messages)
}

// Returns a Vec of possible suffixes after matching rule idx at the start of message
fn match_rule<'a>(
    rules: &Rules,
    idx: usize,
    message: &'a [char],
) -> Vec<&'a [char]> {
    match &rules[&idx] {
        Rule::L(ch) => {
            if !message.is_empty() && &message[0] == ch {
                vec![&message[1..]]
            } else {
                vec![]
            }
        }
        Rule::S(seqs) => {
            let mut results = Vec::new();
            for seq in seqs {
                let mut suffixes = vec![message];
                for &i in seq {
                    let mut new_suffixes = Vec::new();
                    for suffix in &suffixes {
                        let matches = match_rule(rules, i, suffix);
                        new_suffixes.extend(matches);
                    }
                    suffixes = new_suffixes;
                    if suffixes.is_empty() {
                        break;
                    }
                }
                results.extend(suffixes);
            }
            results
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let (rules, messages) = parse_input(input);

    messages
        .iter()
        .filter(|msg| {
            let chars: Vec<char> = msg.chars().collect();
            match_rule(&rules, 0, &chars)
                .iter()
                .any(|rest| rest.is_empty())
        })
        .count()
}

pub fn part_two(input: &str) -> usize {
    let (mut rules, messages) = parse_input(input);
    rules.insert(8, Rule::S(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::S(vec![vec![42, 31], vec![42, 11, 31]]));

    messages
        .iter()
        .filter(|msg| {
            let chars: Vec<char> = msg.chars().collect();
            match_rule(&rules, 0, &chars)
                .iter()
                .any(|rest| rest.is_empty())
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(19);
        assert_eq!(part_one(&input), 2);
    }

    #[test]
    fn example_part_two() {
        let input = concat!(
            "42: 9 14 | 10 1\n",
            "9: 14 27 | 1 26\n",
            "10: 23 14 | 28 1\n",
            "1: \"a\"\n",
            "11: 42 31\n",
            "5: 1 14 | 15 1\n",
            "19: 14 1 | 14 14\n",
            "12: 24 14 | 19 1\n",
            "16: 15 1 | 14 14\n",
            "31: 14 17 | 1 13\n",
            "6: 14 14 | 1 14\n",
            "2: 1 24 | 14 4\n",
            "0: 8 11\n",
            "13: 14 3 | 1 12\n",
            "15: 1 | 14\n",
            "17: 14 2 | 1 7\n",
            "23: 25 1 | 22 14\n",
            "28: 16 1\n",
            "4: 1 1\n",
            "20: 14 14 | 1 15\n",
            "3: 5 14 | 16 1\n",
            "27: 1 6 | 14 18\n",
            "14: \"b\"\n",
            "21: 14 1 | 1 14\n",
            "25: 1 1 | 1 14\n",
            "22: 14 14\n",
            "8: 42\n",
            "26: 14 22 | 1 20\n",
            "18: 15 15\n",
            "7: 14 5 | 1 21\n",
            "24: 14 1\n",
            "\n",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\n",
            "bbabbbbaabaabba\n",
            "babbbbaabbbbbabbbbbbaabaaabaaa\n",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa\n",
            "bbbbbbbaaaabbbbaaabbabaaa\n",
            "bbbababbbbaaaaaaaabbababaaababaabab\n",
            "ababaaaaaabaaab\n",
            "ababaaaaabbbaba\n",
            "baabbaaaabbaaaababbaababb\n",
            "abbbbabbbbaaaababbbbbbaaaababb\n",
            "aaaaabbaabaaaaababaa\n",
            "aaaabbaaaabbaaa\n",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa\n",
            "babaaabbbaaabaababbaabababaaab\n",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba\n",
        );
        assert_eq!(part_two(input), 12);
    }
}
