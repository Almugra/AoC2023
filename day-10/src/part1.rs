use crate::custom_error::AocError;

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<usize, AocError> {
    let (mut s_y, mut s_x) = (0, 0);
    let arr = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .inspect(|(x, char)| {
                    if char == &'S' {
                        (s_y, s_x) = (y, *x)
                    }
                })
                .map(|(_, x)| x)
                .collect()
        })
        .collect::<Vec<Vec<char>>>();

    Ok(calc_loop_len(
        &arr,
        1,
        s_x,
        s_y + 1,
        s_x,
        s_y,
    ))
}

fn calc_loop_len(
    matrix: &[Vec<char>],
    mut curr: usize,
    mut x: usize,
    mut y: usize,
    mut prev_x: usize,
    mut prev_y: usize,
) -> usize {
    while matrix[y][x] != 'S' {
        let (s_x, s_y) = (prev_x, prev_y);
        prev_x = x;
        prev_y = y;
        match matrix[y][x] {
            '|' if s_y < y => add_op(&mut x, &mut y, 0, 1),
            '|' => add_op(&mut x, &mut y, 0, -1),
            '-' if s_x < x => add_op(&mut x, &mut y, 1, 0),
            '-' => add_op(&mut x, &mut y, -1, 0),
            'L' if s_y < y => add_op(&mut x, &mut y, 1, 0),
            'L' => add_op(&mut x, &mut y, 0, -1),
            'J' if s_x < x => add_op(&mut x, &mut y, 0, -1),
            'J' => add_op(&mut x, &mut y, -1, 0),
            '7' if s_y > y => add_op(&mut x, &mut y, -1, 0),
            '7' => add_op(&mut x, &mut y, 0, 1),
            'F' if s_x > x => add_op(&mut x, &mut y, 0, 1),
            'F' => add_op(&mut x, &mut y, 1, 0),
            _ => unreachable!(),
        }

        curr += 1;
    }

    curr / 2
}

fn add_op(
    x: &mut usize,
    y: &mut usize,
    dx: isize,
    dy: isize,
) {
    *x = x.wrapping_add_signed(dx);
    *y = y.wrapping_add_signed(dy);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4, process(input)?);
        Ok(())
    }
}
