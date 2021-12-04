use std::fs::File;
use std::io::Read;


fn main() {
    let input = read_input().unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<u16>) {
    let mut previous: Option<i32> = None;
    let mut count = 0;
    for x in input {
        if previous.is_some() {
            if *x as i32 - previous.unwrap() > 0 {
                count+= 1;
            }
        }
        previous = Some(*x as i32);
    }
    println!("{}", count);
}

fn part2(input: &Vec<u16>) {
    let mut previous: Option<i32> = None;
    let mut count = 0;
    for i in 0..input.len() {
        if i + 3 > input.len() {
            break;
        }
        let s = window_sum(input, i) as i32;
        if previous.is_some() {
            if s - previous.unwrap() > 0 {
                count+= 1;
            }
        }
        previous = Some(s);
    }
    println!("{}", count);
}

fn window_sum(input: &Vec<u16>, position: usize) -> u32 {
    input[position] as u32 + input[position + 1] as u32 + input[position + 2] as u32
}

fn read_input() -> std::io::Result<Vec<u16>> {
    let mut file = File::open("input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let v: Vec<u16> = data.split('\n').map(|x| x.parse().unwrap()).collect();
    Ok(v)
}