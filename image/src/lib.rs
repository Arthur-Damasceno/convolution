use std::{
    fs::File,
    io::{self, Error, ErrorKind, Read},
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
}
