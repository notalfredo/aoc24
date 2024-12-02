use std::{cmp::Ordering, collections::BinaryHeap, usize};

fn main() {
    let file = include_str!("../input/input.txt");

    let mut left_map: BinaryHeap<usize> = BinaryHeap::new();
    let mut right_map: BinaryHeap<usize> = BinaryHeap::new();

    for row in file.split('\n').into_iter() {
        if row.is_empty() {
            continue;
        }

        let (lhs, rhs) = row.split_once("   ").unwrap();

        let lhs = lhs.parse::<usize>().unwrap();
        let rhs = rhs.parse::<usize>().unwrap();

        left_map.push(lhs);
        right_map.push(rhs);
    }

    let mut sum: usize = 0;

    while !left_map.is_empty() && !right_map.is_empty() {
        let lhs = left_map.pop().unwrap();
        let rhs = right_map.pop().unwrap();

        println!("{:} | {:}", lhs, rhs);

        let distance: usize = (((lhs as isize) - (rhs as isize)).abs()) as usize;

        sum += distance;
    }

    println!("q1: {:}", sum);
}
