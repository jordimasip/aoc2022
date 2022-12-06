use core::fmt;
use std::str::FromStr;

use anyhow::Result;

struct Crane {
    stack: Vec<Vec<char>>,
}

impl Crane {
   fn new() -> Self {
       Crane {
           stack: Vec::new()
       }
   }
}

impl FromStr for Crane {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::new();
        for line in s.lines().rev() {
            let mut idx = 0;
            let mut crane_idx = 0;

            // A line looks like this:
            // [M] [B] [C]
            // [A]     [E]
            while idx < line.len() {
               if line[idx..].starts_with("[") {
                    let c = line.chars().nth(idx - 1).expect("must exist");
                    if stack.len() <= crane_idx {
                        stack.push(Vec::new());
                    }
                    stack[crane_idx].push(c);
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

struct Position {
    who: i16,
    from: i16,
    to: i16,
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Position")
            .field("who", &self.who)
            .field("from", &self.from)
            .field("to", &self.to)
            .finish()
    }
}

fn main() -> Result<()> {
    let data = include_str!("./sample5.txt");
    //println!("{:?}", data);

    let division: Vec<&str> = data.split("\n\n").collect();

    //println!("{:?}", division);

    let cranes = division[0];
    let crane = cranes.parse::<Crane>()?;
    
    let orders = division[1];
    
    //println!("{:?}", orders);
    let list_orders: Vec<Position> = orders.split("\n")
        .map(|e| {
            let parts: Vec<&str> = e.split(" ").collect();

            let p = Position {
                who: parts[1].parse::<i16>().unwrap(),
                from: parts[3].parse::<i16>().unwrap(),
                to: parts[5].parse::<i16>().unwrap(),
            };
            p
    }).collect();
    println!("{:?}", list_orders);
    
    return Ok(());
}
