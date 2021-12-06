use std::fs::File;
use std::io::Read;
use std::collections::VecDeque;
use std::iter::FromIterator;


fn main() {
    let input = read_input().unwrap();
    part1(&input, 80);
    part2(&input, 256);
}

fn part1(input: &Vec<u32>, days: u32) {
    let mut population = input.clone();
    
    let mut current_day = 0;
    while current_day < days {
        let mut new_counter = 0;
        for fish in population.iter_mut() {
            if *fish == 0 {
                *fish = 7;
                new_counter += 1;
            }
            *fish -= 1;
        }
        for _ in 0..new_counter {
            population.push(8);
        }
        current_day += 1;
    }
    println!("{}", population.len());
}

fn part2(input: &Vec<u32>, days: u32) {
    // 8 day window
    let mut window: VecDeque<u64> = VecDeque::from_iter([0; 9]);
    for x in input {
        let i = *x as usize;
        window[i] += 1;
    }

    let mut n_fish = input.len() as u64;
    for _ in 0..days {
        let front = window.pop_front().unwrap();
        n_fish += front;
        window.push_back(front);
        if front != 0 {
            window[6] += front;
        }
    }
    println!("{}", n_fish);
}

fn read_input() -> std::io::Result<Vec<u32>> {
    let mut file = File::open("input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let v: Vec<u32> = data.split(',').map(|x| x.parse().unwrap()).collect();
    Ok(v)
}