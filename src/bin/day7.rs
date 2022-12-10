use std::fs;

use anyhow::Result;


fn main() -> Result<()>{

    let input = fs::read_to_string("src/bin/input7.txt")?;
    let mut stack = vec![
        ("/", 0)
    ];

    let report_amount = 100_000;
    let mut total = 0;

    for line in input.lines().filter(|x| !x.is_empty()){
       if line == "$ cd /" || line == "$ ls" {
           continue;
       }

       if line.starts_with("$ cd ") {
           let dir = &line[5..];
           if dir == ".." {
               let (_, amount) = stack.pop().unwrap();
               if amount <= report_amount {
                   total += amount;
               }
               stack.last_mut().unwrap().1 += amount;
           } else {
               stack.push((dir, 0));
           }
           continue;
       }

       let (amount, _) = line.split_once(" ").unwrap();

       if let Ok(amount) = amount.parse::<usize>() {
           stack.last_mut().unwrap().1 += amount;
       } else if amount == "dir" {
          // ignore 
       }

    }
    println!("{}", total);

    return Ok(());
}
