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

pub fn pt_one() -> io::Result<()> {

    let lines = read_lines("./src/input.txt");
    let instructions = &lines[0];

    let char_vec: Vec<char> = instructions.chars().collect();
    let mut char_map: HashMap<String, (String, String)> = HashMap::new();

    for i in 2..lines.len() {
        let n = &lines[i];
        let res: Vec<_> = n.split(&[' ', ',', '(', ')'][..]).collect();
        let new_key = res[0].to_string(); 
        let new_tuple: (String, String) = (res[3].to_string(), res[5].to_string());
    
        char_map.insert(new_key, new_tuple);
    
    }

    let mut current_chars = "AAA".to_string();
    let mut count:i32 = 0;

    let infinite_iterator = char_vec.iter().cycle();

    for char in infinite_iterator {

        let holder = char_map.get(&current_chars).unwrap();
        let temp_char = *char;

        let left = holder.0.clone();
        let right = holder.1.clone();

        match temp_char {
            'L' => current_chars = left,
            _ => current_chars = right
        }

        count = count + 1;
        if current_chars == "ZZZ".to_string() {
            break;
        }
    }

    println!("Count: {:?}", count);

    Ok(())

}