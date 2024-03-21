#![deny(clippy::all)]

fn main() {
    let mut num = 5.4;
    num += 1.5;
    println!("{}... nice {}", num, num + 1.0);
}
