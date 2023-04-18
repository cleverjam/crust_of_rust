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
         * This is an interesting one, self.remainder is not a mutable reference,
         * but because self is a mutable reference, we can modify the value remainder
         * points to!
         *           '&mut &'a str' from a 'Option<&'a str>'
         */
        if let Some(ref mut remainder) = self.remainder {
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let next = &remainder[..next_delim];
                *remainder = &remainder[next_delim + self.delimiter.len()..];
                Some(next)
            } else {
                self.remainder.take()
            }
        } else {
            None
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
