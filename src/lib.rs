use smallvec::SmallVec;
use std::fmt;
include!(concat!(env!("OUT_DIR"), "/emojis.rs"));

#[derive(Debug, Clone, Copy, PartialEq)]
struct Range(usize, usize);

#[derive(Clone, PartialEq)]
pub struct EmojiStr<'a>(SmallVec<[Range; 10]>, &'a str, SmallVec<[&'static str; 10]>);

impl<'a> fmt::Display for EmojiStr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut c = 0;
        for (i, range) in self.0.iter().enumerate() {
            f.write_str(&self.1[c..range.0])?;
            c = range.1;
            f.write_str(&self.2[i])?;
        }
        if c < self.1.len() {
            f.write_str(&self.1[c..])?;
        }

        Ok(())
    }
}

pub fn lookup(alias: &str) -> Option<&'static str> {
    EMOJIS.get(alias).map(|m| m.1)
}

pub fn lookup_emoji(emoji: &str) -> Option<&'static str> {
    for (k, v) in EMOJIS.entries() {
        if v.1 == emoji {
            return Some(k);
        }
    }
    None
}

pub fn replace<'a>(text: &'a str) -> EmojiStr<'a> {
    let mut ranges = SmallVec::default();
    let mut subs = SmallVec::default();

    let mut lower = 0;
    let mut in_code = false;
    for (i, to) in text.chars().enumerate() {
        match to {
            ':' if in_code == true => {
                let sub = lookup(&text[lower..i + 1]);
                if sub.is_some() {
                    subs.push(sub.unwrap());
                    ranges.push(Range(lower, i + 1));
                }
                in_code = false;
            }
            ':' if in_code == false => {
                lower = i;
                in_code = true;
            }
            ' ' | '\n' if in_code == true => {
                in_code = false;
            }
            _ => {}
        }
    }

    EmojiStr(ranges, text, subs)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_replace() {
        let re = replace("Hello world :smiley: :gun: and so on");
        assert_eq!("Hello world 😃 🔫 and so on", re.to_string().as_str());
        //println!("{}", re);
    }


    #[test]
    fn test_lookup() {
        let re = lookup(":smiley:");
        assert_eq!(Some("😃"), re);
    }

    #[test]
    fn test_lookup_emoji() {
        let re = lookup_emoji("😃");
        assert_eq!(Some(":smiley:"), re);
    }
}
