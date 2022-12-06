
fn main() {
    let data = include_str!("input6.txt");

    for (i, _) in data.chars().enumerate() {
        let first = data.chars().nth(i).expect("should exist");
        let second = data.chars().nth(i+1).expect("should exist");
        let third = data.chars().nth(i+2).expect("should exist");
        let forth = data.chars().nth(i+3).expect("should exist");
        if first != second && first != third && first != forth && second != third && second != forth && third != forth { 
            println!("result {:?}", i + 4);
            return;
        }
    }

}
