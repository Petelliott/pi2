use std::iter::Iterator;
use std::string::String;

pub trait StorageView: Clone {
    fn str_iter(&self) -> Iterator<Item = String>;
    fn char_iter(&self) -> Iterator<Item = char>;
    fn line_iter(&self) -> Iterator<Item = &Self>;

    fn line_at(&self) -> Self;

    fn char_slice(&self, idx: usize, n: usize) -> Self;
    fn line_slice(&self, idx: usize, n: usize) -> Self;
}

pub trait ImmStore: Clone + StorageView {
    fn insert(&self, idx: usize, string: String) -> Self;
    fn delete(&self, idx: usize, n: usize) -> Self;
}
