use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() -> std::io::Result<()> {

    //Part One

    let mut my_vector: Vec<Vec<i32>> = Vec::new();

    // Open the file for reading
    let file = File::open("./src/input.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line
    for line_result in reader.lines() {
        
        let line = line_result?;
        //println!("{}", &line);
        let iterable = line.clone();

        let mut string_count: Vec<i32> = Vec::new();

        for c in iterable.chars().enumerate(){
            if c.1.is_numeric(){
                //println!("{:?}", &c.1);
                let new_int = c.1.to_digit(10).unwrap() as i32;
                //println!("int: {}", &new_int);
                string_count.push(new_int)
            }            
        }

        my_vector.push(string_count)

    }

    let mut sum: i32 = 0;

    for row in &my_vector {
        if let (Some(first), Some(last)) = (row.first(), row.last()) {
            // Create a number from the first and last elements
            let combined_number = first * 10 + last;
            //println!("Combined Number: {}", &combined_number);
            sum += combined_number;
        }
    }

    println!("The Part 1 Total is: {}", sum);
    println!("\n\n");


    let mut sum_two = 0;

    let string_to_int = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let file_two = File::open("./src/input.txt")?;
    let reader_two = BufReader::new(file_two);

    for line_result_two in reader_two.lines() {
        let line = line_result_two?;
        println!("{}", line);
        let mut index_and_position: HashMap<usize, i32> = HashMap::new();

        for (k, v) in &string_to_int {

            // if let Some(position) = line.find(k) {

            //     index_and_position.insert(position, *v);
            // }
            let col: Vec<_> = line.match_indices(k).collect();

            for (index, val) in &col {
                index_and_position.insert(*index, *v);
            }


        }

        let iterable = line.clone();
        for (index, c) in iterable.chars().enumerate(){
            if c.is_numeric(){
                //println!("{:?}", &c.1);
                let new_int = c.to_digit(10).unwrap() as i32;
                index_and_position.insert(index, new_int);
            }            
        }

        let min = index_and_position.keys().min();
        let min_val = index_and_position.get(min.unwrap());
        let max = index_and_position.keys().max();
        let max_val = index_and_position.get(max.unwrap());
        let combined_number = min_val.unwrap() * 10 + max_val.unwrap();
        sum_two += combined_number;
        println!("Index and Position: {:?}", index_and_position);
        println!("Num: {}", combined_number);
    }

    println!("The sum {}", sum_two);

    Ok(())

}
