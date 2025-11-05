use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::collections::HashMap;

pub fn pt_two () -> std::io::Result<()> {

    let mut sum: i32 = 0;
    // Open the file for reading
    let file = File::open("./src/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);


    let mut winners: Vec<_> = Vec::new();
    let mut mine: Vec<_> = Vec::new();
    // Read the file line by line
    for line_result in reader.lines() {

        let line = line_result?;
        println!("{}", line);

        let split_one: Vec<_> = line.split(": ").collect();
        let split_two: Vec<_> = split_one[1].split(" | ").collect();

        let winning_nums: HashSet<i32> = split_two[0]
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse"))
            .collect();

        
        winners.push(winning_nums);

        let my_nums: Vec<i32> = split_two[1]
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse"))
            .collect();

        mine.push(my_nums);

    }

    let mut my_map: HashMap<usize, i32> = HashMap::new();
    let mut count_map: HashMap<usize, i32> = HashMap::new();
    let mut total_cards_map: HashMap<usize, i32> = HashMap::new();
    let mut ticket_sum = 0;
    let mut count = 0;
    
    for i in 0..mine.len() {
        let n = &mine[i];

        for j in 0..n.len(){
            let m = n[j];

            if winners[i].contains(&m){
                count += 1;
                if ticket_sum == 0 {
                    ticket_sum = 1;
                } else {
                    ticket_sum = ticket_sum * 2;
                }

            }
        }
        total_cards_map.insert(i, 1);

        my_map.insert(i, ticket_sum);
        count_map.insert(i, count);
        ticket_sum = 0;
        count = 0;
    }

    println!("{:?}", count_map);
    println!("{:?}", total_cards_map);

    //let mut sum: i32 = 0;
    
    for i in 0..count_map.len() {
        
        let num_cards = total_cards_map.get(&i);
        let iterate_x_time= count_map.get(&i);
        println!("iterate {:?}", &iterate_x_time);

        for j in 0..*num_cards.unwrap() {
            println!("test j {:?}", &j);
            for k in 0..*iterate_x_time.unwrap() {

                let position: usize = (k as i32 + i as i32 + 1) as usize;

                println!("test k {:?}", position);
                total_cards_map.entry(position).and_modify(|v| *v += 1);
            }

        }

    }

    let sum: i32 = total_cards_map.values().copied().sum();
    println!("P2 Sum: {:?}", sum);

    Ok(())
}
