use std::{
    fs::File,
    io::{self, Error, ErrorKind, Read, Write},
    num::ParseIntError,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Image {
    pixels: Vec<u8>,
    width: usize,
}

impl Image {
    pub fn open(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut format = [0u8; 2];

        file.read_exact(&mut format)?;

        if &format != b"P2" {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        let mut buf = String::new();

        file.read_to_string(&mut buf)?;

        let mut data = buf
            .split_whitespace()
            .map(|value| value.parse::<usize>())
            .collect::<Result<Vec<usize>, ParseIntError>>()
            .map_err(|_| Error::from(ErrorKind::InvalidData))?;

        if data.len() < 4 {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        let (width, height, max) = (data.remove(0), data.remove(0), data.remove(0));

        if width * height != data.len() || max > 255 {
            return Err(Error::from(ErrorKind::InvalidData));
        }

        let pixels = data.into_iter().map(|pixel| pixel as u8).collect();

        Ok(Self { pixels, width })
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.pixels.len() / self.width
    }

    fn get_pixel(&self, position: usize, rows: isize, columns: isize) -> Option<u8> {
        let index = position as isize + rows * self.width as isize + columns;

        if index >= 0 {
            self.pixels.get(index as usize).map(|pixel| *pixel)
        } else {
            None
        }
    }

    pub fn convolve(&self, kernel: &Kernel) -> Self {
        let mut pixels = Vec::new();

        for (position, pixel) in self.pixels.iter().enumerate() {
            let mut sum = 0.;

            if let Some(value) = self.get_pixel(position, -1, -1) {
                sum += value as f64 * kernel[2][2];
            }

            if let Some(value) = self.get_pixel(position, -1, 0) {
                sum += value as f64 * kernel[2][1];
            }

            if let Some(value) = self.get_pixel(position, -1, 1) {
                sum += value as f64 * kernel[2][0];
            }

            if let Some(value) = self.get_pixel(position, 0, -1) {
                sum += value as f64 * kernel[1][2];
            }

            sum += *pixel as f64 * kernel[1][1];

            if let Some(value) = self.get_pixel(position, 0, 1) {
                sum += value as f64 * kernel[1][0];
            }

            if let Some(value) = self.get_pixel(position, 1, -1) {
                sum += value as f64 * kernel[0][2];
            }

            if let Some(value) = self.get_pixel(position, 1, 0) {
                sum += value as f64 * kernel[0][1];
            }

            if let Some(value) = self.get_pixel(position, 1, 1) {
                sum += value as f64 * kernel[0][0];
            }

            pixels.push(sum as u8);
        }

        Self { pixels, width: self.width }
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;
        let (width, height) = (self.width, self.height());

        writeln!(&mut file, "P2\n{width} {height}\n255")?;

        for rows in self.pixels.chunks(width) {
            for pixel in rows {
                write!(&mut file, "{pixel} ")?;
            }

            writeln!(&mut file)?;
        }

        Ok(())
    }
}

type Kernel = [[f64; 3]; 3];