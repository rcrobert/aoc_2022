use crate::cpu::Cpu;
use anyhow::anyhow;
use std::fmt;

pub struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<char>,
}

impl Screen {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels = Vec::new();
        pixels.resize(width * height, ' ');
        Self {
            width: width,
            height: height,
            pixels: pixels,
        }
    }

    pub fn draw(&mut self, cpu: &Cpu) {
        let sprite_center = cpu.register;
        let cursor_x_pos = cpu.current_cycle % self.width;
        let is_visible = (sprite_center - cursor_x_pos as i32).abs() <= 1;

        if is_visible {
            self.pixels[cpu.current_cycle] = '#';
        } else {
            self.pixels[cpu.current_cycle] = '.';
        }
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pixels
            .chunks_exact(self.width)
            .map(|line| line.iter().collect::<String>())
            .map(|line| writeln!(f, "{}", line))
            .collect::<fmt::Result>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod draw {
        use super::*;

        #[test]
        fn test_sprite_width() {
            let mut cpu = Cpu::new(vec![]);

            let mut screen = Screen::new(11, 1);
            cpu.register = 5;
            for i in 0..screen.pixels.len() {
                cpu.current_cycle = i;
                screen.draw(&cpu);
            }
            assert_eq!(screen.pixels.iter().collect::<String>(), "....###....");
        }

        #[test]
        fn test_row_wraps() {
            let mut cpu = Cpu::new(vec![]);

            let mut screen = Screen::new(11, 2);
            cpu.register = 3;
            for i in 0..11 {
                cpu.current_cycle = i;
                screen.draw(&cpu);
            }
            cpu.register = 7;
            for i in 11..screen.pixels.len() {
                cpu.current_cycle = i;
                screen.draw(&cpu);
            }
            assert_eq!(
                screen.pixels[..11].iter().collect::<String>(),
                "..###......"
            );
            assert_eq!(
                screen.pixels[11..].iter().collect::<String>(),
                "......###.."
            );
        }
    }
}
