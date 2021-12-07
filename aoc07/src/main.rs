use std::fs::File;
use std::io::Read;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input = read_input().unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &[i32]) {
    let unique = HashSet::<i32>::from_iter(input.iter().cloned());
    let mut min_distance = i32::MAX;
    for value in unique {
        let mut distance = 0;
        for x in input {
            distance += (x - value).abs();
        }
        if distance < min_distance {
            min_distance = distance;
        }
    }
    println!("{}", min_distance);
}

fn part2(input: &[i32]) {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();
    let mut min_distance = i32::MAX;

    let fuel = |x: i32| x*(x+1)/2;
    for value in min..=max {
        let mut distance = 0;
        for x in input {
            distance += fuel((x - value).abs());
        }
        if distance < min_distance {
            min_distance = distance;
        }
    }
    println!("{}", min_distance);
}

fn read_input() -> std::io::Result<Vec<i32>> {
    let mut file = File::open("input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let v: Vec<i32> = data.split(',').map(|x| x.parse().unwrap()).collect();
    Ok(v)
}