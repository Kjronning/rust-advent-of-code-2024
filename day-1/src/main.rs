use std::collections::HashMap;
use std::fs::File;
use std::io;
mod reader;


// gets the input, makes left and right column of sorted integers
fn parse_input(lines: io::Lines<io::BufReader<File>>) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines.flatten() {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().to_string().parse().unwrap());
    }
    left.sort();
    right.sort();
    return (left, right); 
}

fn main() {
    let input = reader::read("./input.txt");
    let (left, right) = parse_input(input);
    
    // Part One
    // calculate distances
    let distance: i32 = left.clone().into_iter().zip(right.clone().into_iter()).map(|(x, y)| (x-y).abs()).reduce(|acc, e| acc+e).unwrap();
    
    // print result
    println!("Part One result: {}", distance);

    //Part Two
    let mut similarities: HashMap<i32, i32> = HashMap::new();

    // get similarities
    left.clone().into_iter().for_each(|n| { similarities.entry(n).or_insert(right.clone().into_iter().filter(|v: &i32| n == v.clone()).count() as i32); });

    //reduce total
    let similarity_score:i32 = similarities.into_iter().map(|(key, value)| key*value).reduce(|acc, e| acc + e).unwrap();

    // print result
    println!("Part Two result: {similarity_score}");

}

