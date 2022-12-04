use anyhow::Result;

fn main() -> Result<()> {
    let data = include_str!("./input4.txt");
    let lines: Vec<&str> = data.split("\n").collect();

    let result: Vec<&str> = lines.iter().map(|row| {
        let pairs: Vec<&str> = row.split(",").collect();
        let first_pair = pairs[0];
        let second_pair = pairs[1];
        let separed_first_pair: Vec<&str> = first_pair.split("-").collect();
        let min_first_pair: i8 = separed_first_pair[0].parse().unwrap();
        let max_first_pair: i8 = separed_first_pair[1].parse().unwrap();
        let separed_second_pair: Vec<&str> = second_pair.split("-").collect();
        let min_second_pair: i8 = separed_second_pair[0].parse().unwrap();
        let max_second_pair: i8 = separed_second_pair[1].parse().unwrap();
        if (min_first_pair <= min_second_pair && max_first_pair >= max_second_pair) || (min_second_pair <= min_first_pair && max_second_pair >= max_first_pair) {
            "1"
        } else {
            "0"
        }
    }).collect();

    let sum: Vec<u16> = result.iter().map(|a| a.parse::<u16>().unwrap()).collect();
    let res: u16 = sum.iter().sum();
    println!("{:?}", res);
    return Ok(());
}
