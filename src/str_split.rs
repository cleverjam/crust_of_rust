#[derive(Debug, PartialEq)]
pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        /*
         * the ? returns the Some(T) if present,
         * otherwise ends current fn and returns None.
         * Note:
         * let ref mut remainder = self.remainder?;
         * Does not move remainder out, because &str implements Copy trait,
         * therefore it just copies it, meaning the a new remainder value is created
         * and modifying does not affect self.remainder.
         */
        let remainder = self.remainder.as_mut()?;
        if let Some(next_delim) = remainder.find(self.delimiter) {
            let next = &remainder[..next_delim];
            *remainder = &remainder[next_delim + self.delimiter.len()..];
            Some(next)
        } else {
            self.remainder.take()
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ").into_iter();
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()))
}
#[test]
fn tail() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ").into_iter();
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()))
}
