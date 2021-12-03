use std::fs::File;
use std::io::Read;



fn main() {
    let input = read_input().unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<String>) {
    let half_inputs = (input.len() / 2) as u16;

    let most_common: Vec<u8> = count_1s(&input).iter().map(|x| if *x > half_inputs {1} else {0}).collect();
    let least_common = invert(&most_common);

    let power = to_u32(&most_common) * to_u32(&least_common);
    println!("{:?}", power);
}

fn part2(input: &Vec<String>) {
    let n_bits = input[0].len();
    let mut count_1: Vec<u16> = Vec::new();

    let mut current_list = input.to_vec();
    for i in 0..n_bits {
        if current_list.len() == 1 {
            break;
        }
        let count = count_1s_bit(&current_list, i);
        let bit = if count >= current_list.len() as u16 - count {
            '1'
        } else {
            '0'
        };
        current_list = current_list.into_iter().filter(|x| x.chars().nth(i).unwrap() == bit).collect();
    }
    let oxygen:Vec<u8> = current_list[0].chars().map(|x| if x == '1' {1} else {0}).collect();

    let mut current_list = input.to_vec();
    for i in 0..n_bits {
        if current_list.len() == 1 {
            break;
        }
        let count = count_1s_bit(&current_list, i);
        let bit = if count < current_list.len() as u16 - count {
            '1'
        } else {
            '0'
        };
        current_list = current_list.into_iter().filter(|x| x.chars().nth(i).unwrap() == bit).collect();
        if current_list.len() == 1 {
            break;
        }
    }
    let c02:Vec<u8> = current_list[0].chars().map(|x| if x == '1' {1} else {0}).collect();
    
    println!("{:?}", to_u32(&oxygen) * to_u32(&c02));
}

fn count_1s(input: &Vec<String>) -> Vec<u16> {
    let n_bits = input[0].len();
    let mut count_1: Vec<u16> = Vec::new();
    for i in 0..n_bits {
        count_1.push(count_1s_bit(input, i));
    }
    count_1
}

fn count_1s_bit(input: &Vec<String>, position: usize) -> u16 {
    let mut count = 0;
    for s in input {
        let c = s.chars().nth(position).unwrap();
        if c == '1' {
            count += 1;
        } 
    }
    return count
}

fn to_u32(slice: &[u8]) -> u32 {
    slice.iter().fold(0, |acc, &b| acc*2 + b as u32)
}

fn invert(bits: &Vec<u8>) -> Vec<u8> {
   bits.iter().map(|x| if *x == 0 {1} else {0}).collect()
}

fn read_input() -> std::io::Result<Vec<String>> {
    let mut file = File::open("input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let v: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();
    Ok(v)
}