use std::rc::Rc;
use std::ops::Range;
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

    pub fn char_slice(&self, r: Range<usize>) -> Self {
        self.char_substr(r.start, r.end - r.start)
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

    /*
    fn line_iter(&self) -> RopeIter<LineIter> {
        RopeIter{
            stack:    vec![self],
            curriter: None,
        }
    }

    fn line_at(&self) -> Rope;
    */

    /*
    fn line_slice(&self, idx: usize, n: usize) -> Self;
    */
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
        for (ch, e) in r1.char_slice(0..r1.len()-1).char_iter().zip("aaabbbccc".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.char_slice(4..4).char_iter().zip("".chars()) {
            assert_eq!(ch, e);
        }
        for (ch, e) in r1.char_slice(3..9).char_iter().zip("bbbccc".chars()) {
            assert_eq!(ch, e);
        }
    }
}
