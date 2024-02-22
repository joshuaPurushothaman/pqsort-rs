use std::{
    cmp::Ordering::{Equal, Greater, Less},
    sync::mpsc::channel,
};
use threadpool::ThreadPool;

use rand::prelude::*;

fn main() {
    // let mut rng = rand::thread_rng();

    // let mut nums: Vec<i32> = (10..100).collect();
    // nums.shuffle(&mut rng);
    let nums = vec![
        50, 20, 89, 82, 61, 84, 48, 28, 29, 95, 10, 57, 70, 30, 62, 68, 22, 59, 99, 93, 14, 65, 92,
        15, 69, 73, 71, 27, 78, 45, 88, 39, 74, 12, 40, 64, 34, 36, 85, 81, 31, 96, 41, 56, 98, 43,
        60, 72, 90, 77, 63, 21, 76, 19, 80, 87, 13, 83, 42, 91, 18, 46, 94, 33, 17, 47, 53, 75, 66,
        44, 86, 55, 37, 49, 32, 38, 24, 16, 67, 26, 97, 79, 51, 11, 52, 35, 58, 23, 25, 54,
    ];

    // let sorted = pqsort(nums);
    let sorted = sqsort(nums);

    println!("{:?}", sorted);
}

// Parallel quick sort?!
// ...i'm not even sure this is qsort anymore lmao
// might be a hybrid merge-quicK???????
// TODO: Generics...?
fn pqsort(v: Vec<i32>) -> Vec<i32> {
    let pool = ThreadPool::new(16); //  TODO: config file

    let (left_sender, left_recv) = channel();
    let (right_sender, right_recv) = channel();

    let mut left = Vec::with_capacity(v.len() / 2);
    let mut right = Vec::with_capacity(v.len() / 2);

    let pivot = v[v.len() - 1];

    for &x in &v {
        match x.cmp(&pivot) {
            Less => left.push(x),
            Greater => right.push(x),
            Equal => (),
        }
    }
    println!("pivot: {pivot}\n");
    println!("left: {left:?}\n");
    println!("right: {right:?}\n");

    pool.execute(move || {
        let res = pqsort(left);
        left_sender.send(res).unwrap();
    });
    pool.execute(move || {
        let res = pqsort(right);
        right_sender.send(res).unwrap();
    });

    let left_sorted = left_recv.recv().unwrap();
    let right_sorted = right_recv.recv().unwrap();

    // join the left and right halves
    let mut res = Vec::with_capacity(v.len());

    res.extend(left_sorted);
    res.push(pivot);
    res.extend(right_sorted);

    res
}

fn sqsort(v: Vec<i32>) -> Vec<i32> {
    if v.len() <= 3 {
        println!("less than 3 items! sorting {v:?}");
        let mut res = v.clone();
        res.sort();
        return res;
    }

    let mut left = Vec::with_capacity(v.len() / 2);
    let mut right = Vec::with_capacity(v.len() / 2);

    let pivot = v[v.len() - 1];

    for &x in &v {
        match x.cmp(&pivot) {
            Less => left.push(x),
            Greater => right.push(x),
            Equal => (),
        }
    }
    println!("pivot: {pivot}\n");
    println!("\tleft: {left:?}\n");
    println!("\tright: {right:?}\n");

    let left_sorted = sqsort(left);
    let right_sorted = sqsort(right);

    // join the left and right halves
    let mut res = Vec::with_capacity(v.len());

    res.extend(left_sorted);
    res.push(pivot);
    res.extend(right_sorted);

    res
}
