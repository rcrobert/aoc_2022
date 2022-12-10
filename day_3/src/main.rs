#![feature(iter_array_chunks)]

use std::fs;
use std::path::Path;

fn main() {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let priority_sum: u32 = content
        .split("\n")
        .filter(|line| line.len() > 0)
        .array_chunks::<3>()
        .map(|chunk| get_duplicate_item_priority(&chunk))
        .sum();
    println!("Total: {}", priority_sum);
}

/// Returns the priority of the item found in all compartments.
///
/// Panics if not exactly one duplicate is found.
fn get_duplicate_item_priority(containers: &[&str]) -> u32 {
    let common_items = containers
        .iter()
        .map(|each| find_items(each))
        .fold(!0u64, |acc, each| acc & each);
    if common_items != 0 {
        // Just in case, panic if there are multiple duplicates.
        if (common_items ^ (0b1 << common_items.trailing_zeros())) != 0 {
            panic!("multiple duplicate items found");
        }
        // Since a is priority 1 and represented as 0b10, trailing_zeros() equals the priority.
        return common_items.trailing_zeros();
    } else {
        panic!("no duplicate items found");
    }
}

/// Returns the items inside a compartment.
///
/// Items are represented by set bits in the result.
/// * Items 'a' - 'z' are indices 1-26.
/// * Items 'A' - 'Z' are indices 27-52.
fn find_items(compartment: &str) -> u64 {
    compartment
        .chars()
        .map(|c| match c {
            'a'..='z' => c as u8 - 'a' as u8 + 1u8,
            'A'..='Z' => c as u8 - 'A' as u8 + 27u8,
            _ => panic!("unhandled item"),
        })
        .fold(0u64, |acc, idx| acc | 0b1 << idx)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_items() {
        assert_eq!(find_items("abc"), 0b1110);
        assert_eq!(find_items("aa"), 0b10);
        assert_eq!(find_items("Aa"), 0b1 << 27 | 0b1 << 1);
    }

    #[test]
    fn test_get_duplicate_item_priority() {
        assert_eq!(get_duplicate_item_priority(&["abc", "ABc"]), 3);
        assert_eq!(get_duplicate_item_priority(&["abC", "CBA"]), 29);
    }

    #[test]
    #[should_panic]
    fn test_get_duplicate_item_priority_panics_on_multiple_duplicates() {
        get_duplicate_item_priority(&["abc", "dab"]);
    }
}
