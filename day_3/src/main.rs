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
        .map(|line| {
            let middle = line.len() / 2;
            get_duplicate_item_priority(&line[..middle], &line[middle..])
        })
        .sum();
    println!("Total: {}", priority_sum);
}

/// Returns the priority of the item found in both compartments.
///
/// Panics if not exactly one duplicate is found.
fn get_duplicate_item_priority(l: &str, r: &str) -> u32 {
    let l_items = find_items(l);
    let r_items = find_items(r);

    let masked = l_items & r_items;
    if masked != 0 {
        // Just in case, panic if there are multiple duplicates.
        if (masked ^ (0b1 << masked.trailing_zeros())) != 0 {
            panic!("multiple duplicate items found: {} {}", l, r);
        }
        // Since a is priority 1 and represented as 0b10, trailing_zeros() equals the priority.
        return masked.trailing_zeros();
    } else {
        panic!("no duplicate items found: {} {}", l, r);
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
        assert_eq!(get_duplicate_item_priority("abc", "ABc"), 3);
        assert_eq!(get_duplicate_item_priority("abC", "CBA"), 29);
    }

    #[test]
    #[should_panic]
    fn test_get_duplicate_item_priority__panics_on_multiple_duplicates() {
        get_duplicate_item_priority("abc", "dab");
    }
}
