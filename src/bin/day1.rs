fn main() {

    let data = include_str!("input1.txt");
    let info = data.split("\n\n");
    let mut parsed_data: Vec<u32> = info
        .map(|line| {
            line.split("\n")
                .flat_map(|n| n.parse::<u32>())
                .sum::<u32>()
        }).collect();

    parsed_data.sort();
    parsed_data.reverse();

    println!("{}", parsed_data[0]);
    println!("{:?}", parsed_data[0] + parsed_data[1] + parsed_data[2]);
}
