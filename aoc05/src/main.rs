use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::ops::AddAssign;

fn main() {
    let input = read_input().unwrap();
    // part1(&input);
    part2(&input);
}

fn part1(lines: &[Line]) {
    let mut points = HashMap::new();
    let mut process_point = |p: Point| {
        let c = points.entry(p).or_insert(0);
        *c += 1;
    };

    for line in lines {
        let p1 = line.0;
        let p2 = line.1;
        if p1.x == p2.x {
            for y in p1.y..=p2.y {
                process_point(Point{x: p1.x, y});
            }
            for y in p2.y..=p1.y {
                process_point(Point{x: p1.x, y});
            }
        } else if p1.y == p2.y {
            for x in p1.x..=p2.x {
                process_point(Point{x, y: p1.y});
            }
            for x in p2.x..=p1.x {
                process_point(Point{x, y: p1.y});
            }
        }
    }
    let mut count = 0;
    for (_p, c) in points {
        if c > 1 {
            count += 1;
        }
    }
    println!("{}", count);
}

fn part2(lines: &[Line]) {
    let mut points = HashMap::new();
    let mut process_point = |p: Point| {
        let c = points.entry(p).or_insert(0);
        *c += 1;
    };

    for line in lines {
        let p1 = line.0;
        let p2 = line.1;

        let mut delta = Point{x: (p2.x - p1.x), y: (p2.y - p1.y)};
        if delta.x != 0 {
            delta.x = delta.x / delta.x.abs();
        } 
        if delta.y != 0 {
            delta.y = delta.y / delta.y.abs();
        } 
        let mut current = p1;
        while current != p2 {
            process_point(current);
            current += delta;
        }
        process_point(current);
    }
    // print_points(&points);
    let mut count = 0;
    for (_p, c) in points {
        if c > 1 {
            count += 1;
        }
    }
    println!("{}", count);
}

fn print_points(points: &HashMap<Point, usize>) {
    for i in 0..10 {
        for j in 0..10 {
            let p = Point{x: j, y: i};
            let v = points.get(&p).or(Some(&0));
            print!("{:?}", v.unwrap());
        }
        println!("");
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn new(s: &str) -> Point {
        let data: Vec<i32> = s.split(",").map(|x| x.parse().unwrap()).collect();
        Point {
            x: data[0],
            y: data[1]
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

type Line = (Point, Point);

fn read_input() -> std::io::Result<Vec<Line>> {
    let mut file = File::open("input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let raw = data.split('\n').map(|s| s.to_string()).collect::<Vec<String>>();
    let mut output: Vec<Line> = vec![];
    for line_str in raw {
        let x: Vec<Point> = line_str.split(" -> ").map(|p| Point::new(p)).collect();
        output.push((x[0], x[1]));
    }
    Ok(output)
}