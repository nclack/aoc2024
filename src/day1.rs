use std::collections::HashMap;

use burn::{
    backend::NdArray,
    tensor::{Int, Shape, Tensor, TensorData},
};
use itertools::Itertools;
use nom::{
    character::complete::{digit1, multispace0, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::preceded,
    IResult,
};

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

fn parse(input: &str) -> IResult<&str, Tensor<NdArray, 2, Int>> {
    let (input, rows) = preceded(multispace0, separated_list1(multispace1, number))(input)?;
    let n = rows.len();
    Ok((input, TensorData::new(rows, Shape::new([n / 2, 2])).into()))
}

fn part1(input: &str) -> i64 {
    let (_, tensor) = parse(input).unwrap();
    let tensor = tensor.sort(0);
    let (left, right) = tensor.iter_dim(1).next_tuple().unwrap();
    (left - right).abs().sum().into_scalar()
}

fn histogram(tensor: Tensor<NdArray, 1, Int>) -> HashMap<i64, i64> {
    let mut counter = HashMap::new();
    for e in tensor.into_data().iter() {
        *counter.entry(e).or_default() += 1;
    }
    counter
}

fn part2(input: &str) -> i64 {
    let (_, tensor) = parse(input).unwrap();
    let (left, right) = tensor.iter_dim(1).next_tuple().unwrap();
    let n = histogram(right.squeeze(1));
    left.into_data()
        .iter()
        .map(|e: i64| n.get(&e).unwrap_or(&0) * e)
        .sum()
}

pub fn run() {
    dbg!(part1(include_str!("../assets/1.txt")));
    dbg!(part2(include_str!("../assets/1.txt")));
}

#[cfg(test)]
mod tests {
    use burn::backend::NdArray;

    use super::*;
    #[test]
    fn test_parse() {
        let input = r#"
            3    4
            4    3
            2    5
            1    3
            3    9
            3    3
        "#;
        let expected: Tensor<NdArray, 2, _> =
            TensorData::new(vec![3, 4, 4, 3, 2, 5, 1, 3, 3, 9, 3, 3], Shape::new([6, 2])).into();

        let (_, tensor) = parse(input).unwrap();

        assert_eq!(tensor.shape(), expected.shape());
        assert!(tensor.equal(expected).all().into_scalar());
    }

    #[test]
    fn test_part1() {
        let input = r#"
            3    4
            4    3
            2    5
            1    3
            3    9
            3    3
        "#;
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = r#"
            3    4
            4    3
            2    5
            1    3
            3    9
            3    3
        "#;
        assert_eq!(part2(input), 31);
    }
}
