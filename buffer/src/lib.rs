//! # the pi buffer
//! this crate has a the buffer of the pi text editor and associated types
mod newlines;
pub mod rcstring;
pub mod rope;

use std::vec::Vec;
use std::option::Option;
use std::fs;
use std::io;

use crate::rope::Rope;

/// an editor buffer with a cursor and undo/redo functionality
pub struct Buffer {
    filename: Option<String>,
    undo_stack: Vec<Rope>,
    line: usize,
    offset: usize,
    undooff: usize,
}

impl Buffer {
    pub fn new(filename: Option<String>) -> io::Result<Self> {
        let mut buff = Buffer {
            filename: filename,
            undo_stack: Vec::new(),
            line: 0,
            offset: 0,
            undooff: 0,
        };

        if let Some(name) = buff.filename.clone() {
            buff.load_file(name)?
        }

        Ok(buff)
    }

    pub fn load_file(& mut self, name: String) -> io::Result<()> {
        self.undo_stack = vec![Rope::from(fs::read_to_string(&name)?)];
        self.undooff = 0;
        self.filename = Some(name);
        Ok(())
    }

    pub fn reload(& mut self) -> io::Result<()> {
        match self.filename.clone() {
            Some(name) => {
                self.load_file(name)?;
                Ok(())
            },
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "buffer has no associated file")),
        }
    }
}
