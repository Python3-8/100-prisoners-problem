use hundred_prisoners_problem::simulate;
use prawnypool::ThreadPool;
use std::sync::{Arc, Mutex};

const NTHREADS: usize = 15;
const NTIMES: usize = 10_000_000;
const NPRISONERS: usize = 100;

fn main() {
    let nwins = Arc::new(Mutex::new(0));
    println!("simulating with {NTHREADS} threads...");
    let mut pool = ThreadPool::new(NTHREADS);
    for _i in 0..NTIMES {
        let nwins = Arc::clone(&nwins);
        pool.queue(move || {
            if simulate(NPRISONERS).1 == 0 {
                *nwins.lock().unwrap() += 1;
            }
        });
    }
    pool.join();
    println!(
        "{} wins out of {NTIMES} iterations with {NPRISONERS} prisoners each time",
        *nwins.lock().unwrap()
    );
    println!(
        "winrate: {}%",
        (*nwins.lock().unwrap() as f64 * 100.) / NTIMES as f64
    );
}
