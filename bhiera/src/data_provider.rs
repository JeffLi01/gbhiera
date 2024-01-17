pub trait DataProvider {
    fn len(&self) -> usize;
    fn get(&self, offset: usize, count: usize) -> Option<&[u8]>;
}
