use peg::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Year(
    u64
);

#[derive(Clone, Copy, PartialEq, Debug)]
enum Height {
    Cm(u64),
    In(u64),
    Unspecified(u64),
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Color<'a> (
    &'a str
);

#[derive(Clone, Copy, PartialEq, Debug)]
struct ID<'a> (
    &'a str
);

#[derive(PartialEq, Debug)]
struct Passport<'a> {
    birth_year: Year,
    issue_year: Year,
    expiration_year: Year,
    height: Height,
    hair_color: Color<'a>,
    eye_color: Color<'a>,
    passport_id: ID<'a>,
    country_id: Option<ID<'a>>,
}

#[derive(PartialEq, Debug, Default)]
struct PassportBuilder<'a> {
    birth_year: Option<Year>,
    issue_year: Option<Year>,
    expiration_year: Option<Year>,
    height: Option<Height>,
    hair_color: Option<Color<'a>>,
    eye_color: Option<Color<'a>>,
    passport_id: Option<ID<'a>>,
    country_id: Option<ID<'a>>,
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("missing field: {0}")]
    MissingField(&'static str),
}

impl<'a> PassportBuilder<'a> {
    fn build(self) -> Result<Passport<'a>, Error> {
        // Ok(Passport {
        //     birth_year: self.birth_year.ok_or(Error::MissingField("birth_year"))?,
        //     issue_year: self.issue_year.ok_or(Error::MissingField("issue year"))?,
        //     expiration_year: self
        //         .expiration_year
        //         .ok_or(Error::MissingField("expiration_year"))?,
        //     height: self.height.ok_or(Error::MissingField("height"))?,
        //     hair_color: self.hair_color.ok_or(Error::MissingField("hair color"))?,
        //     eye_color: self.eye_color.ok_or(Error::MissingField("eye_color"))?,
        //     passport_id: self.passport_id.ok_or(Error::MissingField("passport id"))?,
        //     country_id: self.country_id,
        // })

        /// using macros to implement the above logic
        macro_rules! build {
            (
                required => {
                    $($req: ident),* $(,)*
                }$(,)*
                optional => {
                    $($opt: ident),* $(,)*
                }$(,)*
            ) => {
                Ok(Passport {
                    $($req: self.$req.ok_or(Error::MissingField(stringify!($req)))?),*,
                    $($opt: self.$opt),*
                })
            }
        }

        build! {
            required => {
                birth_year,
                issue_year,
                expiration_year,
                height,
                hair_color,
                eye_color,
                passport_id,
            },
            optional => {
                country_id
            },
        }
    }

    /// A parser that parses only one record
    fn parse(input: &'a str) -> Self {
        let mut b: Self = Default::default();

        peg::parser! {
            grammar parser() for str {
                // ![_] matches the EOF .i.e. end of file.
                pub(crate) rule root(b: &mut PassportBuilder<'input>)
                    = (field(b) separator()*)* ![_]

                rule separator() = ['\n' | ' ']

                rule field(b: &mut PassportBuilder<'input>)
                    = byr(b) / iyr(b) / eyr(b)      // years
                    / hgt(b)                        // height
                    / hcl(b) / ecl(b)               // colors
                    / pid(b) / cid(b)               // IDs

                rule byr(b: &mut PassportBuilder<'input>)
                    = "byr:" year:year() {
                    b.birth_year = Some(year)
                }

                rule iyr(b: &mut PassportBuilder<'input>)
                    = "iyr:" year:year() { b.issue_year = Some(year) }

                rule eyr(b: &mut PassportBuilder<'input>)
                    = "eyr:" year:year() { b.expiration_year = Some(year) }

                rule hgt(b: &mut PassportBuilder<'input>)
                    = "hgt:" height:length() { b.height = Some(height) }

                rule pid(b: &mut PassportBuilder<'input>)
                    = "pid:" id:id() { b.passport_id = Some(id) }

                rule cid(b: &mut PassportBuilder<'input>)
                    = "cid:" id:id() { b.country_id = Some(id) }

                rule hcl(b: &mut PassportBuilder<'input>)
                    = "hcl:" color:color() { b.hair_color = Some(color) }

                rule ecl(b: &mut PassportBuilder<'input>)
                    = "ecl:" color:color() { b.eye_color = Some(color) }

                rule year() -> Year
                    = num:num() { Year(num) }

                // [_] matches anything
                rule color() -> Color<'input>
                    = s:$((!separator()[_])*) { Color(s) }

                rule length() -> Height
                    = num:num() "cm" { Height::Cm(num) }
                    / num:num() "in" { Height::In(num) }
                    / num:num() { Height::Unspecified(num) }

                rule num() -> u64
                    = s:$(['0'..='9']+) { s.parse().unwrap() }

                rule id() -> ID<'input>
                    = s:$(['0'..='9' | 'a'..='z' | '#']+) { ID(s) }
            }
        }
        parser::root(input, &mut b).unwrap_or_else(|e| panic!(
            "Could not parse {}: {}", input, e
        ));
        b
    }
}

fn main() {
    let results = include_str!("input.txt")
        .split("\n\n")
        .map(PassportBuilder::parse)
        .map(PassportBuilder::build);

    let num_valid = results.filter(Result::is_ok).count();
    println!("{} passport records are valid.", num_valid);
    // println!("{:#?}", results);
}


#[cfg(test)]
mod tests {
    use super::{
        PassportBuilder, Year, Color, ID, Height};

    #[test]
    fn test_builder() {
        assert!(PassportBuilder {
            ..Default::default()
        }
            .build()
            .is_err());
        assert!(PassportBuilder {
            birth_year: Some(Year(2014)),
            issue_year: Some(Year(2017)),
            expiration_year: Some(Year(2023)),
            height: Some(Height::Cm(195)),
            hair_color: Some(Color("#ffffff")),
            eye_color: Some(Color("#ee7812")),
            passport_id: Some(ID("00023437")),
            country_id: None,
        }
            .build()
            .is_ok());
    }
}