pub trait DataProvider {
    fn len(&self) -> usize;
    fn get_line(&mut self, line: i32) -> Option<String>;
}
