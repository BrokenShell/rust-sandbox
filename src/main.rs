#[macro_use(c)]
extern crate cute;

fn main() {
    let mut vector = c![x, for x in 1..129, if x % 16 == 0];
    println!("{:?}", vector);
    vector.sort();
    vector.reverse();
    println!("Sum: {}", vector.iter().take(3).sum::<i16>());
    println!("{}", i8::MAX);
}
