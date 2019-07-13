use std::rc::Rc;
use std::cmp::min;
use std::ops::{RangeBounds, Bound};
use crate::newlines::count_newlines;

#[derive(Clone)]
pub struct RcString {
    base: Rc<String>,
    off:  usize,
    len:  usize,
}

impl RcString {
    pub fn str<'a>(&'a self) -> &'a str {
        &self.base[self.off..self.off+self.len]
    }

    pub fn substr(&self, off: usize, len: usize) -> Self {
        RcString {
            base: self.base.clone(),
            off: self.off + min(self.len, off),
            len: if self.len >= off {
                min(len, self.len - off)
            } else {
                0
            },
        }
    }

    pub fn slice(&self, r: impl RangeBounds<usize>) -> Self {
        let start = match r.start_bound() {
            Bound::Included(b) => *b,
            Bound::Excluded(b) => b+1,
            Bound::Unbounded => 0,
        };
        let len = match r.end_bound() {
            Bound::Included(b) => (b - start) + 1,
            Bound::Excluded(b) => b - start,
            Bound::Unbounded => self.len - start,
        };
        self.substr(start, len)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn lenlines(&self) -> usize {
        count_newlines(&self.base[self.off..(self.off+self.len)])
    }
}

impl From<String> for RcString {
    fn from(s: String) -> Self {
        let len = s.len();
        RcString {
            base: Rc::new(s),
            off:  0,
            len:  len,
        }
    }
}

impl From<&str> for RcString {
    //TODO: RcString could have a special case for string references
    fn from(s: &str) -> Self {
        let len = s.len();
        RcString {
            base: Rc::new(String::from(s)),
            off:  0,
            len:  len,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rcstring::RcString;

    #[test]
    fn test_new() {
        let rs = RcString::from("abc");
        assert_eq!(rs.str(), "abc");
    }

    #[test]
    fn test_substr() {
        let rs = RcString::from("abcdefg");
        assert_eq!(rs.str(), "abcdefg");
        let rs2 = rs.substr(2, 4);
        assert_eq!(rs2.str(), "cdef");
        let rs3 = rs2.substr(2, 1);
        assert_eq!(rs3.str(), "e");
        assert_eq!(rs3.substr(1,0).str(), "");
        assert_eq!(rs3.substr(0,10).str(), "e");
        assert_eq!(rs3.substr(15,3).str(), "");
    }

    #[test]
    fn test_slice() {
        let rs = RcString::from("abcdefg");
        assert_eq!(rs.str(), "abcdefg");
        let rs2 = rs.slice(2..);
        assert_eq!(rs2.str(), "cdefg");
        let rs3 = rs2.slice(2..3);
        assert_eq!(rs3.str(), "e");
        assert_eq!(rs3.slice(0..10).str(), "e");
        assert_eq!(rs3.slice(15..18).str(), "");
    }

    #[test]
    fn test_len() {
        let rs = RcString::from("abcdefg");
        assert_eq!(rs.len(), 7);
        let rs2 = rs.substr(2, 4);
        assert_eq!(rs2.len(), 4);
        let rs3 = rs2.substr(2, 1);
        assert_eq!(rs3.len(), 1);
        assert_eq!(rs3.substr(1,0).len(), 0);
        assert_eq!(rs3.substr(0,10).len(), 1);
        assert_eq!(rs3.substr(15,3).len(), 0);
    }

    #[test]
    fn test_lenlines() {
        let rs = RcString::from("\nab\nc\ndefg\n");
        assert_eq!(rs.lenlines(), 4);
        let rs2 = rs.substr(2, 4);
        assert_eq!(rs2.lenlines(), 2);
        let rs3 = rs2.substr(2, 1);
        assert_eq!(rs3.lenlines(), 0);
        assert_eq!(rs3.substr(1,0).lenlines(), 0);
    }
}
