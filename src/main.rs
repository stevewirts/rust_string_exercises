use std::collections::HashMap;

fn main() {
    let max_chars = one(&String::from("Today is Monday"));    
    println!("the most of a char(first appearing) is '{}', appearing {} times", max_chars.0, max_chars.1);
}

fn one(input: &str) -> (char, i32) {
    // return the char that appears most and it's count
    // the first occurence of 
    let mut counts = HashMap::new();
    let mut max_char = 'z'; //input.chars().next().unwrap();
    let mut max_value = 0;
    input.chars().rev().for_each( | c | {
        if counts.contains_key(&c) {
            let next_total = counts.get(&c).unwrap() + 1;
            if next_total >= max_value {
                max_char = c;
                max_value = next_total;
            }
            counts.insert( c, next_total);
        } else {
            counts.insert( c, 1);
        }
    });
    (max_char, max_value)
}