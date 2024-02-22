use std::sync::mpsc::channel;
use threadpool::ThreadPool;

use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();

    let mut nums: Vec<i32> = (10..100).collect();
    nums.shuffle(&mut rng);

    let sorted = pqsort(nums);

    println!("{:?}", sorted);

    // let (sender, receiver) = channel();

    // // Spawn off an expensive computation
    // thread::spawn(move || {
    //     sender.send(expensive_computation()).unwrap();
    // });

    // // Do some useful work for awhile

    // // Let's see what that answer was
    // println!("{:?}", receiver.recv().unwrap());
}

// Parallel quick sort?!
// TODO: Generics...?
fn pqsort(v: Vec<i32>) -> Vec<i32> {
    let mut copy = v.clone();
    let pool = ThreadPool::new(16); //  TODO: config file

    pool.execute(move || {
        copy.sort();
    });

    // copy.sort(); //  temp lol

    copy
}
