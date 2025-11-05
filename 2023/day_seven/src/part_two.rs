use std::fs::read_to_string;
use std::io::{self};
use std::collections::HashMap;

use crate::models::Hand;

pub fn pt_two() -> io::Result<()> {
    let lines = read_lines("./src/input.txt");

    let mut hands: Vec<Hand> = Vec::new();

    for n in lines {
        let res: Vec<_> = n.split_ascii_whitespace().collect();
        let temp = Hand {
            cards: res[0].to_string(),
            bid: res[1].parse::<i32>().unwrap(),
        };
        hands.push(temp);
    }
    bubble_sort(&mut hands);

    for hand in &hands{
        println!("{:?}", hand);
    }

    let mut sum: i32 = 0;
    for i in 0..hands.len(){
        let bid = hands[i].bid;
        sum += bid * (i as i32 +1 );
    }
    println!("Sum: {:?}", sum);
    Ok(())
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from) 
        .collect()
}

fn bubble_sort(arr: &mut Vec<Hand>) {
    let n = arr.len();

    for i in 0..n {
        for j in 0..(n - 1 - i) {
            if winning_hand(&arr[j].cards , &arr[j + 1].cards){
                arr.swap(j, j + 1);
            }
        }
    }
}

fn char_to_card_type(c: char) -> Option<CharCardType> {
    match c {
        '2' => Some(CharCardType::Two),
        '3' => Some(CharCardType::Three),
        '4' => Some(CharCardType::Four),
        '5' => Some(CharCardType::Five),
        '6' => Some(CharCardType::Six),
        '7' => Some(CharCardType::Seven),
        '8' => Some(CharCardType::Eight),
        '9' => Some(CharCardType::Nine),
        'T' => Some(CharCardType::Ten),
        'J' => Some(CharCardType::Jack),
        'Q' => Some(CharCardType::Queen),
        'K' => Some(CharCardType::King),
        'A' => Some(CharCardType::Ace),
        _ => None,
    }
}

fn winning_hand(hand_one: &String, hand_two: &String) -> bool {

    let h1 = hand_one;
    let h2 = hand_two;

    println!("\n");
    println!("{:?}", h1);
    println!("{:?}", h2);

    
    let mut h1_char = HashMap::new();
    for ch in h1.chars() {
        *h1_char.entry(ch).or_insert(0) += 1;
    }
    if let Some(temp_val) = h1_char.remove(&'J') {
        if let Some(max_char) = h1_char.iter().max_by_key(|&(_, count)| count){
            let temp = *max_char.0;
            h1_char.entry(temp).and_modify(|entry| *entry += temp_val);  
        } else {
            *h1_char.entry('A').or_insert(0) += temp_val;                    
        }

    }

    let mut h2_char = HashMap::new();
    for ch in h2.chars() {
        *h2_char.entry(ch).or_insert(0) += 1;
    }
    if let Some(temp_val) = h2_char.remove(&'J') {
        if let Some(max_char) = h2_char.iter().max_by_key(|&(_, count)| count){
            let temp = *max_char.0;
            h2_char.entry(temp).and_modify(|entry| *entry += temp_val);  
        } else {
            *h2_char.entry('A').or_insert(0) += temp_val;                    
        }

    }

    let h1_max = *h1_char.values().max_by_key(|&v| v).unwrap();
    let h2_max = *h2_char.values().max_by_key(|&v| v).unwrap();

    let mut h1_hand = HandType::HighCard;
    let mut h2_hand = HandType::HighCard;

    if h1_max == 5 {
        h1_hand = HandType::FiveOfAKind;
    } else if h1_max == 4 {
        h1_hand = HandType::FourOfAKind;
    } else if h1_max  == 3 {

        //Three and Full House
        if h1_char.len() == 2{
            h1_hand = HandType::FullHouse;
        }else{
            h1_hand = HandType::ThreeOfAKind;
        }

    } else if h1_max  ==  2 {
        
        //One Pair Two Pair
        if h1_char.len() == 3 {
            h1_hand = HandType::TwoPair;
        }else{
            h1_hand = HandType::OnePair;
        }

    } else {
        h1_hand = HandType::HighCard;
    }

    if h2_max == 5 {
        h2_hand = HandType::FiveOfAKind;
    } else if h2_max == 4 {
        h2_hand = HandType::FourOfAKind;
    } else if h2_max  == 3 {

        //Three and Full House
        if h2_char.len() == 2{
            h2_hand = HandType::FullHouse;
        }else{
            h2_hand = HandType::ThreeOfAKind;
        }

    } else if h2_max  ==  2 {
        
        //One Pair Two Pair
        if h2_char.len() == 3 {
            h2_hand = HandType::TwoPair;
        }else{
            h2_hand = HandType::OnePair;
        }

    } else {
        h2_hand = HandType::HighCard;
    }

    if h1_hand > h2_hand {

        return true
    } else if h1_hand < h2_hand {

        return false
    } else {


        let char1_array: Vec<char> = hand_one.chars().collect();
        let char2_array: Vec<char> = hand_two.chars().collect();

        for i in 0..char1_array.len() {
            let c1 = char1_array[i];
            let c2 = char2_array[i];

            let card_type_one = char_to_card_type(c1).unwrap();
            let card_type_two = char_to_card_type(c2).unwrap();

            if card_type_one > card_type_two {
                return true
            }else if card_type_one < card_type_two {
                return false
            } else {
                continue;
            }

        }

        return false
    
    }

}


#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug)]
enum CharCardType {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 1,
    Queen = 12,
    King = 13,
    Ace = 14,
}