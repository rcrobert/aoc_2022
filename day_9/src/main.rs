use anyhow::{anyhow, Error, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let steps: Vec<(Direction, usize)> = content
        .lines()
        .enumerate()
        .map(|(line_num, line)| (line_num, parse_line(line)))
        .map(|(line_num, r)| match r {
            Err(why) => Err(anyhow!("failed to parse line {}: {}", line_num + 1, why)),
            Ok(val) => Ok(val),
        })
        .collect::<Result<_>>()?;

    let tail_positions = calculate_tail_positions(steps.into_iter())?;

    println!("{}", tail_positions.len());

    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn calculate_tail_positions<T>(steps: T) -> Result<HashSet<Coordinate>>
where
    T: Iterator<Item = (Direction, usize)>,
{
    let mut rope = Rope::new(10);

    let mut tail_positions = HashSet::new();

    for (direction, distance) in steps {
        for _ in 0..distance {
            rope.move_head(direction);
            tail_positions.insert(*rope.segments.last().unwrap());
        }
    }

    Ok(tail_positions)
}

fn parse_line(line: &str) -> Result<(Direction, usize)> {
    let components: Vec<_> = line.split(" ").collect();
    if components.len() != 2 {
        return Err(anyhow!(
            "expected 2 compoents on a line but found {}",
            components.len()
        ));
    }

    let distance = components[1].parse::<usize>()?;
    let direction = match components[0] {
        "U" => Ok(Direction::Up),
        "D" => Ok(Direction::Down),
        "L" => Ok(Direction::Left),
        "R" => Ok(Direction::Right),
        _ => Err(anyhow!("unexpected direction {}", components[0])),
    }?;

    Ok((direction, distance))
}

fn move_knot(location: Coordinate, direction: Direction) -> Coordinate {
    match direction {
        Direction::Up => Coordinate::new(location.x, location.y + 1),
        Direction::Down => Coordinate::new(location.x, location.y - 1),
        Direction::Left => Coordinate::new(location.x - 1, location.y),
        Direction::Right => Coordinate::new(location.x + 1, location.y),
    }
}

fn update_tail(mut tail: Coordinate, head: Coordinate) -> Coordinate {
    if tail.is_adjacent(&head) {
        return tail;
    }

    if head.x > tail.x {
        tail.x += 1;
    } else if head.x < tail.x {
        tail.x -= 1;
    }

    if head.y > tail.y {
        tail.y += 1;
    } else if head.y < tail.y {
        tail.y -= 1;
    }

    tail
}

#[derive(Clone, Debug)]
struct Rope {
    segments: Vec<Coordinate>,
}

impl Rope {
    fn new(length: usize) -> Self {
        Self {
            segments: (0..length).map(|_| Coordinate::new(0, 0)).collect(),
        }
    }

    fn move_head(&mut self, direction: Direction) {
        let head = move_knot(self.segments[0], direction);

        let mut new_segments = vec![head];
        let mut last_segment = head;
        for segment in self.segments.iter().skip(1) {
            let new_segment = update_tail(*segment, last_segment);
            last_segment = new_segment;
            new_segments.push(new_segment);
        }

        self.segments.swap_with_slice(&mut new_segments);
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x: x, y: y }
    }

    fn is_adjacent(&self, other: &Coordinate) -> bool {
        ((self.x - other.x).abs() <= 1) && ((self.y - other.y).abs() <= 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_knot() {
        let k = Coordinate::new(0, 0);
        assert_eq!(move_knot(k, Direction::Up), Coordinate::new(0, 1));
        assert_eq!(move_knot(k, Direction::Down), Coordinate::new(0, -1));
        assert_eq!(move_knot(k, Direction::Left), Coordinate::new(-1, 0));
        assert_eq!(move_knot(k, Direction::Right), Coordinate::new(1, 0));
    }

    #[test]
    fn test_update_tail() {
        let tail = Coordinate::new(0, 0);

        assert_eq!(
            update_tail(tail, Coordinate::new(0, 0)),
            Coordinate::new(0, 0)
        );

        assert_eq!(
            update_tail(tail, Coordinate::new(1, 0)),
            Coordinate::new(0, 0)
        );

        assert_eq!(
            update_tail(tail, Coordinate::new(1, 1)),
            Coordinate::new(0, 0)
        );

        assert_eq!(
            update_tail(tail, Coordinate::new(5, 0)),
            Coordinate::new(1, 0)
        );

        assert_eq!(
            update_tail(tail, Coordinate::new(-5, -1)),
            Coordinate::new(-1, -1)
        );
    }
}
