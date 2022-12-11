use anyhow::{anyhow, Error, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

const MAX_SIZE: u64 = 100000;
const TOTAL_SPACE: u64 = 70000000;
const MINIMUM_SPACE: u64 = 30000000;

fn main() -> Result<(), Error> {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let fs = construct(&content)?;

    let sizes = calculate_sizes(&fs);

    let used_space = sizes[Path::new("/")];
    let free_space = TOTAL_SPACE - used_space;
    let space_to_free = MINIMUM_SPACE - free_space;

    let (optimal_delete, _) = sizes
        .iter()
        // Don't consider directories that are too small
        .filter(|(_, size)| **size >= space_to_free)
        // Consider error from space_to_free
        .map(|(path, size)| (path, *size - space_to_free))
        // Fold by keeping the lesser error path
        .fold(
            (Path::new(""), TOTAL_SPACE),
            |(best_path, best_error), (path, error)| {
                if error < best_error {
                    return (path.as_path(), error);
                } else {
                    return (best_path, best_error);
                }
            },
        );

    println!(
        "Delete {} (size {}), to free at least {}",
        optimal_delete.display(),
        sizes[optimal_delete],
        space_to_free,
    );

    Ok(())
}

type FileSystem = BTreeMap<PathBuf, File>;

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: u64,
}

fn construct(content: &str) -> Result<FileSystem> {
    let mut fs = FileSystem::new();

    let mut cwd = Path::new("/").to_path_buf();
    let mut lines = content.split("\n").peekable();

    while let Some(line) = lines.next() {
        if line.len() == 0 {
            continue;
        }

        // Command-specific code should finish its job, expect to find a new start-of-command
        if !line.starts_with("$") {
            return Err(anyhow!("expected to parse a command, got '{}'", line));
        }

        let components: Vec<_> = line.split(" ").collect();
        if components.len() < 2 {
            return Err(anyhow!("invalid command '{}'", line));
        }

        match components[1] {
            "cd" => {
                cwd.push(components[2]);
                cwd = canonicalize(&cwd)?;
            }
            "ls" => {
                while let Some(next_line) = lines.peek() {
                    // If we see the next command, then we are done consuming `ls` outputs
                    if next_line.starts_with("$") || next_line.len() == 0 {
                        break;
                    }

                    let components: Vec<_> = next_line.split(" ").collect();
                    match components[0] {
                        "dir" => (),
                        _ => {
                            let file = File {
                                size: components[0].parse()?,
                                name: String::from(components[1]),
                            };
                            let mut file_path = cwd.clone();
                            file_path.push(&file.name);
                            fs.insert(file_path, file);
                        }
                    }

                    // Advance overall parser
                    lines.next();
                }
            }
            _ => return Err(anyhow!("invalid command '{}'", line)),
        }
    }

    Ok(fs)
}

fn canonicalize(p: &Path) -> Result<PathBuf> {
    if !p.is_absolute() {
        return Err(anyhow!("cannot do relative paths"));
    }

    let mut r = PathBuf::new();
    for component in p.components() {
        match component {
            Component::RootDir => r.push("/"),
            Component::Normal(dir) => r.push(dir),
            Component::ParentDir => {
                r.pop();
            }
            Component::CurDir => (),
            Component::Prefix(_) => panic!("no prefixes on unix"),
        }
    }
    return Ok(r);
}

fn calculate_sizes(fs: &FileSystem) -> BTreeMap<PathBuf, u64> {
    let mut sizes: BTreeMap<PathBuf, u64> = BTreeMap::new();
    for (path, file) in fs {
        // Skip 1 in ancestors because that is the file itself, not the directory
        for ancestor in path.ancestors().skip(1) {
            match sizes.get_mut(ancestor) {
                Some(val) => {
                    *val += file.size;
                }
                None => {
                    sizes.insert(ancestor.to_path_buf(), file.size);
                }
            }
        }
    }
    return sizes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalize() -> Result<()> {
        assert_eq!(
            canonicalize(Path::new("/parent/child/.."))?,
            Path::new("/parent")
        );
        assert_eq!(
            canonicalize(Path::new("/parent/child/."))?,
            Path::new("/parent/child")
        );
        assert_eq!(
            canonicalize(Path::new("/parent/../aunt/child/."))?,
            Path::new("/aunt/child")
        );
        assert_eq!(canonicalize(Path::new("/parent/.."))?, Path::new("/"));
        assert_eq!(canonicalize(Path::new("//.."))?, Path::new("/"));

        Ok(())
    }
}
