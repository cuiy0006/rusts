struct SentenceIter<'a> {
    s: &'a mut &'a str,
    delimiter: char,
}

impl<'a> SentenceIter<'a> {
    pub fn new(s: &'a mut &'a str, delimiter: char) -> Self {
        Self {s, delimiter}
    }
}

impl<'a> Iterator for SentenceIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.s.find(self.delimiter) {
            Some(idx) => {
                let ret: &str = &self.s[..idx+self.delimiter.len_utf8()];
                let suffix = &self.s[idx+self.delimiter.len_utf8()..];
                *self.s = suffix;
                Some(ret.trim())
            },
            None => {
                let s = self.s[..].trim();
                *self.s = "";
                if s.len() == 0 {
                    return None;
                } else {
                    Some(s)
                }
            },
        }
    }
}

#[test]
fn iterator_should_work() {
    let mut s = "This is the 1st sentence. This is the second sentence.";
    let mut iter = SentenceIter::new(&mut s, '.');
    assert_eq!(iter.next(), Some("This is the 1st sentence."));
    assert_eq!(iter.next(), Some("This is the second sentence."));
    assert_eq!(iter.next(), None);

    let mut s1 = "a。 b。 c";
    let sentences: Vec<_> = SentenceIter::new(&mut s1, '。').collect();
    println!("setences: {:?}", sentences);
}