use std::ops::RangeInclusive;

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
    todo!()
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



#[cfg(test)]
mod tests {
    use super::PasswordPolicy;

    #[test]
    fn test_is_valid() {
        let pp = PasswordPolicy {
            range: 1..=3,
            byte: b'a',
        };
        assert_eq!(pp.is_valid("zeus"), false, "no 'a's");
        assert_eq!(pp.is_valid("hades"), false, "single 'a's");
        assert_eq!(pp.is_valid("banana"), false, "three 'a's");
        assert_eq!(pp.is_valid("aaaah"), false, "too many 'a's");

    }
}