use anyhow::{anyhow, bail, Error, Result};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Error> {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let mut yard = parse_yard(&content)?;

    for line in content.split("\n").filter(|line| line.starts_with("move")) {
        yard.crane_lift(parse_action(line)?)?;
    }

    let tops: String = yard
        .stacks
        .iter()
        .map(|stack| stack.last().ok_or(anyhow!("empty stack")))
        .collect::<Result<_>>()?;
    println!("{}", tops);

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
        // Our internal stacks are zero indexed, adjust the index
        let from_stack = self
            .stacks
            .get_mut((action.from - 1) as usize)
            .ok_or(anyhow!("no such stack"))?;

        if action.count as usize > from_stack.len() {
            return Err(anyhow!("not enough crates remaining"));
        }

        let at = from_stack.len() - action.count as usize;
        let mut crates = from_stack.split_off(at);

        // Our internal stacks are zero indexed, adjust the index
        let to_stack = self
            .stacks
            .get_mut((action.to - 1) as usize)
            .ok_or(anyhow!("no such stack"))?;
        to_stack.append(&mut crates);

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Action {
    from: u32,
    to: u32,
    count: u32,
}

/// Parses a Yard from string input.
fn parse_yard(content: &str) -> Result<Yard> {
    let rows: Vec<_> = content
        .split("\n")
        .map_while(|line| if line.len() == 0 { None } else { Some(line) })
        .map(|line| parse_yard_line(line))
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
fn parse_yard_line(line: &str) -> Result<Vec<char>> {
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

fn parse_action(line: &str) -> Result<Action> {
    if !line.starts_with("move") {
        return Err(anyhow!("line does not start with the Action prefix"));
    }

    let components: Vec<_> = line.split(" ").collect();
    if components.len() != 6 {
        return Err(anyhow!("line has wrong number of components for an Action"));
    }

    return Ok(Action {
        count: components[1].parse()?,
        // Defined stack indices are not zero indexed
        from: components[3].parse::<u32>()?,
        to: components[5].parse::<u32>()?,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() -> Result<()> {
        assert_eq!(parse_yard_line("[A] [B]")?, ['A', 'B']);
        assert_eq!(parse_yard_line("[A]     [B]")?, ['A', ' ', 'B']);
        assert_eq!(parse_yard_line("    [A] [B]")?, [' ', 'A', 'B']);
        Ok(())
    }

    #[test]
    fn test_parse_action() -> Result<()> {
        assert_eq!(
            parse_action("move 1 from 2 to 3")?,
            Action {
                count: 1,
                from: 2,
                to: 3
            },
        );
        assert_eq!(
            parse_action("move 1000 from 20 to 30")?,
            Action {
                count: 1000,
                from: 20,
                to: 30
            },
        );
        Ok(())
    }

    #[test]
    fn test_parse_action_fails() {
        assert!(parse_action("notmove 5 from 2 to 3").is_err());
        assert!(parse_action("move 3 to 3").is_err());
        assert!(parse_action("move 3 from 3").is_err());
    }

    #[test]
    fn test_crane_lift() -> Result<()> {
        let get_tops = |yard: &Yard| -> Vec<char> {
            return yard
                .stacks
                .iter()
                .map(|stack| stack.last())
                .map(|last_or_none| match last_or_none {
                    Some(crate_) => *crate_,
                    None => ' ',
                })
                .collect();
        };

        // It can move crates
        {
            let mut yard = Yard {
                stacks: vec![vec!['A'], vec!['B']],
            };
            yard.crane_lift(parse_action("move 1 from 1 to 2")?)?;
            assert_eq!(get_tops(&yard), vec![' ', 'A']);
        }

        // It can move multiple crates
        {
            let mut yard = Yard {
                stacks: vec![vec!['A'], vec!['B', 'C', 'D']],
            };
            yard.crane_lift(parse_action("move 2 from 2 to 1")?)?;
            assert_eq!(get_tops(&yard), vec!['D', 'B']);
        }

        // It can move crates to empty stacks
        {
            let mut yard = Yard {
                stacks: vec![vec!['A'], vec![]],
            };
            yard.crane_lift(parse_action("move 1 from 1 to 2")?)?;
            assert_eq!(get_tops(&yard), vec![' ', 'A']);
        }

        // It fails to move a crate from an empty stack
        {
            let mut yard = Yard {
                stacks: vec![vec![], vec!['B']],
            };
            assert!(yard
                .crane_lift(parse_action("move 1 from 1 to 2")?)
                .is_err());
        }

        Ok(())
    }
}
