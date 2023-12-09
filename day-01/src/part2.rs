use crate::custom_error::AocError;

const DIGITS: [&str; 18] = [
    "one", "1", "two", "2", "three", "3", "four", "4",
    "five", "5", "six", "6", "seven", "7", "eight", "8",
    "nine", "9",
];

fn convert_number_string(str: &str) -> &str {
    match str {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => str,
    }
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {
    Ok(input
        .lines()
        .map(|line| {
            let mut a = DIGITS
                .iter()
                .flat_map(|digit_str| {
                    line.match_indices(digit_str)
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<(usize, &str)>>();
            a.sort_by(|a, b| a.0.cmp(&b.0));
            format!(
                "{}{}",
                convert_number_string(a.first().unwrap().1),
                convert_number_string(a.last().unwrap().1)
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input)?);
        Ok(())
    }
}
