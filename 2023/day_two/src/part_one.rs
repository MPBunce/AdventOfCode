use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn pt_one () -> std::io::Result<()> {

    let mut sum = 0;
    let mut sum_two = 0;

    let red_max: i32 = 12;
    let green_max: i32 = 13;
    let blue_max: i32 = 14;

    // Open the file for reading
    let file = File::open("./src/small.txt")?;

    // Create a buffered reader to read the file
    let reader = BufReader::new(file);

    // Read the file line by line
    for line_result in reader.lines() {
        // Handle errors
        let line = match line_result {
            Ok(line) => line,
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                continue; // Skip to the next iteration
            }
        };

        let split_one: Vec<_> = line.split(": ").collect();
        let game_char: Vec<_> = split_one[0].split(" ").collect();
        let game_int: i32 = game_char[1].parse().unwrap();
        sum_two += game_int;
        let split_two: Vec<_> = split_one[1].split("; ").collect();

        'outer: for n in split_two{
            let split_three: Vec<_> = n.split(", ").collect();
            for n in split_three{

                let b = n.find("blue");
                let g = n.find("green");
                let r = n.find("red");
                
                if let Some(c) = b {
                    let blue_int: i32 = n.split_whitespace().next().unwrap().parse().unwrap();
                    if blue_int > blue_max {
                        sum += game_int;
                        break 'outer;
                    }

                    
                }
                else if let Some(c) = g {
                    let green_int: i32 = n.split_whitespace().next().unwrap().parse().unwrap();
                    if green_int > green_max {
                        sum += game_int;
                        break 'outer;
                    }

                }
                else if let Some(c) = r {
                    let red_int: i32 = n.split_whitespace().next().unwrap().parse().unwrap();
                    if red_int > red_max {
                        sum += game_int;
                        break 'outer;
                    }
                }
                else {
                    println!("Error");
                }
            }
        }



    }
    let res = sum_two - sum;
    println!("Sum: {:?}", res);
    println!("\nDone\n\n");

    Ok(())
}
