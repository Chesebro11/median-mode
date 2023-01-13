// suse std::collections::HashMap;

// Either create a vector with a bunch of integers or write a function that can generate integers then push to a vector

// Write a function that can iterate over this vector and calculate the median

// push this vector onto a hashmap, iterate over the hashmap with a function to calculate the mode
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();

    for _ in 0..100 {
        vec.push(rng.gen_range(1..501));
    }

    println!("{:?}", vec);

    let median = calculate_median(&vec);

    println!("the median of the vector is: {}", median);
}

fn calculate_median(vec: &Vec<i32>) -> f64 {
    let mut vec = vec.clone();
    vec.sort();
    let len = vec.len();
    if len % 2 == 0 {
        let mid = len /2;
        (vec[mid] as f64 + vec[mid - 1] as f64) / 2.0
    } else {
        vec[len / 2] as f64
    }
}

