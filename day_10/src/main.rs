use anyhow::{anyhow, Result};
use std::fs;
use std::path::Path;

mod cpu;
use cpu::{parse_instruction, Cpu};

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

    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut signal_strength = 0;

    let mut cpu = Cpu::new(program);
    loop {
        let done = cpu.step().is_none();
        if done {
            break;
        }
        if interesting_cycles.contains(&cpu.current_cycle) {
            signal_strength += cpu.current_cycle as i32 * cpu.register;
        }
    }

    println!("Signal strength: {}", signal_strength);
    Ok(())
}
