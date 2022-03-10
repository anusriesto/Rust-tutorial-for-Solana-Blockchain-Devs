fn main() {
    let i=2;
    match i {
        0 => println!("o"),
        1 | 2 => println!("1 or 2"),
        3..=4 => println!("3,4"),
        _ => println!("default")
    }
   
}



