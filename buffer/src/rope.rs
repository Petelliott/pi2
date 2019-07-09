use std::rc::Rc;
use std::vec::Vec;
use std::option::Option;
use std::str::Chars;
//use crate::bufferstorage::{ImmStore, StorageView};

#[derive(Clone)]
pub struct Node {
    leftn:   usize,
    leftnnl: usize,
    left:    Rope,
    right:   Rope,
}

#[derive(Clone)]
pub enum Rope {
    Node(Rc<Node>),
    Leaf(Rc<String>),
}

impl Rope {
    fn str_iter(&self) -> RopeIter<StrIter> {
        RopeIter{
            stack:    vec![self],
            curriter: None,
        }
    }

    fn char_iter(&self) -> RopeIter<CharIter> {
        RopeIter{
            stack:    vec![self],
            curriter: None,
        }
    }
}

pub trait LeafIter<'a>: Iterator {
    fn from(s: &'a String) -> Self;
}

struct StrIter<'a> {
    s: Option<&'a String>,
}

impl<'a> LeafIter<'a> for StrIter<'a> {
    fn from(s: &'a String) -> Self {
        StrIter{ s: Some(s) }
    }
}

impl<'a> Iterator for StrIter<'a> {
    type Item = &'a String;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.s;
        self.s = None;
        r
    }
}

struct CharIter<'a> {
    it: Chars<'a>,
}

impl<'a> LeafIter<'a> for CharIter<'a> {
    fn from(s: &'a String) -> Self {
        CharIter{ it: s.chars()}
    }
}

impl<'a> Iterator for CharIter<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}

pub struct RopeIter<'a, T: LeafIter<'a>> {
    stack:    Vec<&'a Rope>,
    curriter: Option<T>,
}

impl<'a, T: LeafIter<'a>> Iterator for RopeIter<'a, T> {
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match match &mut self.curriter {
            None => None,
            Some(i) => i.next()
        }
        {
            Some(i) => Some(i),
            None => {
                while let Some(cur) = self.stack.pop() {
                    match cur {
                        Rope::Node(n) => {
                            self.stack.push(&n.right);
                            self.stack.push(&n.left);
                        },
                        Rope::Leaf(l) => {
                            self.curriter = Some(T::from(l));
                            return self.next();
                        },
                    }
                }
                return None
            }
        }
    }
}
