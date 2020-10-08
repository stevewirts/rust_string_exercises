use std::collections::HashMap;

fn main() {
    let test_one_input = "Today is Monday";
    let max_chars = one(test_one_input);
    println!("1) the most of a char(first appearing) in '{}' is '{}', appearing {} times", test_one_input, max_chars.0, max_chars.1);

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
    println!("5) '{}' is a {} rotation of '{}'", test_five_input_1, maybe,test_five_input_2);

    let test_six_input = "abracadabra";
    let test_six_output = six(test_six_input);
    println!("6) '{}' reversed is '{}'", test_six_input, test_six_output);

    let test_seven_input = "123";
    let test_seven_output = seven("", test_seven_input);
    println!("7) '{}' reversed is '{}'", test_seven_input, test_seven_output);

    let test_eight_input = "012";
    let test_eight_output = eight(test_eight_input);
    println!("8) '{}' has {} permutations {:?}", test_eight_input, test_eight_output.len(), test_eight_output);

    let test_nine_input = "uprasupradupra";
    let test_nine_output = nine(test_nine_input);
    println!("9) the first unrepeated char in '{}' is '{}'", test_nine_input, test_nine_output);

    let test_ten_input = "best is Rust";
    let test_ten_output = ten(test_ten_input);
    println!("10) reversed sentence '{}' is '{}'", test_ten_input, test_ten_output);

    let test_eleven_input1 = "this is a test string";
    let test_eleven_input2 = "tist";
    let test_eleven_output = eleven(test_eleven_input1, test_eleven_input2);
    println!("11) smallest substring '{}' inside of '{}' is '{}'", test_eleven_input2, test_eleven_input1, test_eleven_output);

}

fn eleven(i1: &str, i2: &str) -> String {
    let mut solutions: Vec<String> = vec!();

    // tuples of (char, original index of char in the input string)
    let mut pairs: Vec<(char, usize)> = i1.chars().enumerate().map( |e | -> (char, usize) {
        (e.1, e.0)
    }).filter(|p| i2.contains(p.0)).collect();
    // println!("{:?}", pairs);

    //iterate the input string from left to right
    for _i in 0..pairs.len() {

        // p will be the match that we remove characters from
        // if p becomes empty we know we've matched
        let mut p = String::from(i2.clone());

        // remember the first match from p as the head
        let mut head: Option<(char, usize)> = None;

        // remember the final match/char from p as the tail
        let mut tail: Option<(char, usize)> = None;

        // lets iterate over our pairs of (char, index)
        for e in &pairs {

            // if the pair is in p,
            // remove the character from p and
            // try and set the head and tail
            if p.contains(e.0) {
                p = p.replacen(e.0, "", 1);
                match head {
                    None => { head = Some(*e) },
                    Some(_) if p.is_empty() => {
                        tail = Some(*e);
                        break
                    },
                    Some(_) => {}
                }
            }
        }

        // if we found all the characters in i2
        // we have a match, so head and tail will be populated
        // chop the string out of i1 and submit it as a solution
        if head != None && tail != None {
            let h = head.unwrap();
            let t = tail.unwrap();
            let solution = String::from(&i1[h.1..=t.1]);
            solutions.push(solution);
        }

        // remove the front character, and iterate again
        pairs.remove(0);
    }
    // println!("{:?}", solutions);

    // find the shortest solution
    let shortest = solutions.iter().fold(solutions[0].clone(), |acc, item| {
        if item.len() < acc.len() {
            item.clone()
        } else {
            acc
        }
    });

    shortest
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

fn five(input1: &str, input2: &str) -> bool {
    let doubleup = format!("{}{}", input1, input1);
    doubleup.contains(input2)
}

fn six(input: &str) -> String {
    let mut r = String::new();
    input.chars().for_each( | c | {
        r = format!("{}{}", c, r);
    });
    r
}

fn seven(i1: &str, i2: &str) -> String {
    let mut r2 = String::from(i2);
    if i1.len() == 0 {
        return r2;
    }
    r2.push(i1.chars().last().unwrap());
    let size_minus_one = i1.len() - 1;
    let r1 = &i1[..size_minus_one];
    return seven(&r1, &r2);
}

fn eight(i: &str) -> Vec<String>  {
    let mut r = vec!();
    if i.len() == 1 {
        r.push(String::from(i));
        return r;
    }
    for idx in 0..i.len() {
        let front = &i[0..idx];
        let char = &i[idx..idx+1];
        let end = &i[idx+1..];
        let without = format!("{}{}", front, end);
        let subperms = eight(&without);
        for sp in subperms {
            r.push(format!("{}{}", char, sp));
        }
    }
    r
}

fn nine(i: &str) -> char {
    for e in i.chars() {
        let mut count = 0;
        for se in i.chars() {
            if se == e {
                count = count + 1;
            }
        }
        if count == 1 {
            return e;
        }
    }
    '\0'
}

fn ten(i: &str) -> String {
    let mut r = String::new();
    let mut is_first = true;
    for each in i.split(" ") {
        if is_first {
            is_first = false;
            r = String::from(each);
        } else {
            r = format!("{} {}", each, r);
        }
    }
    r
}