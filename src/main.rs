#![deny(missing_docs)]

//! Carcassonne implementation in Rust.

mod models;
use models::direction;

fn main() {
    let a: models::feature::Feature = models::feature::Feature::Monastery;

    match a {
        models::feature::Feature::Road { connections } => println!("yes"),
        _ => panic!("dad"),
    }
}
