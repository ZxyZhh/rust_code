use crate::gardens::vegatables::Asparagus;
pub mod gardens;

fn main() {
    let plant = Asparagus {};
    println!("I am growing {plant:?}",plant);
}
