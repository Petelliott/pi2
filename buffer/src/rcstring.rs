use std::rc::Rc;

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
            off: self.off + off,
            len: len,
        }
    }

    pub fn slice(&self, lo: usize, hi: usize) -> Self {
        self.substr(lo, hi-lo)
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
        assert_eq!(rs2.substr(2, 1).str(), "e");
    }

    #[test]
    fn test_slice() {
        let rs = RcString::new(String::from("abcdefg"));
        assert_eq!(rs.str(), "abcdefg");
        let rs2 = rs.slice(2, 6);
        assert_eq!(rs2.str(), "cdef");
        assert_eq!(rs2.slice(2, 3).str(), "e");
    }
}
