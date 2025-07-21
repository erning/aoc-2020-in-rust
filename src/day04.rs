//! Day 4: Passport Processing
//!
//! ## Problem Description
//!
//! Part 1: Count valid passports based on required fields presence.
//! Part 2: Count valid passports based on field presence AND field value validation.
//!
//! Required fields: byr, iyr, eyr, hgt, hcl, ecl, pid (cid is optional)
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Splits input by double newlines to separate passports,
//! then parses each passport into a HashMap of field-value pairs.
//!
//! **Part 1 Strategy**: Field presence validation
//! - Checks if all required fields (except cid) are present
//! - Uses a predefined list of required field keys
//!
//! **Part 2 Strategy**: Field value validation
//! - Applies all Part 1 validations first
//! - Validates each field value according to specific rules:
//!   - byr: 1920-2002 (birth year)
//!   - iyr: 2010-2020 (issue year)
//!   - eyr: 2020-2030 (expiration year)
//!   - hgt: 150-193cm or 59-76in (height)
//!   - hcl: # followed by 6 hex digits (hair color)
//!   - ecl: one of [amb, blu, brn, gry, grn, hzl, oth] (eye color)
//!   - pid: 9-digit number (passport ID)
//!
//! **Validation Logic**: Uses pattern matching for clean validation of each field type.

use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .trim()
        .split("\n\n")
        .map(|s| {
            s.split(['\n', ' '])
                .map(|s| s.trim())
                .map(|s| (&s[..3], &s[4..]))
                .collect()
        })
        .collect()
}

const FIELDS: [&str; 8] =
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"];

fn is_valid_fields(pp: &HashMap<&str, &str>) -> bool {
    FIELDS.iter().rev().skip(1).all(|k| pp.contains_key(k))
}

fn is_valid_values(pp: &HashMap<&str, &str>) -> bool {
    pp.iter().all(|(k, v)| match *k {
        "byr" => match v.parse::<usize>() {
            Ok(v) => (1920..=2002).contains(&v),
            _ => false,
        },
        "iyr" => match v.parse::<usize>() {
            Ok(v) => (2010..=2020).contains(&v),
            _ => false,
        },
        "eyr" => match v.parse::<usize>() {
            Ok(v) => (2020..=2030).contains(&v),
            _ => false,
        },
        "hgt" if v.ends_with("cm") => {
            match v[..v.len() - 2].parse::<usize>() {
                Ok(v) => (150..=193).contains(&v),
                _ => false,
            }
        }
        "hgt" if v.ends_with("in") => {
            match v[..v.len() - 2].parse::<usize>() {
                Ok(v) => (59..=76).contains(&v),
                _ => false,
            }
        }
        "hcl" => {
            v.len() == 7
                && v.starts_with("#")
                && v.chars().skip(1).all(|ch| ch.is_ascii_hexdigit())
        }
        "ecl" => {
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v)
        }
        "pid" => v.len() == 9 && v.chars().all(|ch| ch.is_ascii_digit()),
        "cid" => true,
        _ => false,
    })
}

pub fn part_one(input: &str) -> usize {
    let pps = parse_input(input);
    pps.iter().filter(|pp| is_valid_fields(pp)).count()
}

pub fn part_two(input: &str) -> usize {
    let pps = parse_input(input);
    pps.iter()
        .filter(|pp| is_valid_fields(pp))
        .filter(|pp| is_valid_values(pp))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(4);
        assert_eq!(part_one(&input), 2);

        const INPUT: &str = concat!(
            "eyr:1972 cid:100\n",
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926\n",
            "\n",
            "iyr:2019\n",
            "hcl:#602927 eyr:1967 hgt:170cm\n",
            "ecl:grn pid:012533040 byr:1946\n",
            "\n",
            "hcl:dab227 iyr:2012\n",
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277\n",
            "\n",
            "hgt:59cm ecl:zzz\n",
            "eyr:2038 hcl:74454a iyr:2023\n",
            "pid:3556412378 byr:2007\n",
            "\n",
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980\n",
            "hcl:#623a2f\n",
            "\n",
            "eyr:2029 ecl:blu cid:129 byr:1989\n",
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm\n",
            "\n",
            "hcl:#888785\n",
            "hgt:164cm byr:2001 iyr:2015 cid:88\n",
            "pid:545766238 ecl:hzl\n",
            "eyr:2022\n",
            "\n",
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        );
        assert_eq!(part_two(INPUT), 4);
    }
}
