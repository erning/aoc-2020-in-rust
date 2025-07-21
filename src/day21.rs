//! Day 21: Allergen Assessment
//!
//! ## Problem Description
//!
//! Part 1: Count how many times ingredients that cannot contain allergens appear across all foods.
//! Part 2: Determine which ingredient contains which allergen and return a canonical list sorted by allergen name.
//!
//! ## Solution Approach
//!
//! **Input Parsing**: Parses each food line into:
//! - Ingredients: Set of ingredient names (space-separated before "(contains")
//! - Allergens: Set of allergen names (comma-separated after "contains")
//!
//! **Part 1 Strategy**: Constraint elimination
//! - For each allergen, find ingredients that could potentially contain it by intersecting all ingredient sets from foods containing that allergen
//! - Collect all ingredients that could contain any allergen
//! - Count occurrences of ingredients that cannot contain any allergen
//!
//! **Part 2 Strategy**: Constraint solving via elimination
//! - Uses process of elimination to determine exact allergen-to-ingredient mapping
//! - Repeatedly finds allergens with only one possible ingredient and eliminates that ingredient from other allergen possibilities
//! - Sorts allergens alphabetically and returns corresponding ingredients as comma-separated list
//!
//! **Algorithm**: Constraint satisfaction problem solved using iterative elimination with smallest-domain-first heuristic.

use std::collections::{HashMap, HashSet};

/// Represents a food item with its ingredients and known allergens
#[derive(Debug, Clone)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

/// Parse the input string into a vector of Food items
fn parse_foods(input: &str) -> Vec<Food> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(" (contains ").collect();
            let ingredients: HashSet<String> =
                parts[0].split_whitespace().map(|s| s.to_string()).collect();

            let allergens: HashSet<String> = if parts.len() > 1 {
                parts[1]
                    .trim_end_matches(')')
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect()
            } else {
                HashSet::new()
            };

            Food {
                ingredients,
                allergens,
            }
        })
        .collect()
}

/// Find all possible ingredients that could contain each allergen
/// For each allergen, return the intersection of all ingredient sets from foods containing that allergen
fn find_possible_allergen_ingredients(
    foods: &[Food],
) -> HashMap<String, HashSet<String>> {
    let mut allergen_possibilities: HashMap<String, HashSet<String>> =
        HashMap::new();

    // For each allergen, find the intersection of all ingredient sets that contain it
    for food in foods {
        for allergen in &food.allergens {
            if let Some(current_possibilities) =
                allergen_possibilities.get_mut(allergen)
            {
                // Intersect with current ingredients
                *current_possibilities = current_possibilities
                    .intersection(&food.ingredients)
                    .cloned()
                    .collect();
            } else {
                // First time seeing this allergen
                allergen_possibilities
                    .insert(allergen.clone(), food.ingredients.clone());
            }
        }
    }

    allergen_possibilities
}

/// Solve which ingredient contains which allergen by process of elimination
/// Uses constraint solving: repeatedly find allergens with only one possible ingredient
fn solve_allergen_ingredients(
    mut possibilities: HashMap<String, HashSet<String>>,
) -> HashMap<String, String> {
    let mut solved: HashMap<String, String> = HashMap::new();

    while !possibilities.is_empty() {
        // Find an allergen with only one possible ingredient
        let mut found_unique = None;
        for (allergen, ingredients) in &possibilities {
            if ingredients.len() == 1 {
                let ingredient = ingredients.iter().next().unwrap().clone();
                found_unique = Some((allergen.clone(), ingredient));
                break;
            }
        }

        if let Some((allergen, ingredient)) = found_unique {
            // Remove this allergen from possibilities
            possibilities.remove(&allergen);
            solved.insert(allergen, ingredient.clone());

            // Remove this ingredient from all other allergen possibilities
            for (_, ingredients) in possibilities.iter_mut() {
                ingredients.remove(&ingredient);
            }
        } else {
            // This shouldn't happen with valid input
            break;
        }
    }

    solved
}

/// Part 1: Count how many times ingredients that cannot contain allergens appear
pub fn part_one(input: &str) -> usize {
    let foods = parse_foods(input);
    let allergen_possibilities = find_possible_allergen_ingredients(&foods);

    // Get all ingredients that could contain allergens
    let possible_allergen_ingredients: HashSet<String> =
        allergen_possibilities
            .values()
            .flat_map(|ingredients| ingredients.iter())
            .cloned()
            .collect();

    // Count occurrences of ingredients that cannot contain allergens
    let mut count = 0;
    for food in &foods {
        for ingredient in &food.ingredients {
            if !possible_allergen_ingredients.contains(ingredient) {
                count += 1;
            }
        }
    }

    count
}

/// Part 2: Return the canonical dangerous ingredient list (sorted by allergen name)
pub fn part_two(input: &str) -> String {
    let foods = parse_foods(input);
    let allergen_possibilities = find_possible_allergen_ingredients(&foods);
    let solved = solve_allergen_ingredients(allergen_possibilities);

    // Sort allergens alphabetically and get corresponding ingredients
    let mut allergen_ingredient_pairs: Vec<(String, String)> =
        solved.into_iter().collect();
    allergen_ingredient_pairs.sort_by(|a, b| a.0.cmp(&b.0));

    allergen_ingredient_pairs
        .into_iter()
        .map(|(_, ingredient)| ingredient)
        .collect::<Vec<String>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::read_example;

    #[test]
    fn example() {
        let input = read_example(21);
        assert_eq!(part_one(&input), 5);
        assert_eq!(part_two(&input), "mxmxvkd,sqjhc,fvjkl");
    }
}
