use std::error::Error;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::BinaryHeap;

fn main() -> Result<(), Box<dyn Error>> {
    // Day 1 
    let f = File::open("./day1.txt").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut heap: BinaryHeap<u32> = BinaryHeap::new();
    let mut curr_sum:u32 = 0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        if line.eq("") {
            heap.push(curr_sum);
            curr_sum = 0;
        } else {
            let curr_val: u32 = line.parse()?;
            curr_sum += curr_val;
        }
    }
    //part 1
    println!("top value {:?}", heap.peek());

    //part 2
    let mut top_3_sum:u32 = 0;
    for _ in 0..3 {
        top_3_sum += heap.pop().unwrap();
    }
    println!("top 3 sum : {:?}", top_3_sum);
    
    Ok(())
}
