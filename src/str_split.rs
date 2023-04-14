#[derive(Debug, PartialEq)]
pub struct StrSplit<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        StrSplit {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let next = &self.remainder[..next_delim];
            self.remainder = &self.remainder[next_delim + self.delimiter.len()..];
            Some(next)
        } else if self.remainder.is_empty() {
            // no more string to search through
            None
        } else {
            // delim not found, entire remainder is the last thing found
            let result = self.remainder;
            self.remainder = "";
            Some(result)
        }
    }
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ").into_iter();
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()))
}
