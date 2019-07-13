use std::rc::Rc;
use std::ops::{RangeBounds, Bound};
use std::vec::Vec;
use std::option::Option;
use std::str::Chars;
use crate::rcstring::RcString;

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
    Leaf(RcString),
}

fn nth_line_idx(s: &str, lnum: usize) -> usize {
    let mut n = lnum;
    let mut r = 0;
    for (i, ch) in s.chars().enumerate() {
        if n == 0 {
            return i;
        }
        if ch == '\n' {
            n -= 1;
        }
        r = i;
    }
    return r + 1;
}

impl Rope {
    pub fn new(s: String) -> Self {
        Rope::Leaf(RcString::from(s))
    }

    pub fn concat(r1: &Self, r2: &Self) -> Self {
        Rope::Node(Rc::new(Node {
            leftn:   r1.len(),
            leftnnl: r1.lenlines(),
            left:    r1.clone(),
            right:   r2.clone(),
        }))
    }

    pub fn len(&self) -> usize {
        match &self {
            Rope::Node(nd) => nd.leftn + nd.right.len(),
            Rope::Leaf(rcs) => rcs.len(),
        }
    }

    pub fn lenlines(&self) -> usize {
        match &self {
            Rope::Node(nd) => nd.leftnnl + nd.right.lenlines(),
            Rope::Leaf(rcs) => rcs.lenlines(),
        }
    }

    pub fn char_substr(&self, idx: usize, n: usize) -> Self {
        match &self {
            Rope::Leaf(rcs) => Rope::Leaf(rcs.substr(idx, n)),
            Rope::Node(nd) =>
                if idx > nd.leftn {
                    nd.right.char_substr(idx-nd.leftn, n)
                } else if idx + n < nd.leftn {
                    nd.left.char_substr(idx, n)
                } else {
                    Rope::concat(
                        &nd.left.char_substr(idx, nd.leftn),
                        &nd.right.char_substr(0, n - (nd.leftn - idx)))
                },
        }
    }

    pub fn char_slice(&self, r: impl RangeBounds<usize>) -> Self {
        let start = match r.start_bound() {
            Bound::Included(b) => *b,
            Bound::Excluded(b) => b+1,
            Bound::Unbounded => 0,
        };
        let len = match r.end_bound() {
            Bound::Included(b) => (b - start) + 1,
            Bound::Excluded(b) => b - start,
            Bound::Unbounded => self.len() - start,
        };
        self.char_substr(start, len)
    }

    pub fn line_start(&self, lnum: usize) -> usize {
        match &self {
            Rope::Leaf(rcs) => nth_line_idx(rcs.str(), lnum),
            Rope::Node(nd) => if lnum <= nd.leftnnl {
                nd.left.line_start(lnum)
            } else {
                nd.right.line_start(lnum - nd.leftnnl) + nd.leftn
            }
        }
    }

    pub fn line_substr(&self, idx: usize, n: usize) -> Self {
        self.line_slice(idx..(idx+n))
    }

    pub fn line_slice(&self, r: impl RangeBounds<usize>) -> Self {
        //TODO: this could be faster by scanning from the start maybe
        let start = match r.start_bound() {
            Bound::Included(b) => self.line_start(*b),
            Bound::Excluded(b) => self.line_start(b+1),
            Bound::Unbounded => 0,
        };

        match r.end_bound() {
            Bound::Included(b) => self.char_slice(start..self.line_start(*b)),
            Bound::Excluded(b) => self.char_slice(start..=self.line_start(*b)),
            Bound::Unbounded => self.char_slice(start..),
        }
    }

    pub fn str_iter(&self) -> RopeIter<StrIter> {
        RopeIter {
            stack:    vec![self],
            curriter: None,
        }
    }

    pub fn char_iter(&self) -> RopeIter<CharIter> {
        RopeIter {
            stack:    vec![self],
            curriter: None,
        }
    }

    pub fn line_iter(&self) -> LineIter {
        LineIter::from(self.clone())
    }
}

pub trait LeafIter<'a>: Iterator {
    fn from(s: &'a RcString) -> Self;
}

pub struct StrIter<'a> {
    s: Option<&'a str>,
}

impl<'a> LeafIter<'a> for StrIter<'a> {
    fn from(s: &'a RcString) -> Self {
        StrIter{ s: Some(s.str()) }
    }
}

impl<'a> Iterator for StrIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.s;
        self.s = None;
        r
    }
}

pub struct CharIter<'a> {
    it: Chars<'a>,
}

impl<'a> LeafIter<'a> for CharIter<'a> {
    fn from(s: &'a RcString) -> Self {
        CharIter{ it: s.str().chars()}
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

pub struct LineIter {
    slice: Rope,
}

impl From<Rope> for LineIter {
    fn from(rope: Rope) -> Self {
        LineIter {
            slice: rope,
        }
    }
}

impl Iterator for LineIter {
    type Item = Rope;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.slice.line_slice(0..1);
        self.slice = self.slice.line_slice(1..);
        if self.slice.len() == 0 {
            None
        } else {
            Some(line)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rope::Rope;

    #[test]
    fn test_concat() {
        let r1 = Rope::concat(
            &Rope::new("aaa".to_string()),
            &Rope::concat(
                &Rope::new("bbb".to_string()),
                &Rope::new("ccc".to_string())));

        for (ch, e) in r1.char_iter().zip("aaabbbccc".chars()) {
            assert_eq!(ch, e);
        }
    }

    #[test]
    fn test_len() {
        let r1 = Rope::concat(
            &Rope::new("aaa".to_string()),
            &Rope::concat(
                &Rope::new("bbb".to_string()),
                &Rope::new("ccc".to_string())));

        assert_eq!(r1.len(), 9);
        assert_eq!(Rope::new("".to_string()).len(), 0);
    }

    #[test]
    fn test_lenlines() {
        let r1 = Rope::concat(
            &Rope::new("aaa\n".to_string()),
            &Rope::concat(
                &Rope::new("b\nb\nb".to_string()),
                &Rope::new("ccc\n".to_string())));

        assert_eq!(r1.lenlines(), 4);
        assert_eq!(Rope::new("".to_string()).lenlines(), 0);
        assert_eq!(Rope::new("\n".to_string()).lenlines(), 1);
    }

    #[test]
    fn test_char_substr() {
        let r1 = Rope::concat(
            &Rope::new("aaa".to_string()),
            &Rope::concat(
                &Rope::new("bbb".to_string()),
                &Rope::new("ccc".to_string())));

        for (ch, e) in r1.char_substr(1, 4).char_iter().zip("aabb".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.char_substr(0, r1.len()).char_iter().zip("aaabbbccc".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.char_substr(4, 0).char_iter().zip("".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.char_substr(3, 6).char_iter().zip("bbbccc".chars()) {
            assert_eq!(ch, e);
        }
    }

    #[test]
    fn test_char_slice() {
        let r1 = Rope::concat(
            &Rope::new("aaa".to_string()),
            &Rope::concat(
                &Rope::new("bbb".to_string()),
                &Rope::new("ccc".to_string())));

        for (ch, e) in r1.char_slice(1..5).char_iter().zip("aabb".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.char_slice(..).char_iter().zip("aaabbbccc".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.char_slice(4..4).char_iter().zip("".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.char_slice(3..9).char_iter().zip("bbbccc".chars()) {
            assert_eq!(ch, e);
        }
    }

    #[test]
    fn test_line_start() {
        let r0 = Rope::new("\nhel".to_string());
        assert_eq!(r0.line_start(0), 0);
        assert_eq!(r0.line_start(1), 1);
        let r1 = Rope::new("hello\nworld\nhi\n".to_string());
        assert_eq!(r1.line_start(0), 0);
        assert_eq!(r1.line_start(1), 6);
        assert_eq!(r1.line_start(2), 12);
        let r2 = Rope::concat(
            &Rope::new("hello\n".to_string()),
            &Rope::new("world\n".to_string()));
        assert_eq!(r2.line_start(0), 0);
        assert_eq!(r2.line_start(1), 6);

        let r3 = Rope::concat(
            &Rope::new("hello".to_string()),
            &Rope::new("\nworld\n".to_string()));
        assert_eq!(r3.line_start(0), 0);
        assert_eq!(r3.line_start(1), 6);
    }

    #[test]
    fn test_line_substr() {
        let r1 = Rope::concat(
            &Rope::new("aa\na".to_string()),
            &Rope::concat(
                &Rope::new("\nbbb\n".to_string()),
                &Rope::new("ccc".to_string())));

        for (ch, e) in r1.line_substr(0, 4).char_iter().zip("aa\na\nbbb\nccc".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.line_substr(1, 2).char_iter().zip("a\nbbb\n".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.line_substr(2, 1).char_iter().zip("bbb\n".chars()) {
            assert_eq!(ch, e);
        }
    }

    #[test]
    fn test_line_slice() {
        let r1 = Rope::concat(
            &Rope::new("aa\na".to_string()),
            &Rope::concat(
                &Rope::new("\nbbb\n".to_string()),
                &Rope::new("ccc".to_string())));

        for (ch, e) in r1.line_slice(..).char_iter().zip("aa\na\nbbb\nccc".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.line_slice(1..3).char_iter().zip("a\nbbb\n".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.line_slice(2..3).char_iter().zip("bbb\n".chars()) {
            assert_eq!(ch, e);
        }
    }

}
