use itertools::Itertools;
use ndarray::{s, Array1};
use nom::{
    character::complete::{digit1, multispace0, space1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn parse(input: &str) -> IResult<&str, Vec<Array1<i64>>> {
    fn number(input: &str) -> IResult<&str, i64> {
        map_res(digit1, str::parse)(input)
    }
    fn line(input: &str) -> IResult<&str, Array1<i64>> {
        map(
            preceded(multispace0, separated_list1(space1, number)),
            |v| Array1::from_vec(v),
        )(input)
    }
    separated_list1(multispace0, line)(input)
}

fn part1(input: &str) -> i64 {
    let (_, arrays) = parse(input).unwrap();

    arrays
        .into_iter()
        .map(|v| {
            let d = &v.slice(s![0..-1]) - &v.slice(s![1..]);
            let increasing = d.iter().all(|&e| e > 0);
            let decreasing = d.iter().all(|&e| e < 0);
            let bounded = d.iter().all(|&e| (1..=3).contains(&e.abs()));

            if (increasing || decreasing) && bounded {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    let (_, arrays) = parse(input).unwrap();

    arrays
        .into_iter()
        .map(|v| {
            let d = &v.slice(s![0..-1]) - &v.slice(s![1..]);
            let increasing = d.iter().map(|&e| (e > 0) as i64).sum();
            0
            // let decreasing = d.iter().all(|&e| e < 0);
            // let bounded = d.iter().all(|&e| (1..=3).contains(&e.abs()));

            // if (increasing || decreasing) && bounded {
            //     1
            // } else {
            //     0
            // }
        })
        .sum()
}

pub(crate) fn run() {
    dbg!(part1(include_str!("../assets/2.txt")));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "#;

        let (_, _tensor) = parse(input).unwrap();
    }

    #[test]
    fn test_part1() {
        let input = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "#;
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = r#"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "#;
        assert_eq!(part2(input), 4);
    }
}
