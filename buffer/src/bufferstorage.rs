use std::ops::Index;
use std::iter::Iterator;
use std::string::String;

pub trait BufferStorage: Clone + Index<usize, Output=char> + Iterator<Item=char> {
    fn insert(&self, idx: usize, string: String) -> Self;
    fn piece_iter(&self) -> Iterator<Item = String>;
    fn delete(&self, idx: usize, n: usize) -> Self;
}
