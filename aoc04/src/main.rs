use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let input = read_input().unwrap();
    // part1(input);
    part2(input);
}

fn part1(mut game: GameManager) {
    'outer: for x in game.numbers {
        for p in &mut game.puzzles {
            if p.mark(x) {
                println!("{}", calculate_score(p) * x);
                break 'outer;
            }
        }
    }
}

fn part2(mut game: GameManager) {
    let mut to_skip = HashSet::new();

    for x in game.numbers {
        let mut i = 0;
        let mut last_score = 0;

        for p in &mut game.puzzles {
            if !to_skip.contains(&i) && p.mark(x) {
                last_score = calculate_score(p) * x;
                to_skip.insert(i);
            }
            i += 1;
        }

        if game.puzzles.len() == to_skip.len() {
            println!("{}", last_score);
            break;
        }
    }
}

#[derive(Debug)]
struct GameManager {
    numbers: Vec<u32>,
    puzzles: Vec<Puzzle>
}

impl GameManager {
    fn new(input: &[String]) -> GameManager {
        let mut puzzles = vec![];
        let n_puzzles = (input.len() - 1) / 6;
        for i in 0..n_puzzles {
            let starting = i * 6 + 2;
            let ending = starting + 5;
            let puzzle = Puzzle::new(&input[starting..ending]);
            puzzles.push(puzzle);
        }

        GameManager {
            numbers: input[0].split(",").map(|x| x.parse().unwrap()).collect::<Vec<u32>>(),
            puzzles
        }
    }
}

#[derive(Debug)]
struct Puzzle {
    puzzle: Vec<u32>,
    checked: Vec<bool>
}

impl Puzzle {
    fn new(input: &[String]) -> Puzzle {
        let mut puzzle: Vec<u32> = Vec::new();
        for s in input {
            let l: Vec<u32> = s.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
            puzzle.extend(l);
        }     
        Puzzle {
            puzzle,
            checked: vec![false; 25]
        }
    }

    fn mark(&mut self, n: u32) -> bool {
        for (i, x) in self.puzzle.iter().enumerate() {
            if *x == n {
                self.checked[i] = true;
            }
        }
        for i in 0..5 {
            if self.check_row(i) {
                return true;
            }
            if self.check_col(i) {
                return true;
            }
        }
        false
    }

    fn check_row(&self, id: usize) -> bool {
        let start = id * 5;
        self.checked[start] && self.checked[start+1] && self.checked[start+2] && 
            self.checked[start+3] && self.checked[start+4]
    }

    fn check_col(&self, id: usize) -> bool {
        self.checked[0 + id] && self.checked[5 + id] && self.checked[10 + id] && 
            self.checked[15 + id] && self.checked[20 + id]
    }
}

fn calculate_score(puzzle: &Puzzle) -> u32 {
    let mut score = 0;
    for (i, c) in puzzle.checked.iter().enumerate() {
        if *c == false {
            score += puzzle.puzzle[i];
        }
    }
    score
}

fn read_input() -> std::io::Result<GameManager> {
    let mut file = File::open("input.txt")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let v: Vec<String> = data.split('\n').map(|s| s.to_string()).collect();
    Ok(GameManager::new(&v))
}