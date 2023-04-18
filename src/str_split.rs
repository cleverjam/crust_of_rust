#[derive(Debug, PartialEq)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        StrSplit {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;

    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;
        if let Some((delimiter_start, delimiter_end)) = self.delimiter.find_next(remainder) {
            let next = &remainder[..delimiter_start];
            *remainder = &remainder[delimiter_end..];
            Some(next)
        } else {
            self.remainder.take()
        }
    }
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
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
