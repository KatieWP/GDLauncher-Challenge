use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const START: usize = 0;
const END: usize = 100;
fn main() {
    // Read file data
    let file = File::open("src/data/challenge_input.txt").unwrap();
    //let file = File::open("src/data/challenge_5.txt").unwrap();
    let v: Vec<i32> = BufReader::new(file)
        .lines()
        .flatten() // gets rid of Err from lines
        .flat_map(|line| line.parse::<i32>()) // ignores Err variant from Result of str.parse
        .collect();

    // Hashmap where the key and value are stored:
    // key = is the sum of the values
    // value = is the amount of times the result of adding two numbers is repeated.
    let mut map_sum: HashMap<i32, i32> = HashMap::new();
    map_sum = start_iter_over_map(v.clone(), map_sum.clone());

    // loop through to all the values in the file (after the 100th position) and check whether its a valid value.
    // Then proceed to remove from the hashmap (or -1 value) all the keys that resulted form the sum of the first number to the rest in the list.
    // After we will add the values of the checked_value + the list of the previous 99 values. This will result in the sum of previous 100 values of the next number to check in the list.
    let mut start = START;
    let mut end = END;
    'find: loop {
        let check_value = v[end];
        // Break if the value is not secure
        if check_next_val(map_sum.clone(), &check_value) == false {
            println!(
                "Value ({:?}) not secure. Position: {}",
                check_value,
                end + 1
            );
            break 'find;
        } else {
            map_sum = remove_key_value(v.clone(), map_sum.clone(), start, end);
            start += 1;
            map_sum = add_key_value(v.clone(), map_sum, start, end);
            end += 1;
        }

        //Break if we go over the content length of the file.
        if end >= v.len() {
            break 'find;
        }
    }
}

// Get first iteration of the first 100 elements and add each value to the hashmap
fn start_iter_over_map(content: Vec<i32>, mut new_map: HashMap<i32, i32>) -> HashMap<i32, i32> {
    for x in START..END - 1 {
        for y in (x + 1)..END {
            if let Some(value) = new_map.get_mut(&(content[x] + content[y])) {
                *value += 1;
            } else {
                new_map.insert(content[x] + content[y], 1);
            }
        }
    }
    new_map
}

// checks if the value is a key in the hashmap.
fn check_next_val(map_sum: HashMap<i32, i32>, val: &i32) -> bool {
    map_sum.contains_key(val)
}

// remove all the sums that the first value in the list has with the rest of the values.
// Return the remaining hashmap.
fn remove_key_value(
    content: Vec<i32>,
    mut map_sum: HashMap<i32, i32>,
    start: usize,
    end: usize,
) -> HashMap<i32, i32> {
    for y in (start + 1)..(end) {
        if let Some(value) = map_sum.get_mut(&(content[start] + content[y])) {
            *value -= 1;
            if *value == 0 {
                map_sum.remove(&(content[start] + content[y]));
            }
        }
    }
    map_sum
}

// Addd the new values that result in the sum of the end value with the rest of the numbers in the list.
// return the changed hashmap
fn add_key_value(
    content: Vec<i32>,
    mut map_sum: HashMap<i32, i32>,
    new_start: usize,
    new_end: usize,
) -> HashMap<i32, i32> {
    for x in new_start..(new_end) {
        if let Some(value) = map_sum.get_mut(&(content[x] + content[new_end])) {
            *value += 1;
        } else {
            map_sum.insert(content[x] + content[new_end], 1);
        }
    }
    map_sum
}
