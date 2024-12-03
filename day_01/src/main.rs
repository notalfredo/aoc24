use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    usize,
};

fn main() {
    let file = include_str!("../input/input.txt");
    //let file = include_str!("../input/example.txt");

    let mut left_map: BinaryHeap<usize> = BinaryHeap::new();
    let mut right_map: BinaryHeap<usize> = BinaryHeap::new();
    let mut page_map: HashMap<usize, usize> = HashMap::new();

    for row in file.split('\n').into_iter() {
        if row.is_empty() {
            continue;
        }

        let (lhs, rhs) = row.split_once("   ").unwrap();

        let lhs = lhs.parse::<usize>().unwrap();
        let rhs = rhs.parse::<usize>().unwrap();

        left_map.push(lhs);
        right_map.push(rhs);

        if page_map.contains_key(&rhs) {
            let key = page_map.get_mut(&rhs).unwrap();
            *key += 1;
        } else {
            page_map.insert(rhs, 1);
        }
    }

    let mut sum_q1: usize = 0;
    let mut sum_q2: usize = 0;

    while !left_map.is_empty() && !right_map.is_empty() {
        let lhs = left_map.pop().unwrap();
        let rhs = right_map.pop().unwrap();

        let distance: usize = (((lhs as isize) - (rhs as isize)).abs()) as usize;

        sum_q1 += distance;

        if let Some(freq) = page_map.get_mut(&lhs) {
            sum_q2 += lhs * (*freq);
        }
    }

    println!("q1: {:}", sum_q1);
    println!("q2: {:}", sum_q2);
}
