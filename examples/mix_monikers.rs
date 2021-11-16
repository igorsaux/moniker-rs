#[doc(hidden)]
extern crate moniker_rs as moniker;

use moniker::{Animal, Moby};

// Mix `Animal` names with `Moby` adjectives
fn main() {
    let names = Animal::get_names().into_iter().take(25);
    let adjectives = Moby::get_adjectives().into_iter().take(25);

    let result: Vec<String> = names
        .zip(adjectives)
        .map(|(name, adjective)| format!("{} {}", adjective, name))
        .collect();

    println!("{:?}", result);
}
