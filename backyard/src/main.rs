use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}

// pub mod garden;

// fn main() {
//     let plant = garden::vegetables::Asparagus {};
//     println!("I'm growing {:?}!", plant);
// }
