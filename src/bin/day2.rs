
fn main() {
    let data = include_str!("./input2.txt");
    let round: Vec<&str> = data.split("\n").collect();
    let _temp: Vec<u32> = round.iter().map(|element| {
        let couple: Vec<&str> = element.split(" ").collect();
        let other = couple[0];
        let me = couple[1];
        let mut result;
        
        if me == "Y" {
            result = 2
        } else if me == "X" {
            result = 1
        } else {
            result = 3
        }


        if other == "A" {
            if me == "Y" {
                result += 6;
            } else if me == "X" {
                result += 3 
            } else {
                result += 0 
            }
        } else if other == "B"{
            if me == "Y" {
                result += 3;
            } else if me == "X" {
                result += 0
            } else {
                result += 6 
            }
        } else {
            if me == "Y" {
                result += 0;
            } else if me == "X" {
                result += 6 
            } else {
                result += 3
            }
        }
        result
    }).collect();

    let result: u32 = _temp.iter().sum();
    
    println!("{:?}", result); 

}

