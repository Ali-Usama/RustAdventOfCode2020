use std::ops::RangeInclusive;

#[derive(PartialEq, Debug)]
struct PasswordPolicy {
    byte: u8,
    range: RangeInclusive<usize>,
}

fn main() -> anyhow::Result<()> {
    let count = include_str!("input.txt")
        .lines().map(parse_line)
        .map(Result::unwrap)
        .filter(|(policy, password)| policy.is_valid(password))
        .count();
    println!("{} passwords are valid", count);

    Ok(())
}

fn parse_line(s: &str) -> anyhow::Result<(PasswordPolicy, &str)> {
    let (policy, password) = {
        let mut tokens = s.split(':');
        (
            tokens.next()
                .ok_or(ParseError::Expected("password policy"))?,
            tokens.next()
                .ok_or(ParseError::Expected("password"))?
                .trim(),
        )
    };

    let (range, byte) = {
        let mut tokens = policy.split(' ');
        (
            tokens.next()
                .ok_or(ParseError::Expected("policy range"))?,
            tokens.next()
                .ok_or(ParseError::Expected("policy byte"))?,
        )
    };

    let byte = if byte.as_bytes().len() == 1 {
        byte.as_bytes()[0]
    } else {
        return Err(ParseError::Expected("password policy byte to be exactly 1 byte").into());
    };

    let (min, max) = {
        let mut tokens = range.split('-');
        (
            tokens.next()
                .ok_or(ParseError::Expected("policy range (lower bound)"))?,
            tokens.next()
                .ok_or(ParseError::Expected("policy range (upper bound)"))?
        )
    };

    let range = (min.parse()?)..=(max.parse()?);

    println!("{}: {}, {}", policy, min, max);

    Ok((PasswordPolicy {
        range,
        byte,
    }, password))
}

impl PasswordPolicy {
    fn is_valid(&self, password: &str) -> bool {
        self.range.contains(
            &password
                .as_bytes().iter()
                .copied()
                .filter(|&b| b == self.byte)
                .count(),
        )
    }
}

#[derive(thiserror::Error, Debug)]
enum ParseError {
    #[error("expected {0}")]
    Expected(&'static str),
}


#[cfg(test)]
mod tests {
    use super::{PasswordPolicy, parse_line};

    #[test]
    fn test_is_valid() {
        let pp = PasswordPolicy {
            range: 1..=3,
            byte: b'a',
        };
        assert_eq!(pp.is_valid("zeus"), false, "no 'a's");
        assert_eq!(pp.is_valid("hades"), true, "single 'a's");
        assert_eq!(pp.is_valid("banana"), true, "three 'a's");
        assert_eq!(pp.is_valid("aaaah"), false, "too many 'a's");
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse_line("1-3 a: banana").unwrap(),
            (
                PasswordPolicy {
                    range: 1..=3,
                    byte: b'a',
                }, "banana"
            )
        );

        assert_eq!(
            parse_line("1-3 a").unwrap_err().to_string(),
            "expected password"
        );

        assert_eq!(
            parse_line("1-3 : banana").unwrap_err().to_string(),
            "expected password policy byte to be exactly 1 byte"
        );
    }
}