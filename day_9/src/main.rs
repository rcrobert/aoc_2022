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
    let mut head = Coordinate::new(0, 0);
    let mut tail = Coordinate::new(0, 0);
    let mut tail_positions = HashSet::new();

    tail_positions.insert(Coordinate::new(0, 0));

    for (direction, distance) in steps {
        for _ in 0..distance {
            head = move_knot(head, direction);
            tail = update_tail(tail, head);
            tail_positions.insert(tail);
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

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Coordinate {
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
    fn test_example() -> Result<()> {
        let steps = vec![
            (Direction::Right, 4),
            (Direction::Up, 4),
            (Direction::Left, 3),
            (Direction::Down, 1),
            (Direction::Right, 4),
            (Direction::Down, 1),
            (Direction::Left, 5),
            (Direction::Right, 2),
        ];
        let expected = HashSet::from_iter(
            vec![
                Coordinate::new(0, 0),
                Coordinate::new(1, 0),
                Coordinate::new(2, 0),
                Coordinate::new(3, 0),
                Coordinate::new(4, 1),
                Coordinate::new(1, 2),
                Coordinate::new(2, 2),
                Coordinate::new(3, 2),
                Coordinate::new(4, 2),
                Coordinate::new(3, 3),
                Coordinate::new(4, 3),
                Coordinate::new(2, 4),
                Coordinate::new(3, 4),
            ]
            .into_iter(),
        );
        assert_eq!(calculate_tail_positions(steps.into_iter())?, expected);

        Ok(())
    }

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
