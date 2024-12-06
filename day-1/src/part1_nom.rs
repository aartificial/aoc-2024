use miette::miette;
use nom::{
    character::complete::{self, line_ending, space1},
    combinator::opt,
    multi::fold_many1,
    sequence::{separated_pair, terminated},
    IResult,
};

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, (mut a, mut b)) = parse(input).map_err(|e| miette!("parse error: {}", e))?;

    a.sort_unstable();
    b.sort_unstable();

    let result = a
        .iter()
        .zip(b.iter())
        .fold(0, |acc, (a, b)| acc + a.abs_diff(*b));

    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<u32>)> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, space1, complete::u32),
            opt(line_ending),
        ),
        || (Vec::new(), Vec::new()),
        |mut acc: (Vec<u32>, Vec<u32>), (a, b)| {
            acc.0.push(a);
            acc.1.push(b);
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
