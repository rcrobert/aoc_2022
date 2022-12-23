use anyhow::{anyhow, Error, Result};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    env_logger::init();

    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let forest = parse_forest(&content)?;
    let best_view = find_best_view(&forest);

    println!("{}", best_view);
    Ok(())
}

struct Forest {
    trees: Vec<Vec<TreeHeight>>,
}

type TreeHeight = i32;

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Forest {
    fn get_tree(&self, coordinate: Coordinate) -> Option<TreeHeight> {
        // trees are stored row-major
        self.trees
            .get(coordinate.y)
            .and_then(|row| row.get(coordinate.x))
            .and_then(|val| Some(*val))
    }

    fn rows(&self) -> impl Iterator<Item = &Vec<TreeHeight>> {
        self.trees.iter()
    }

    fn columns(&self) -> Columns {
        Columns {
            forest: self,
            index: 0,
        }
    }

    fn iter_direction(&self, location: Coordinate, direction: Direction) -> DirectionIterator {
        DirectionIterator {
            forest: self,
            direction: direction,
            loc: location,
        }
    }
}

struct DirectionIterator<'a> {
    forest: &'a Forest,
    direction: Direction,
    loc: Coordinate,
}

impl<'a> Iterator for DirectionIterator<'a> {
    type Item = TreeHeight;
    fn next(&mut self) -> Option<Self::Item> {
        match self.direction {
            Direction::North => {
                if self.loc.y == 0 {
                    return None;
                }
                self.loc = Coordinate::new(self.loc.x, self.loc.y - 1);
                return self.forest.get_tree(self.loc);
            }
            Direction::South => {
                self.loc = Coordinate::new(self.loc.x, self.loc.y + 1);
                return self.forest.get_tree(self.loc);
            }
            Direction::East => {
                self.loc = Coordinate::new(self.loc.x + 1, self.loc.y);
                return self.forest.get_tree(self.loc);
            }
            Direction::West => {
                if self.loc.x == 0 {
                    return None;
                }
                self.loc = Coordinate::new(self.loc.x - 1, self.loc.y);
                return self.forest.get_tree(self.loc);
            }
        }
    }
}

struct Columns<'a> {
    forest: &'a Forest,
    index: usize,
}

impl<'a> Iterator for Columns<'a> {
    type Item = Vec<TreeHeight>;
    fn next(&mut self) -> Option<Self::Item> {
        let next_item: Option<_> = self
            .forest
            .trees
            .iter()
            .map(|row| row.get(self.index).map(|inner| *inner))
            .collect();
        self.index += 1;
        return next_item;
    }
}

fn find_best_view(forest: &Forest) -> i32 {
    let directions = vec![
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    let mut max_beautiful_tree = Coordinate::new(0, 0);
    let mut max_beauty = 0;
    let mut max_counts = vec![];
    for (y, row) in forest.rows().enumerate() {
        for (x, _) in row.iter().enumerate() {
            let counts: Vec<_> = directions
                .iter()
                .map(|direction| count_visible_trees(forest, Coordinate::new(x, y), *direction))
                .collect();
            let beauty = directions
                .iter()
                .map(|direction| count_visible_trees(forest, Coordinate::new(x, y), *direction))
                .fold(1, |acc, count| acc * count);
            if beauty > max_beauty {
                max_beauty = beauty;
                max_beautiful_tree = Coordinate::new(x, y);
                max_counts = counts;
            }
        }
    }
    println!(
        "{:?} at {:?} {:?}",
        forest.get_tree(max_beautiful_tree),
        max_beautiful_tree,
        max_counts,
    );
    return max_beauty;
}

fn count_visible_trees(forest: &Forest, location: Coordinate, direction: Direction) -> i32 {
    let my_height = forest.get_tree(location).unwrap();

    let mut count = 0;
    for height in forest.iter_direction(location, direction) {
        count += 1;
        if height >= my_height {
            break;
        }
    }

    count
}

fn find_visible_trees(forest: &Forest) -> HashSet<Coordinate> {
    let mut visible_trees = HashSet::new();

    // Check for trees visible from East or West
    for (y, row) in forest.rows().enumerate() {
        // Visible from the West
        row.iter().enumerate().fold(-1, |tallest_tree, (x, tree)| {
            if *tree > tallest_tree {
                visible_trees.insert(Coordinate::new(x, y));
                return *tree;
            } else {
                return tallest_tree;
            }
        });

        // Visible from the East
        row.iter().enumerate().rfold(-1, |tallest_tree, (x, tree)| {
            if *tree > tallest_tree {
                visible_trees.insert(Coordinate::new(x, y));
                return *tree;
            } else {
                return tallest_tree;
            }
        });
    }

    // Check for trees visible from North or South
    for (x, column) in forest.columns().enumerate() {
        // Visible from the North
        column
            .iter()
            .enumerate()
            .fold(-1, |tallest_tree, (y, tree)| {
                if *tree > tallest_tree {
                    // println!("{:?} visible from North", Coordinate::new(x, y));
                    visible_trees.insert(Coordinate::new(x, y));
                    return *tree;
                } else {
                    // println!("{:?} not visible from North", Coordinate::new(x, y));
                    return tallest_tree;
                }
            });

        // Visible from the South
        column
            .iter()
            .enumerate()
            .rfold(-1, |tallest_tree, (y, tree)| {
                if *tree > tallest_tree {
                    // println!("{:?} visible from South", Coordinate::new(x, y));
                    visible_trees.insert(Coordinate::new(x, y));
                    return *tree;
                } else {
                    // println!("{:?} not visible from South", Coordinate::new(x, y));
                    return tallest_tree;
                }
            });
    }

    return visible_trees;
}

fn parse_forest(content: &str) -> Result<Forest> {
    Ok(Forest {
        trees: parse_input(content)?,
    })
}

fn parse_input(content: &str) -> Result<Vec<Vec<TreeHeight>>> {
    content.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(line: &str) -> Result<Vec<TreeHeight>> {
    line.chars()
        .map(|c| c.to_digit(10))
        .map(|opt| opt.ok_or(anyhow!("unrecognized non-digit character")))
        .map(|each| each.and_then(|u| i32::try_from(u).map_err(Error::from)))
        .collect()
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Coordinate {
        Coordinate { x: x, y: y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_parse_line() -> Result<()> {
        assert_eq!(parse_line("24680")?, vec![2, 4, 6, 8, 0]);
        assert_eq!(parse_line("1")?, vec![1]);
        assert!(parse_line("123a").is_err());
        Ok(())
    }

    #[test]
    fn test_parse_input() -> Result<()> {
        assert_eq!(parse_input("123\n456")?, vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert!(parse_input("123\n4a6").is_err());
        Ok(())
    }

    #[test]
    fn test_count_visible_trees() -> Result<()> {
        let f = parse_forest("111\n111\n111")?;
        assert_eq!(
            count_visible_trees(&f, Coordinate::new(0, 0), Direction::North),
            0
        );

        let f = parse_forest("111\n213\n111")?;
        assert_eq!(
            count_visible_trees(&f, Coordinate::new(0, 1), Direction::East),
            2
        );

        let f = parse_forest("111\n223\n111")?;
        assert_eq!(
            count_visible_trees(&f, Coordinate::new(0, 1), Direction::East),
            1
        );

        let f = parse_forest("111\n211\n111")?;
        assert_eq!(
            count_visible_trees(&f, Coordinate::new(0, 1), Direction::East),
            2
        );

        Ok(())
    }

    mod forest {
        use super::*;

        #[test]
        fn test_columns() -> Result<()> {
            let f = parse_forest("123\n456\n789")?;
            let mut it = f.columns();
            assert_eq!(it.next().unwrap(), vec![1, 4, 7]);
            assert_eq!(it.next().unwrap(), vec![2, 5, 8]);
            assert_eq!(it.next().unwrap(), vec![3, 6, 9]);
            assert!(it.next().is_none());

            Ok(())
        }

        #[test]
        fn test_iter_direction() -> Result<()> {
            let f = parse_forest("123\n456\n789")?;

            let mut it = f.iter_direction(Coordinate::new(1, 1), Direction::North);
            assert_eq!(it.next().unwrap(), 2);
            assert!(it.next().is_none());

            let mut it = f.iter_direction(Coordinate::new(1, 1), Direction::South);
            assert_eq!(it.next().unwrap(), 8);
            assert!(it.next().is_none());

            let mut it = f.iter_direction(Coordinate::new(1, 1), Direction::East);
            assert_eq!(it.next().unwrap(), 6);
            assert!(it.next().is_none());

            let mut it = f.iter_direction(Coordinate::new(1, 1), Direction::West);
            assert_eq!(it.next().unwrap(), 4);
            assert!(it.next().is_none());

            let mut it = f.iter_direction(Coordinate::new(0, 0), Direction::West);
            assert!(it.next().is_none());

            let mut it = f.iter_direction(Coordinate::new(0, 0), Direction::South);
            assert_eq!(it.next().unwrap(), 4);
            assert_eq!(it.next().unwrap(), 7);
            assert!(it.next().is_none());

            Ok(())
        }
    }

    #[test]
    fn test_find_visible_trees() -> Result<()> {
        let f = parse_forest("111\n121\n111")?;
        assert_eq!(find_visible_trees(&f).len(), 9);

        Ok(())
    }
}
