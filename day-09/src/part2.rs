use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<isize, AocError> {
    Ok(input
        .lines()
        .flat_map(|line| {
            let mut last_nums: Vec<isize> = vec![];
            let mut num_line = line
                .split_whitespace()
                .rev()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>();
            last_nums.push(*num_line.last().unwrap());
            loop {
                num_line = num_line
                    .windows(2)
                    .map(|window| window[1] - window[0])
                    .collect::<Vec<_>>();

                if num_line.iter().all(|num| num == &0) {
                    break;
                }

                last_nums.push(*num_line.last().unwrap());
            }
            last_nums
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "10  13  16  21  30  45";
        assert_eq!(5, process(input)?);
        Ok(())
    }
}
