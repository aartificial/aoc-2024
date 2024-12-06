use itertools::Itertools;
use miette::IntoDiagnostic;

const DELIMITER: &str = "   ";

#[tracing::instrument(skip(input))]
pub fn process(input: &str) -> miette::Result<String> {
    Ok(input
        .lines()
        .map(|line| line.split(DELIMITER).map(str::parse).try_collect())
        .try_collect()
        .map(|parsed_lines: Vec<Vec<i32>>| {
            parsed_lines.into_iter().fold(
                (Vec::new(), Vec::new()),
                |(mut acc_a, mut acc_b), line| {
                    if let [a, b] = line.as_slice() {
                        acc_a.push(*a);
                        acc_b.push(*b);
                    }
                    (acc_a, acc_b)
                },
            )
        })
        .map(|(mut vec_a, mut vec_b)| {
            vec_a.sort_unstable();
            vec_b.sort_unstable();
            (vec_a, vec_b)
        })
        .map(|(vec_a, vec_b)| {
            vec_a
                .into_iter()
                .zip(vec_b.into_iter())
                .map(|(a, b)| a.abs_diff(b))
                .sum::<u32>()
        })
        .into_diagnostic()?
        .to_string())
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
