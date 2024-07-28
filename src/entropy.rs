use crate::index::Index;

pub struct Entropy {
    bit_flag: bool,
    freqs: [u64; 256],
}

impl Index for Entropy {
    fn new(bit_flag: bool) -> Self {
        Entropy {
            bit_flag,
            freqs: [0; 256],
        }
    }

    fn update(&mut self, bytes: &[u8]) {
        
        if self.bit_flag {
            let length = 8 * bytes.len() as u64;
            let zeros = bytes.into_iter().map(|x| x.count_zeros()).sum::<u32>() as u64;

            self.freqs[0] += zeros;
            self.freqs[1] += length - zeros;
        } else {
            for &b in bytes {
                self.freqs[b as usize] += 1;
            }
        }
    }

    fn get_value(&self) -> f64 {
        let total = self.freqs.into_iter().sum::<u64>() as f64;

        self.freqs
            .into_iter()
            .filter(|&x| x > 0)
            .map(|x| x as f64 / total)
            .fold(0.0, |e, x| e - (x * x.log2()))
    }
}
