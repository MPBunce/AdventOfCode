use std::fs::read_to_string;
use std::fs::File;
use std::io;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from) 
        .collect()
}

pub fn pt_two() -> io::Result<()> {

    let lines = read_lines("./src/input.txt");
    let instructions = &lines[0];

    let char_vec: Vec<char> = instructions.chars().collect();
    let mut char_map: HashMap<String, (String, String)> = HashMap::new();

    let mut staring_points: Vec<String> = Vec::new();

    for i in 2..lines.len() {
        let n = &lines[i];
        let res: Vec<_> = n.split(&[' ', ',', '(', ')'][..]).collect();
        let new_key = res[0].to_string();
        if new_key.contains("A") {
            staring_points.push(new_key.clone())
        }
        let new_tuple: (String, String) = (res[3].to_string(), res[5].to_string());
    
        char_map.insert(new_key, new_tuple);
    
    }

    println!("{:?}", instructions);
    println!("{:?}", staring_points);

    let mut count: i64 = 0;

    let infinite_iterator = char_vec.iter().cycle();


    'outer_loop: for char in infinite_iterator {

        let temp_staring_points = staring_points.clone();

        for (index, char_value) in temp_staring_points.iter().enumerate() {
            
            //println!("{:?}", char_value);
            let t = char_value.clone();
            let holder = char_map.get(&t).unwrap();

            let left = holder.0.clone();
            let right = holder.1.clone();

            match char {
                'L' => staring_points[index] = left,
                _ => staring_points[index] = right
            }




        }

        let all_z = staring_points.iter().all(|ch| ch.contains("Z"));
        count = count + 1;
        if all_z {
            break 'outer_loop;
        } else {
            continue;
        }


    }

    println!("Count pt2: {:?}", count);

    Ok(())

}

