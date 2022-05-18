use std::collections::HashMap;
fn main() {
    println!("Hello, world!");

    //integers are simple values stored on the stack
    //in this case b is a copy of 1
    let x = 1;
    let b = x;
    println!("{}", b);
    println!("{}", x);

    let mut s1 = String::from("hello");
    let _s2 = s1.clone();
    println!("{}", s1);
    s1.push_str(" , world");
    println!("{}", s1);

    let s3 = "huzail";
    let s4 = s3;
    println!("{}", s4);

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it's okay to still
    let numbers = vec![1, 1, 2, 3, 3, 4, 4, 5, 6, 4]; // use x afterward
    let average = mean(&numbers);
    println!("The average is {}", average);

    let median = median(&numbers);
    println!("median {}", median);

    let mode = mode(&numbers);
    println!("{}", mode);

    //fibonacci sequence
    let mut nths = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for nth in nths {
        println!("{} {}", nth, fib(nth))
    }

   
        println!("{}", first_letter_to_uppper_case("hello world".to_string()));
     
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

use std::vec;

fn mean(numbers: &[i32]) -> f64 {
    let mut sum = 0.0 as f64;
    for num in numbers {
        sum += *num as f64;
    }
    sum / numbers.len() as f64
}

fn median(numbers: &Vec<i32>) -> f64 {
    let mut sortednumbers = numbers.to_vec();
    sortednumbers.sort();
    println!("{:?}", sortednumbers);
    let middle = sortednumbers.len() / 2;
    if sortednumbers.len() % 2 == 0 {
        return mean(&vec![sortednumbers[middle], sortednumbers[middle - 1]]);
    }
    sortednumbers[middle] as f64
}

fn mode(numbers: &[i32]) -> i32 {
    let mut map = HashMap::new();
    for num in numbers {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    println!("map of occurences{:?}", map);
    let mut maxv = 0;
    let mut mode = 0;
    for (key, value) in map {
        if value > maxv {
            maxv = value;
            mode = *key;
        }
    }
    mode
}

fn fib(nths: i32) -> i32 {
    let mut prev = 0;
    let mut curr = 1;

    for i in 1..nths {
        let mut next = prev + curr;
        prev = curr;
        curr = next;
    }
    curr
}
fn first_letter_to_uppper_case (s1: String) -> String {
    let mut c = s1.chars();
    match c.next() {
    
        Some(f) => f.to_uppercase().chain(c).collect(),
        // Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
      None => String::new(),
    }
  }

  