use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

mod cpu;
mod screen;
use cpu::{parse_instruction, Cpu};
use screen::Screen;

fn main() -> Result<()> {
    let input_path = Path::new("input.txt");

    let content = match fs::read_to_string(input_path) {
        Err(why) => panic!("failed to open {}: {}", input_path.display(), why),
        Ok(content) => content,
    };

    let program: Vec<_> = content
        .lines()
        .map(parse_instruction)
        .collect::<Result<_>>()?;

    let mut cpu = Cpu::new(program);
    let mut screen = Screen::new(40, 6);
    loop {
        screen.draw(&cpu);

        let done = cpu.step().is_none();
        if done {
            break;
        }
    }

    println!("{}", screen);

    Ok(())
}
