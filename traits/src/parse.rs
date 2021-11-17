use regex::Regex;
use std::str::FromStr;

pub trait Parse {
    type Error;
    fn parse(s: &str) -> Result<Self, Self::Error>
    where 
        Self: Sized;
}

impl<T> Parse for T 
    where
        T: FromStr + Default,
{
    type Error = String;
    fn parse(s: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
        if let Some(captures) = re.captures(s) {
            captures
                .get(0)
                .map_or(Err("failed to capture".to_string()), 
                        |s| s.as_str().parse::<Self>().map_err(|_err| "failed to parse captured string".to_string())
                    )
        } else {
            Err("failed to parse string".to_string())
        }
    }
}

#[test]
fn parse_should_word() {
    assert_eq!(u32::parse("123abc"), Ok(123));
    assert_eq!(u32::parse("1234.456abc"), Err("failed to parse captured string".to_string()));

    assert_eq!(f64::parse("123.456abc"), Ok(123.456));
    assert_eq!(f64::parse("abcd"), Err("failed to parse string".to_string()));
}