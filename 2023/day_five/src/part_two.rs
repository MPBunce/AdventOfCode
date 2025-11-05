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
    let mut global_min:(i64, bool) = (10000000, false); 

    let lines = read_lines("./src/input.txt");

    let seeds = &lines[0]
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut chunked_seeds = seeds.chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<(i64, i64)>>();
    println!("{:?}", chunked_seeds);

    let mut all_seeds: Vec<i64> = Vec::new();

    for n in chunked_seeds {

        let total_seeds: Vec<i64> = (n.0..n.0 + n.1 as i64).collect();

        let mut locations_vec: Vec<(i64, bool)> = Vec::new();
        
        for n in total_seeds {
            locations_vec.push((n, false));
        }

        for line in &lines[1..] {

            if line.is_empty(){      
                continue;
            } else {
                if line.contains("map:") {
                    let temp_name: Vec<_> = line.split(" map:").collect();
                    for i in 0..locations_vec.len() {
                        locations_vec[i].1 = false
                    }
                } else {
                    let values: Vec<i64> = line
                        .split_whitespace()
                        .map(|s| s.parse::<i64>().expect("Failed to parse usize"))
                        .collect();
                    //println!("{:?}",values);

                    let des_range = values[0];
                    let source_range = values[1];
                    let range = values[2];


                    let destination_map: Vec<i64> = (des_range..des_range + range as i64).collect();
                    //println!("{:?}", destination_map);                
                    let source_map: Vec<i64> = (source_range..source_range + range as i64).collect();
                    //println!("{:?}", source_map); 

                    for i in 0..locations_vec.len() {
                        let temp_seed = locations_vec[i].0;
                        if let Some(index) = source_map.iter().position(|&x| x == temp_seed) {
                            if locations_vec[i].1 != true {
                                locations_vec[i].0 = destination_map[index];
                                locations_vec[i].1 = true;
                            }   

                        } else {
                            //println!("Value {} not found in the vector",temp_seed);
                            continue;
                        }                    
                    }



                }
            }
        }


        if let Some(min_value) = locations_vec.iter().cloned().min() {
            if min_value.0 < global_min.0 {
                global_min = min_value;
            }
        } else {
            continue;
        }


    }


    println!("{:?}", global_min);

    Ok(())
}
