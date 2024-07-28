use crate::index::Index;

pub struct Mean {
    bit_flag: bool,
    accumulator: u64,
    counter: u64,
}

impl Index for Mean {
    fn new(bit_flag: bool) -> Self {
        Mean {
            bit_flag,
            accumulator: 0,
            counter: 0,
        }
    }

    fn update(&mut self, bytes: &[u8]) {
        self.counter += if self.bit_flag {
            8 * bytes.len()
        } else {
            bytes.len()
        } as u64;

        self.accumulator += if self.bit_flag {
            bytes.into_iter().map(|&x| x.count_ones()).sum::<u32>()
        } else {
            bytes.into_iter().map(|&x| x as u32).sum()
        } as u64;
    }

    fn get_value(&self) -> f64 {
        self.accumulator as f64 / self.counter as f64
    }
}
