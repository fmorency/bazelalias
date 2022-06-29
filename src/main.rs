use rand::{Rng, thread_rng};
use rand_07::{Rng as Rng2, thread_rng as thread_rng_07};

fn main() {
    let rng: u32 = thread_rng().gen();
    let rng_07: u32 = thread_rng_07().gen();

    println!("rng: {rng}");
    println!("rng_07: {rng_07}");
}
