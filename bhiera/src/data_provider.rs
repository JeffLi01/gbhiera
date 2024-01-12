pub trait DataProvider {
    fn len(&self) -> usize;
    fn get(&self, offset: usize, count: usize) -> Vec<u8>;
}
