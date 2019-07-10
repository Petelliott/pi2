use std::rc::Rc;
use std::cmp::min;

#[derive(Clone)]
pub struct RcString {
    base: Rc<String>,
    off:  usize,
    len:  usize,
}

impl RcString {
    pub fn new(s: String) -> Self {
        let len = s.len();
        RcString {
            base: Rc::new(s),
            off:  0,
            len:  len,
        }
    }

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

    pub fn slice(&self, lo: usize, hi: usize) -> Self {
        self.substr(lo, hi-lo)
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use crate::rcstring::RcString;

    #[test]
    fn test_new() {
        let rs = RcString::new(String::from("abc"));
        assert_eq!(rs.str(), "abc");
    }

    #[test]
    fn test_substr() {
        let rs = RcString::new(String::from("abcdefg"));
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
        let rs = RcString::new(String::from("abcdefg"));
        assert_eq!(rs.str(), "abcdefg");
        let rs2 = rs.slice(2, 6);
        assert_eq!(rs2.str(), "cdef");
        let rs3 = rs2.slice(2, 3);
        assert_eq!(rs3.str(), "e");
        assert_eq!(rs3.slice(0,10).str(), "e");
        assert_eq!(rs3.substr(15,18).str(), "");
    }

    #[test]
    fn test_len() {
        let rs = RcString::new(String::from("abcdefg"));
        assert_eq!(rs.len(), 7);
        let rs2 = rs.substr(2, 4);
        assert_eq!(rs2.len(), 4);
        let rs3 = rs2.substr(2, 1);
        assert_eq!(rs3.len(), 1);
        assert_eq!(rs3.substr(1,0).len(), 0);
        assert_eq!(rs3.substr(0,10).len(), 1);
        assert_eq!(rs3.substr(15,3).len(), 0);
    }
}
