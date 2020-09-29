use std::collections::HashMap;

fn main() {
    let max_chars = one("Today is Monday");    
    println!("the most of a char(first appearing) is '{}', appearing {} times", max_chars.0, max_chars.1);

    let test_two_input = "supracalafragalisticexpealaddcious";
    let test_two_output = two(test_two_input);
    println!("all subsequent duplicates removed of {} is {}", test_two_input, test_two_output);
}

fn two(input: &str) -> String {
    let mut r = String::new();
    let mut s = std::collections::HashSet::new();
    input.chars().for_each( |c | {
        if !s.contains(&c) {
            s.insert(c);
            r.push(c);
        }
    });
    r
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