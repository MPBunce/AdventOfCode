use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn pt_two () -> std::io::Result<()> {

    let mut sum: i32 = 0;
    let mut red_min: i32 = 0;
    let mut green_min: i32 = 0;
    let mut blue_min: i32 = 0;

    // Open the file for reading
    let file = File::open("./src/input.txt")?;

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

        println!("{}", line);
        let split_one: Vec<_> = line.split(": ").collect();
        let game_char: Vec<_> = split_one[0].split(" ").collect();
        println!("{:?}", game_char);
        let game_int: i32 = game_char[1].parse().unwrap();
        println!("{:?}", game_int);
        let split_two: Vec<_> = split_one[1].split("; ").collect();

        for n in split_two{
            let split_three: Vec<_> = n.split(", ").collect();
            for n in split_three{

                let mut flag = true;
                println!("Line: {:?}, Game Int: {:?}", n, &game_int);

                let b = n.find("blue");
                let g = n.find("green");
                let r = n.find("red");
                
                if let Some(c) = b {
                    let blue_int: i32 = n.split_whitespace().next().unwrap().parse().unwrap();
                    if blue_int > blue_min {
                        blue_min = blue_int
                    }

                    
                }
                else if let Some(c) = g {
                    let green_int: i32 = n.split_whitespace().next().unwrap().parse().unwrap();
                    if green_int > green_min {
                        green_min = green_int
                    }

                }
                else if let Some(c) = r {
                    let red_int: i32 = n.split_whitespace().next().unwrap().parse().unwrap();
                    if red_int > red_min {
                        red_min = red_int
                    }

                }
                else {
                    println!("Error");
                }

            }

        }
        println!("Red: {:?}, Greeb: {:?}, Blue: {:?},", red_min, green_min, blue_min);
        //Calculate
        let mut temp = red_min * green_min * blue_min;
        println!("{:?}", &temp);
        sum += temp;

        //Reset
        red_min = 0;
        green_min = 0;
        blue_min = 0;

    }
    println!("{:?}", sum);
    Ok(())
}
