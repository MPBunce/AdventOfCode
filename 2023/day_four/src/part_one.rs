use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn pt_one () -> std::io::Result<()> {

    let mut sum: i32 = 0;
    // Open the file for reading
    let file = File::open("./src/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line
    for line_result in reader.lines() {

        let mut ticket_sum = 0;

        let line = line_result?;
        println!("{}", line);

        let split_one: Vec<_> = line.split(": ").collect();
        let split_two: Vec<_> = split_one[1].split(" | ").collect();

        let winning_nums: HashSet<i32> = split_two[0]
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse"))
            .collect();

        println!("{:?}", winning_nums);
        
        let my_nums: Vec<i32> = split_two[1]
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse"))
            .collect();
        println!("{:?}", my_nums);

        for n in my_nums{
            if winning_nums.contains(&n){
                
                if ticket_sum == 0 {
                    ticket_sum = 1;
                } else {
                    ticket_sum = ticket_sum * 2;
                }

            }

        }
        println!("{:?}", ticket_sum);

        sum += ticket_sum;
    }

    println!("{:?}", sum);

    Ok(())

}