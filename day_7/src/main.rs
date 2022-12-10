use anyhow::{anyhow, Error, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

fn main() -> Result<(), Error> {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let fs = construct(&content)?;
    println!("{:?}", fs);

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
                            fs.insert(cwd.clone(), file);
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

fn canonicalize(p: &PathBuf) -> Result<PathBuf> {
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
