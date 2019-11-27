use std::iter::Peekable;
use Input::*;

enum Input {
    Uncompressed,
    RepeatSpec,
    Compressed,
}

fn decomp_len(contents: &str, recursive: bool) -> Option<usize> {
    decomp_len_iter(
        contents
            .chars()
            .filter(|&ch| !ch.is_whitespace())
            .peekable(),
        recursive,
    )
}

fn decomp_len_iter<I>(mut chars: Peekable<I>, recursive: bool) -> Option<usize>
where
    I: Iterator<Item = char>,
{
    let mut len = 0;
    let mut rep_len = 0;
    let mut rep_times = 0;
    let mut state = Uncompressed;
    while chars.peek().is_some() {
        match state {
            Uncompressed => {
                len += (&mut chars).take_while(|&ch| ch != '(').count();
                state = RepeatSpec;
            }
            RepeatSpec => {
                rep_len = (&mut chars)
                    .take_while(|&ch| ch != 'x')
                    .collect::<String>()
                    .parse()
                    .ok()?;
                rep_times = (&mut chars)
                    .take_while(|&ch| ch != ')')
                    .collect::<String>()
                    .parse()
                    .ok()?;
                state = Compressed;
            }
            Compressed => {
                let data_len = if recursive {
                    let data = (&mut chars).take(rep_len).collect::<Vec<_>>();
                    if data.len() < rep_len {
                        // Missing compressed data
                        return None;
                    }
                    decomp_len_iter(data.into_iter().peekable(), recursive)?
                } else {
                    if (&mut chars).take(rep_len).count() < rep_len {
                        // Missing compressed data
                        return None;
                    }
                    rep_len
                };
                len += rep_times * data_len;
                state = Uncompressed;
            }
        }
    }
    Some(len)
}

pub fn part1(contents: &str) -> Option<usize> {
    decomp_len(contents, false)
}

pub fn part2(contents: &str) -> Option<usize> {
    decomp_len(contents, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1("ADVENT"), Some(6));
        assert_eq!(part1("A(1x5)BC"), Some(7));
        assert_eq!(part1("(3x3)XYZ"), Some(9));
        assert_eq!(part1("A(2x2)BCD(2x2)EFG"), Some(11));
        assert_eq!(part1("(6x1)(1x3)A"), Some(6));
        assert_eq!(part1("X(8x2)(3x3)ABCY"), Some(18));
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2("ADVENT"), Some(6));
        assert_eq!(part2("(3x3)XYZ"), Some(9));
        assert_eq!(part2("X(8x2)(3x3)ABCY"), Some(20));
        assert_eq!(part2("(27x12)(20x12)(13x14)(7x10)(1x12)A"), Some(241920));
        assert_eq!(
            part2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN"),
            Some(445)
        );
    }
}
