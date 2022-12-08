
fn main() {

    let data = include_str!("./sample7.txt");

    let commands: Vec<&str> = data.split("\n").collect();
    println!("{:?}", commands);
}
