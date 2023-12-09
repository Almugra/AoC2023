use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {
    Ok(input
        .lines()
        .map(|line| {
            let chars = line
                .chars()
                .filter(|char| char.is_ascii_digit())
                .collect::<Vec<char>>();
            format!(
                "{}{}",
                chars.first().unwrap(),
                chars.last().unwrap()
            )
            .parse::<usize>()
            .unwrap()
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, process(input)?);
        Ok(())
    }
}
