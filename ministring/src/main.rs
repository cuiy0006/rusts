use std::ops::Deref;
use std::{str, fmt};


const MINI_STRING_MAX_LEN: usize = 30;

#[derive(Debug)]
enum MyString {
    Inline(MiniString),
    Standard(String),
}

struct MiniString {
    len: u8,
    data: [u8; MINI_STRING_MAX_LEN],
}

impl MiniString {
    pub fn new(v: impl AsRef<str>) -> MiniString {
        let bytes = v.as_ref().as_bytes();
        let len = bytes.len();

        let mut data = [0u8; MINI_STRING_MAX_LEN];
        data[0..len].copy_from_slice(bytes);
        MiniString {
            len: len as u8,
            data: data,
        }
    }
}

impl Deref for MiniString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        str::from_utf8(&self.data[..self.len as usize]).unwrap()
    }
}

impl fmt::Debug for MiniString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{}", self.deref())
    }
}

impl MyString {
    pub fn push_str(&mut self, sstr: &str) {

        match self {
            Self::Standard(s) => {
                s.push_str(sstr);
            },
            Self::Inline(s) => {
                let len = sstr.bytes().len();
                if s.len() + len > MINI_STRING_MAX_LEN {
                    let mut new_s = s.to_owned();
                    new_s.push_str(sstr);
                    *self = Self::Standard(new_s);
                } else {
                    let start_idx = s.len as usize;
                    s.data[start_idx..start_idx+len].copy_from_slice(sstr.as_bytes());
                    s.len = (start_idx + len) as u8;
                }
            }
        }
    }
}

impl Deref for MyString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Inline(s) => {
                s.deref()
            },
            Self::Standard(s) => {
                &s[..]
            }
        }
    }
}

impl From<&str> for MyString {
    fn from(s: &str) -> Self {
        if s.len() > MINI_STRING_MAX_LEN {
            Self::Standard(s.to_owned())
        } else {
            Self::Inline(MiniString::new(s))
        }
    }
}

impl From<String> for MyString {
    fn from(s: String) -> Self {
        if s.len() > MINI_STRING_MAX_LEN {
            Self::Standard(s)
        } else {
            Self::Inline(MiniString::new(&s[..]))
        }
    }
}

impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deref())
    }
}

fn main() {
    let len1 = std::mem::size_of::<MyString>();
    let len2 = std::mem::size_of::<MiniString>();
    println!("Len: MyString {}, MiniString: {}", len1, len2);

    let s1: MyString = "hello world".into();
    let s2: MyString = "这是一个长度超过三十个字节的很长的字符串".into();

    println!("s1: {:?}, s2: {:?}", s1, s2);
    println!(
        "s1: {}({} bytes, {} chars), s2: {}({} bytes, {} chars)",
        s1,
        s1.len(),
        s1.chars().count(),
        s2,
        s2.len(),
        s2.chars().count()
    );

    let s3 = "12345".to_owned();
    let mut s4: MyString = s3.into();

    println!(
        "s4: {:?}({} bytes, {} chars)",
        s4,
        s4.len(),
        s4.chars().count()
    );

    s4.push_str("67890");
    println!(
        "s4: {:?}({} bytes, {} chars)",
        s4,
        s4.len(),
        s4.chars().count()
    );

    s4.push_str("加到30个字节之上去观察内部的变化");
    println!(
        "s4: {:?}({} bytes, {} chars)",
        s4,
        s4.len(),
        s4.chars().count()
    );
}
