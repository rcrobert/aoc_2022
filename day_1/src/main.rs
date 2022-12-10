use std::cmp;
use std::fs;
use std::path::Path;

fn main() {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let mut calories_per_elf = parse_calorie_totals(&content);
    calories_per_elf.sort();
    calories_per_elf.reverse();

    println!("{}", calories_per_elf[..3].iter().sum::<i32>());
}

fn parse_maximum_calories(content: &str) -> i32 {
    let mut max: i32 = 0;
    let mut current_sum: i32 = 0;
    for line in content.split("\n") {
        match line {
            "" => {
                current_sum = 0;
            }
            _ => {
                let val = match line.parse::<i32>() {
                    Err(why) => panic!("failed to parse {}: {}", line, why),
                    Ok(val) => val,
                };
                current_sum += val;
                max = cmp::max(current_sum, max);
            }
        }
    }
    return max;
}

fn parse_calorie_totals(content: &str) -> Vec<i32> {
    let mut totals: Vec<i32> = vec![0];
    for line in content.split("\n") {
        match line {
            "" => {
                totals.push(0);
            }
            _ => {
                let val = match line.parse::<i32>() {
                    Err(why) => panic!("failed to parse {}: {}", line, why),
                    Ok(val) => val,
                };
                if let Some(current_sum) = totals.last_mut() {
                    *current_sum += val;
                }
            }
        }
    }
    return totals;
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

    #[test]
    fn test_parse_calorie_totals() {
        assert_eq!(parse_calorie_totals("2\n2\n\n5"), [4, 5]);
        assert_eq!(parse_calorie_totals("2\n2\n\n5\n"), [4, 5, 0]);
        assert_eq!(parse_calorie_totals("2\n2"), [4]);
    }
}
