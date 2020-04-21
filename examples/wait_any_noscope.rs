#![deny(warnings)]
#![allow(clippy::float_cmp)]
extern crate mpi;

use mpi::traits::*;
use std::{thread, time};

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();

    let x: Vec<u8> = vec![1, 2, 3];

    if world.rank() == 0 {
        let mut requests = Vec::new();
        for i in 1..world.size() {
            requests.push(
                world
                    .process_at_rank(i)
                    .immediate_synchronous_send_with_tag_noscope(x.clone(), 0),
            );
        }

        println!("World size {}", world.size());
        while let Some(_status) = mpi::request::wait_any_noscope(&mut requests) {
            println!("Request  completed");
        }
        println!("All requests completed");
    } else {
        let secs = time::Duration::from_secs((1 * world.rank()) as u64);

        thread::sleep(secs);

        let (bin, _): (Vec<u8>, _) = world.any_process().receive_vec();
        println!("Process {} received data {:?}", world.rank(), bin);
    }
}
