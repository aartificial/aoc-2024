use miette::miette;
use nom::{
    character::complete::{self, line_ending, space1},
    combinator::opt,
    multi::fold_many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (a, b)) = parse(input).map_err(|e| miette!("parse error: {}", e))?;

    let result = a
        .iter()
        .fold(0, |acc, n| acc + b.get(n).map(|v| n * v).unwrap_or(0));

    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, HashMap<u32, u32>)> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, space1, complete::u32),
            opt(line_ending),
        ),
        || (Vec::new(), HashMap::new()),
        |mut acc: (Vec<u32>, HashMap<u32, u32>), (a, b)| {
            acc.0.push(a);
            *acc.1.entry(b).or_insert(0) += 1;
            acc
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(process(input)?, "11");
        Ok(())
    }
}
