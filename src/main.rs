// rand::Rng; for generating the vector of integers
use rand::Rng;

// Hashmap is used for calculating the mode
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
    let mode = calculate_mode(vec);
    
    // prints the mode
    println!("The mode of the vector is: {:?}", mode);
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

//This now properly calculates the mode (checks for multiple possible solutions)
// Taking the vector generated in the main function iterate over it through a hashmap
// count each integer in the vector, whatever is > than the max_count gets pushed  onto the vector
// if something surpasses the count clear the vector and push the new integer onto it
// if something is = to the count just push to the vector
fn calculate_mode(vec: Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut mode = Vec::new();
    let mut max_count = 0;

        for i in vec {
            let count = map.entry(i).or_insert(0);
            *count += 1;

            if *count > max_count {
                max_count = *count;
                mode.clear();
                mode.push(i);
            } else if *count == max_count {
                mode.push(i);
            }
        }
        mode
    }

