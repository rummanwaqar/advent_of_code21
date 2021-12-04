use std::fs::File;
use std::io::Read;

fn main() {
    let input = read_input().unwrap();
    part1(&input);
    part2(&input);
}

#[derive(Debug)]
enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
    Unknown
}

fn part1(input: &Vec<String>) {
    let mut x = 0;
    let mut z = 0;
    for l in input {
        match parse(l) {
            Command::Forward(v) => x += v,
            Command::Up(v) => z -= v,
            Command::Down(v) => z += v,
            Command::Unknown => println!("bad command")
        };
    }
    println!("{}", x * z);
}

fn part2(input: &Vec<String>) {
    let mut x = 0;
    let mut z = 0;
    let mut a = 0;
    for l in input {
        match parse(l) {
            Command::Forward(v) => {
                x += v;
                z += v * a;
            },
            Command::Up(v) => a -= v,
            Command::Down(v) => a += v,
            Command::Unknown => println!("bad command")
        };
    }
    println!("{}", x * z);
}

fn parse(s: &String) -> Command {
    let sp = s.split(" ").collect::<Vec<&str>>();
    let val: u32 = sp[1].parse().unwrap();
    match sp[0] {
        "forward" => Command::Forward(val),
        "up" => Command::Up(val),
        "down" => Command::Down(val),
        _ => Command::Unknown
    }
}

fn read_input() -> std::io::Result<Vec<String>> {
    let mut file = File::open("input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let v: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();
    Ok(v)
}