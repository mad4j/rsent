pub trait Index {
    fn new(bit_flag: bool) -> Self;
    fn update(&mut self, bytes: &[u8]);

    fn get_value(&self) -> f64;
}
