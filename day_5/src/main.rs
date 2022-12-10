use anyhow::{anyhow, bail, Error, Result};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Error> {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let yard = parse_yard(&content)?;

    for stack in yard.stacks {
        println!("{:?}", stack);
    }

    Ok(())
}

type Stack = Vec<char>;

#[derive(Clone, Debug)]
struct Yard {
    stacks: Vec<Stack>,
}

impl Yard {
    fn new() -> Self {
        Self { stacks: Vec::new() }
    }

    fn crane_lift(&mut self, action: Action) -> Result<()> {
        for _ in 0..action.count {
            let c = self.stacks[action.source as usize]
                .pop()
                .ok_or(anyhow!("no crate in stack {}", action.source))?;

            self.stacks[action.destination as usize].push(c);
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Action {
    source: u32,
    destination: u32,
    count: u32,
}

/// Parses a Yard from string input.
fn parse_yard(content: &str) -> Result<Yard> {
    let rows: Vec<_> = content
        .split("\n")
        .map_while(|line| if line.len() == 0 { None } else { Some(line) })
        .map(|line| parse_line(line))
        .collect::<Result<_>>()?;

    let mut yard = Yard::new();

    // The bottom row must be the maximum size
    let num_stacks = rows.last().ok_or(anyhow!("no rows found"))?.len();
    for _ in 0..num_stacks {
        yard.stacks.push(Vec::new());
    }

    // Stacks were read top down, reverse them to build them in the right order; so the first row is the top of each stack.
    // The last row is also not defining crates but the stacks' IDs, skip it.
    for row in rows.iter().rev().skip(1) {
        row.iter().enumerate().try_for_each(|(i, c)| match c {
            'A'..='Z' => {
                yard.stacks[i].push(*c);
                Ok(())
            }
            ' ' => Ok(()),
            _ => Err(anyhow!("invalid crate {}", c)),
        })?;
    }

    Ok(yard)
}

/// Parses one row from a Yard's string representation.
fn parse_line(line: &str) -> Result<Vec<char>> {
    let mut stack_idx = 0;
    let mut row = Vec::new();
    loop {
        let pos = stack_idx * 4;

        if pos > line.len() {
            break;
        }

        let next_crate = &line[pos..pos + 3];
        row.push(
            next_crate
                .chars()
                .nth(1)
                .ok_or(anyhow!("failed to read crate"))?,
        );

        stack_idx += 1;
    }
    Ok(row)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() -> Result<()> {
        assert_eq!(parse_line("[A] [B]")?, ['A', 'B']);
        assert_eq!(parse_line("[A]     [B]")?, ['A', ' ', 'B']);
        assert_eq!(parse_line("    [A] [B]")?, [' ', 'A', 'B']);
        Ok(())
    }
}
