use anyhow::{anyhow, bail, Error, Result};
use std::fs;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let num_overlaps: Result<u32> = content
        .split("\n")
        .filter(|line| line.len() > 0)
        .map(|line| parse_assignments(line))
        .map(|result: Result<Vec<Assignment>>| {
            result.map(|assignments| {
                assignments[0].contains(&assignments[1]) || assignments[1].contains(&assignments[0])
            })
        })
        .try_fold(0, |acc, result| {
            result.map(|contains| if contains { acc + 1 } else { acc })
        });

    match num_overlaps {
        Ok(num_overlaps) => println!("Total: {}", num_overlaps),
        Err(why) => panic!("{}", why),
    }
}

fn parse_assignments(s: &str) -> Result<Vec<Assignment>> {
    s.split(",")
        .map(|each| each.parse::<Assignment>())
        .collect()
}

/// An Assignment is an inclusive range of sections.
#[derive(Clone, Debug, PartialEq)]
struct Assignment {
    start: u32,
    end: u32,
}

impl Assignment {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && self.start >= other.start
            || self.end <= other.end && self.end >= other.start
    }
}

impl FromStr for Assignment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut result = Self { start: 0, end: 0 };

        let inclusive_bounds: Vec<&str> = s.split("-").collect();

        if let Ok(start) = inclusive_bounds[0].parse::<u32>() {
            result.start = start;
        } else {
            bail!("failed to parse assignment '{}'", s);
        }

        if let Ok(end) = inclusive_bounds[1].parse::<u32>() {
            result.end = end;
        } else {
            bail!("failed to parse assignment '{}'", s);
        }

        return Ok(result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assignment_fromstr() -> Result<()> {
        assert_eq!(
            "1-2".parse::<Assignment>()?,
            Assignment { start: 1, end: 2 }
        );

        assert_eq!(
            "10-2000".parse::<Assignment>()?,
            Assignment {
                start: 10,
                end: 2000
            }
        );

        Ok(())
    }

    #[test]
    fn test_assignment_fromstr_fails() {
        assert!("a-b".parse::<Assignment>().is_err());
    }
}
