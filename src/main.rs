use std::collections::HashMap;

fn main() {
    let max_chars = one("Today is Monday");    
    println!("1) the most of a char(first appearing) is '{}', appearing {} times", max_chars.0, max_chars.1);

    let test_two_input = "supracalafragalisticexpealadocious";
    let test_two_output = two(test_two_input);
    println!("2) all subsequent duplicates removed of '{}' is '{}'", test_two_input, test_two_output);

    let test_three_input = "supracalafragalisticexpealadocious";
    let test_three_output = three(test_three_input);
    println!("3) duplicates of '{}' is '{}'", test_three_input, test_three_output);

    let test_four_input_1 = "spain";
    let test_four_input_2 = "the rain in spain falls mainly in the plain";
    let test_four_output = four(test_four_input_1, test_four_input_2);
    println!("4) characters of '{}' removed from '{}' yields '{}'", test_four_input_1, test_four_input_2, test_four_output);

    let test_five_input_1 = "XaXbXcXdXe";
    let test_five_input_2 = "XcXdXeXaXb";
    let test_five_output = five(test_five_input_1, test_five_input_2);
    let mut maybe = "";
    if !test_five_output {
        maybe = "not";
    }
    println!("4) '{}' is a {} rotation of '{}'", test_five_input_1, maybe,test_five_input_2);

}

fn five(input1: &str, input2: &str) -> bool {
    let doubleup = format!("{}{}", input1, input1);
    doubleup.contains(input2)
}


fn four(input1: &str, input2: &str) -> String {
    let mut r = String::new();
    let mut p = 'z';
    input2.chars().for_each( |c | {
        if !input1.contains(c) {
            if !(c == ' ' && p == ' ') {
                r.push(c);
                p = c;
            }
        }
    });
    r
}

fn three(input: &str) -> String {
    let mut r = String::new();
    let mut s = std::collections::HashSet::new();
    input.chars().for_each( |c | {
        if !s.contains(&c) {
            s.insert(c);
        } else {
            if !r.contains(c) {
                r.push(c);
            }
        }
    });
    r
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