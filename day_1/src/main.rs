use std::fs;
use std::path::Path;
use std::cmp;

fn main() {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let max_calories = parse_maximum_calories(&content);

    println!("{}", max_calories);
}

fn parse_maximum_calories(content: &str) -> i32 {
    let mut max: i32 = 0;
    let mut current_sum: i32 = 0;
    for line in content.split("\n") {
        match line {
            "" => {
                current_sum = 0;
            },
            _ => {
                let val = match line.parse::<i32>() {
                    Err(why) => panic!("failed to parse {}: {}", line, why),
                    Ok(val) => val,
                };
                current_sum += val;
                max = cmp::max(current_sum, max);
            },
        }
    }
    return max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_maximum_calories() {
        assert_eq!(parse_maximum_calories("2\n2\n\n5"), 5);
        assert_eq!(parse_maximum_calories("2\n2\n\n5\n"), 5);
        assert_eq!(parse_maximum_calories("2\n2"), 4);
    }
}
