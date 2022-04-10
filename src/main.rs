use soa_derive::StructOfArray;

#[derive(StructOfArray)]
pub struct Cheese {
    pub smell: f64,
    pub color: (f64, f64, f64),
    pub with_mushrooms: bool,
    pub name: String,
}

fn main() {
    println!("Hello, world!");
}
