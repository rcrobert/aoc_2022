use anyhow::{anyhow, Error, Result};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Error> {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let mut match_idx = 0;
    let mut window_end = 4;
    loop {
        let window = &content[window_end - 4..window_end];
        let mut found_chars = 0u64;
        let mut dupe = false;
        for b in window.chars() {
            let mask = 1 << (b as u32 - 'a' as u32);
            if mask & found_chars != 0 {
                dupe = true;
                break;
            } else {
                found_chars |= mask;
            }
        }

        if !dupe {
            match_idx = window_end;
            break;
        }

        window_end += 1;
    }

    match match_idx {
        0 => println!("no sequence found"),
        _ => println!("sequence ends at {}", match_idx),
    }

    Ok(())
}
