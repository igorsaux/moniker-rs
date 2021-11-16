#[doc(hidden)]
extern crate moniker_rs as moniker;

use moniker::{Animal, Moniker};

// Print first 25 `Animal` monikers.
fn main() {
    let monikers_count = 25;
    let mut monikers = Vec::with_capacity(monikers_count);

    for _ in 0..monikers_count {
        monikers.push(Animal::get_random());
    }

    println!("{:?}", monikers);
}
