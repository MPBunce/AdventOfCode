use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::collections::HashSet;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from) 
        .collect()
}

pub fn pt_two () -> std::io::Result<()> {

    let lines = read_lines("./src/input.txt");
    let times = &lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    //println!("{:?}", times);
    
    let distances = lines[1]
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    //println!("{:?}", distances);


    let joined_number: String = times.iter().map(|&num| num.to_string()).collect();
    let joined_distance: String = distances.iter().map(|&num| num.to_string()).collect();
    println!("{:?}", joined_number);
    println!("{:?}", joined_distance);
    let mut data: Vec<(usize, usize)> = Vec::new();

    data.push((joined_number.parse::<usize>().unwrap() , joined_distance.parse::<usize>().unwrap() ));

    let res = result(&data);
    println!("Total: {:?}", res);

    Ok(())
}

fn result(data: &[(usize, usize)]) -> usize {
    data.iter()
        .map(|(time, record)| (0..*time).filter(|x| x * (*time - x) > *record).count())
        .reduce(|result, count| result * count).unwrap()
}