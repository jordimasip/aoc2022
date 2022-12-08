use std::str::FromStr;

use anyhow::Result;

#[derive(Debug)]
struct Crane {
    stack: Vec<Vec<Option<char>>>,
}

impl Crane {
    fn move_crane(&mut self, m: &Move) {
       for _ in 0..m.counter {
           let item = self.stack[m.from].pop().expect("should exist");
           self.stack[m.to].push(item);
       }
    }

    fn print_bottom(&self) {
        for stack in &self.stack {
            print!("{}", stack.last().expect("should exist").expect("should EXIST"));
        }
    }
}

impl FromStr for Crane {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for line in s.lines().rev().skip(1) {
            let mut idx = 0;
            let mut crane_idx = 0;

            while idx < line.len() {
                if stack.len() <= crane_idx {
                    stack.push(Vec::new());
                }

                if line[idx..].starts_with("[") {
                    let c = line.chars().nth(idx + 1).expect("must exist");
                    stack[crane_idx].push(Some(c));
                }

                idx += 4;
                crane_idx += 1;
            }
        }

        return Ok(Crane {
            stack,
        });
    }
}

#[derive(Debug)]
struct Move {
    counter: usize,
    from: usize,
    to: usize,
}

impl FromIterator<usize> for Move {
   fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        let counter = iter.next().expect("should exist");
        let from = iter.next().expect("should exist")-1; 
        let to = iter.next().expect("should exist")-1;
       
        return Move { 
           counter, 
           from,
           to,
        }
   } 
}

impl FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // "move 1 from 2 to 1"
        return Ok(s
                  .split_whitespace()
                  .flat_map(|x| x.parse::<usize>())
                  .map(|x| {
                      //println!("map from_str Move {}", x);
                      return x;
                  })
                  .collect::<Move>());
    }
}


fn main() -> Result<()> {
    let data = include_str!("./input5.txt");

    let division: Vec<&str> = data.split("\n\n").collect();

    let cranes = division[0];
    let mut crane = cranes.parse::<Crane>()?;
    
    let moves = division[1]
        .lines()
        .filter(|x| !x.is_empty())
        .flat_map(|x| x.parse::<Move>())
        .collect::<Vec<Move>>();
    
    //println!("{:?}", crane);
    for m in moves {
        crane.move_crane(&m);
    }
    //println!("{:?}", crane);

    crane.print_bottom();
    return Ok(());
}

#[cfg(test)]
mod test {
    use anyhow::{Result, Ok};

    #[test]
    fn test_crane_parse() -> Result<()> {
        let str= include_str!("./sample5.txt").split("\n\n").collect::<Vec<_>>()[0];
        let crane = str.parse::<super::Crane>()?;

        println!("crane = {:?}", crane);

        return Ok(());
    }
}

