use crate::types::Color;
use std::io::Write;

pub struct Screen {
    pub(crate) screen_data: Vec<Color>,
    width: usize,
    height: usize,
}

impl Screen {
    pub fn from(width: usize, height: usize) -> Self {
        Self {
            screen_data: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        self.screen_data[x + y * self.width]
    }

    pub fn set(&mut self, c: Color, x: usize, y: usize) {
        self.screen_data[x + y * self.width] = c;
    }

    pub fn write(&self, path: &str) {
        let file = std::fs::File::create(path).unwrap();
        let mut writer = std::io::BufWriter::new(file);

        writeln!(writer, "P3").unwrap();
        writeln!(writer, "{} {}", self.width, self.height).unwrap();
        writeln!(writer, "255").unwrap();

        for j in 0..self.height {
            for i in 0..self.width {
                let pixel_color = self.get(i, j);

                writeln!(
                    writer,
                    "{} {} {}",
                    pixel_color.x, pixel_color.y, pixel_color.z
                )
                .unwrap();
            }
        }
    }
}
