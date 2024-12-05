use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(5);

// Store rules as a map of each page to the set of pages that are required to be before it.
fn parse_rules_and_updates(input: &str) -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut updates = Vec::new();
    let mut end_of_rules = false;
    for line in input.lines() {
        if line.is_empty() {
            end_of_rules = true;
            continue;
        }

        if !end_of_rules {
            let (a, b) = parse_rule(line);
            rules.entry(b).or_default().insert(a);
        } else {
            updates.push(parse_update(line));
        }
    }

    (rules, updates)
}

fn parse_rule(rule: &str) -> (u32, u32) {
    rule.split("|")
        .map(str::parse)
        .map(|res| res.expect("Could not parse u32 from rule"))
        .collect_tuple()
        .expect("Rule does not contain 2 numbers")
}

fn parse_update(update: &str) -> Vec<u32> {
    update
        .split(",")
        .map(str::parse)
        .map(|res| res.expect("Could not parse u32 from update"))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse_rules_and_updates(input);

    let mut sum = 0;
    for update in updates {
        let pages_set = update.clone().into_iter().collect::<HashSet<_>>();

        let mut update_ok = true;
        for (i, page) in update.iter().enumerate() {
            // The correct idx of each page should be equal to
            // the number of pages in this update that are required to be before it
            let idx = match rules.get(page) {
                None => 0, // no pages required to be before this so it should be the first
                Some(page_befores) => pages_set.intersection(page_befores).count(),
            };

            if idx != i {
                update_ok = false;
                break;
            }
        }

        if update_ok {
            // add middle page
            sum += update[update.len() / 2];
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse_rules_and_updates(input);

    let mut sum = 0;
    for update in updates {
        let pages_set = update.clone().into_iter().collect::<HashSet<_>>();

        let mut reordered = false;
        let mut reordered_page = vec![0; update.len()];

        for (i, page) in update.iter().enumerate() {
            let idx = match rules.get(page) {
                None => 0, // no rules requiring anything to be before this page
                Some(rule_befores) => {
                    // The correct index of this page is equal to the number of pages in
                    // this update that must be before this page
                    pages_set.intersection(rule_befores).count()
                }
            };

            if i != idx {
                reordered = true;
            }
            reordered_page[idx] = *page;
        }

        if reordered {
            sum += reordered_page[update.len() / 2];
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
