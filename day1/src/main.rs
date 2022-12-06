use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    // let s = std::fs::read_to_string("./src/input.txt")?;
    let pair = find_a_pair(include_str!("input.txt").lines()
                                   .map(str::parse::<i64>)
                                   .collect::<Result<Vec<_>, _>>()?,
    );
    // dbg!(&s[..20]);

    dbg!(pair.into_iter().map(|(a, b, c)| a*b*c).collect::<Vec<i64>>());

    // println!("Hello, world!");

    Ok(())
}


fn find_a_pair(input_vec: Vec<i64>) -> Option<(i64, i64, i64)> {
    input_vec.into_iter()
        .tuple_combinations()
        .find(|(a, b, c)| a + b +c == 2020)
}