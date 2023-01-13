//After figuring out how to improve calculate mode to actually calculate the full mode
// this code could be refactored, I think two individual functions to calculate median and mode
// is excessive
use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    // generates 100 different integers between 1 and 500
    for _ in 0..100 {
        vec.push(rng.gen_range(1..501));
    }
    // prints out the vector
    println!("{:?}", vec);
    // passes the vector to the calculate_median function
    let median = calculate_median(&vec);
    // prints the median
    println!("the median of the vector is: {}", median);
    // passes the vector to calculate_mode function
    let mode = calculate_mode(&vec);
    // prints the mode
    println!("The mode of the vector is: {}", mode);

    // let (median, mode) = calculate_median_mode(&mut vec);
    // println!("The mode of the vector is: {}, and the Median of the vector is {}", mode, median)
}

// function that borrows the vector and calculates the median
fn calculate_median(vec: &Vec<i32>) -> f64 {
    let mut vec = vec.clone();
    vec.sort();
    let len = vec.len();
    // checks to see if the length is even or odd, if it is even take the average of the two middle elements
    // if odd return the middle element.
    if len % 2 == 0 {
        let mid = len /2;
        (vec[mid] as f64 + vec[mid - 1] as f64) / 2.0
    } else {
        vec[len / 2] as f64
    }
}

// this calculates the mode.. sort of. It doesn't keep track of all of the most occuring integers
// This should be written to be able to keep track if theres 1, 2, 3... etc. numbers occurring the most
fn calculate_mode(vec: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    for i in vec {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut vec: Vec<i32> = vec.clone();
        vec.sort();

    let mut max_value = 0;
    let mut mode = 0;
    for (i, count) in &map {
        if count > &max_value {
            max_value = *count;
            mode = **i;
        }
    }
    mode
}
// trying to figure out how to get this all into one function but struggling with mismatched types
// When I've solved the mismatched types issue i then run into an issue with returning the mode
// where I receive some sort of error about referencing / dereferncing a value
// fn calculate_median_mode(vec: &mut Vec<i32>) -> (f64, Vec<i32>) {
//     vec.sort();
//     let len = vec.len();

//     let mut map = HashMap::new();
//     let mut median = 0.0;
//     let mut mode = Vec::new();
//     let mut max_count = 0;

//     for i in vec {
//         let count = map.entry(i).or_insert(0);
//         *count += 1;

//         if *count > max_count {
//             max_count = *count;
//             mode.clear();
//             mode.push(i);
//         } else if *count == max_count {
//             mode.push(i);
//         }
//     }
//     if len % 2 == 0 {
//         let mid = len / 2;
//         median = (vec[mid] as f64 + vec[mid - 1] as f64) / 2.0;
//     } else {
//         median = vec[len / 2] as f64;
//     }
//     (median, mode)
// }
