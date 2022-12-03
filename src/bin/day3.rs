use std::collections::HashSet;


fn main() {
    let info = include_str!("./input3.txt");
    let parsed_info: Vec<&str> = info.split("\n").collect();
    println!("{:?}", parsed_info);

    let response: Vec<char> = parsed_info.iter().map(|a| {
        let length = a.chars().count();
        let first_part = &a[..length/2];
        let last_part = &a[length/2..];
        let first_chars: HashSet<char> = first_part.chars().collect();
        let last_chars: HashSet<char> = last_part.chars().collect();
        let result: char = first_chars.intersection(&last_chars).copied().last().unwrap();
        result
    }).collect();
    println!("{:?}", response);
    let dictionary = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let res: Vec<usize> = response.iter().map(|e| {
        let position = dictionary.find(e.clone()).unwrap(); 
        position + 1
    }).collect();
    println!("{:?}", res.iter().sum::<usize>());

}
