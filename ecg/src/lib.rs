use std::{
    fs::File,
    io::{self, Error, ErrorKind, Read, Write},
    num::ParseFloatError,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct ECGData(Vec<f64>);

impl ECGData {
    pub const MEAN_KERNEL_SIZE: usize = 5;

    pub fn read(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut data = String::new();

        file.read_to_string(&mut data)?;

        let data = data
            .split('\n')
            .skip(1)
            .map(|value| value.parse::<f64>())
            .collect::<Result<Vec<f64>, ParseFloatError>>()
            .map_err(|_| Error::from(ErrorKind::InvalidData))?;

        Ok(Self(data))
    }

    pub fn convolve(&self, kernel: &[f64]) -> Self {
        let mut data = Vec::with_capacity(self.0.len() + kernel.len() - 1);

        for i in 0..self.0.len() - 1 {
            let max = if i >= kernel.len() { kernel.len() } else { i + 1 };
            let mut sum = 0.;

            for j in 0..max {
                sum += self.0[i - j] * kernel[j];
            }

            data.push(sum);
        }

        for i in 0..kernel.len() {
            let mut sum = 0.;

            for j in i..kernel.len() {
                sum += self.0[self.0.len() + i - j - 1] * kernel[j];
            }

            data.push(sum);
        }

        Self(data)
    }

    pub fn mean_filter(&self) -> Self {
        let kernel = [1. / (Self::MEAN_KERNEL_SIZE as f64); Self::MEAN_KERNEL_SIZE];

        let mut data = self.convolve(&kernel);

        data.0.drain(0..(Self::MEAN_KERNEL_SIZE / 2));
        data.0.truncate(self.0.len());

        data
    }

    pub fn save(&self, path: &str) -> io::Result<()> {
        let mut file = File::create(path)?;

        writeln!(file, "{}", self.0.len())?;

        for value in &self.0 {
            writeln!(file, "{value}")?;
        }

        file.flush()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convolve() {
        let data = ECGData(vec![1., 2., 3.]);

        let convolution = data.convolve(&[4., 5., 6.]);

        assert_eq!(convolution.0, vec![4., 13., 28., 27., 18.]);
    }

    #[test]
    fn mean_filter() {
        let data = ECGData(vec![1., 2.5, 4., 2.5, 5.]);

        let mean = data.mean_filter();

        assert_eq!(mean.0, vec![1.5, 2., 3., 2.8, 2.3]);
    }
}
