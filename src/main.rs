#![feature(slice_concat_trait)]
mod canvas;
#[macro_use]
mod fuzzy_eq;
mod matrix;
pub const EPSILON: f64 = 0.00001;
type F = f64;

mod tuples;
fn main() {
    println!("Hello, world!");
}
