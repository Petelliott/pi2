//pub mod bufferstorage;
pub mod rope;

//pub use bufferstorage::*;
pub use rope::*;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
