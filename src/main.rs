use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const START: usize = 0;
const END: usize = 100;
fn main() {
    // Read file data

    let file = File::open("src/data/challenge_input.txt").unwrap();
    // let file = File::open("src/data/challange_5.txt").unwrap();
    let v: Vec<i32> = BufReader::new(file)
        .lines()
        .flatten() // gets rid of Err from lines
        .flat_map(|line| line.parse::<i32>()) // ignores Err variant from Result of str.parse
        .collect();

    let mut map_sum: HashMap<i32, i32> = HashMap::new();
    map_sum = start_iter_over_map(v.clone(), map_sum.clone());

    // loop somehow to all the values in the file and add to start and end depending on function to do
    let mut start = START;
    let mut end = END;
    'find: loop {
        let check_value = v[end];
        // println!("\nmap_sum: {:?}", map_sum);
        if check_next_val(map_sum.clone(), &check_value) == false {
            println!(
                "Value ({:?}) not secure. Position: {}",
                check_value,
                end + 1
            );
            break 'find;
        } else {
            map_sum = remove_key_value(v.clone(), map_sum.clone(), start, end);
            // println!("after remove: {:?}", map_sum);
            start += 1;
            map_sum = add_key_value(v.clone(), map_sum, start, end);
            // println!("after add: {:?}", map_sum);
            end += 1;
        }

        if end >= v.len() {
            break 'find;
        }
    }
}

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

fn check_next_val(map_sum: HashMap<i32, i32>, val: &i32) -> bool {
    // println!("Checking value... {}", val);
    map_sum.contains_key(val)
}

fn remove_key_value(
    content: Vec<i32>,
    mut map_sum: HashMap<i32, i32>,
    start: usize,
    end: usize,
) -> HashMap<i32, i32> {
    // println!("\nRemoving value: {}", content[start]);
    // println!(
    //     "From {} to {}  ({}, {})",
    //     content[start + 1],
    //     content[end - 1],
    //     start + 1,
    //     end - 1
    // );
    for y in (start + 1)..(end) {
        // println!(
        //     "Removing sum of {} + {} = {}",
        //     content[start],
        //     content[y],
        //     (content[start] + content[y])
        // );
        if let Some(value) = map_sum.get_mut(&(content[start] + content[y])) {
            *value -= 1;
            if *value == 0 {
                map_sum.remove(&(content[start] + content[y]));
            }
        }
    }
    map_sum
}

fn add_key_value(
    content: Vec<i32>,
    mut map_sum: HashMap<i32, i32>,
    new_start: usize,
    new_end: usize,
) -> HashMap<i32, i32> {
    // println!("\nAdding value: {}", content[new_end]);
    // println!(
    //     "From {} to {} ({}, {})",
    //     content[new_start],
    //     content[new_end - 1],
    //     new_start,
    //     new_end - 1
    // );
    for x in new_start..(new_end) {
        // println!(
        //     "Adding sum of {} + {} = {} ",
        //     content[new_end],
        //     content[x],
        //     (content[x] + content[new_end])
        // );
        if let Some(value) = map_sum.get_mut(&(content[x] + content[new_end])) {
            *value += 1;
        } else {
            map_sum.insert(content[x] + content[new_end], 1);
        }
    }
    map_sum
}
